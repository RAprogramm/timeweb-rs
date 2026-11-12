//! Integration tests of the HTTP layer of the generated operations.
//!
//! The tests drive the client against a local HTTP server that answers with
//! canned responses, so the request the SDK builds and the response it maps
//! back are both covered: the `Authorization` header, the query parameters,
//! the deserialization of a documented body, the mapping of an error response
//! and the retries of the client.

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration
};

use timeweb_rs::{
    ErrorDetails, RetryPolicy, TimewebClient,
    apis::{Error, servers_api}
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// A response the mock server sends.
struct Canned {
    /// The status line and the headers of the response.
    head: String,
    /// The body of the response.
    body: String
}

impl Canned {
    /// A `200 OK` response with a JSON body.
    fn ok(body: &str) -> Self {
        Self {
            head: "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n".to_string(),
            body: body.to_string()
        }
    }

    /// An error response with a JSON body.
    fn error(status: &str, body: &str) -> Self {
        Self {
            head: format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\n"),
            body: body.to_string()
        }
    }

    /// A response that carries a `Retry-After` header.
    fn with_retry_after(mut self, seconds: u32) -> Self {
        use std::fmt::Write;

        let _ = write!(self.head, "Retry-After: {seconds}\r\n");
        self
    }
}

/// A local HTTP server that answers the requests with the canned responses.
struct MockServer {
    address:  SocketAddr,
    requests: Arc<Mutex<Vec<String>>>
}

impl MockServer {
    /// Starts a server that answers the requests with the given responses.
    async fn start(responses: Vec<Canned>) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("the mock server binds");
        let address = listener
            .local_addr()
            .expect("the mock server has an address");
        let requests = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&requests);
        tokio::spawn(async move {
            let mut responses = responses.into_iter();
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };
                let Some(response) = responses.next() else {
                    break;
                };
                let request = read_request(&mut stream).await;
                recorded
                    .lock()
                    .expect("the requests are recorded")
                    .push(request);
                let body = response.body;
                let head = format!(
                    "{}Content-Length: {}\r\nConnection: close\r\n\r\n",
                    response.head,
                    body.len()
                );
                let _ = stream.write_all(head.as_bytes()).await;
                let _ = stream.write_all(body.as_bytes()).await;
                let _ = stream.shutdown().await;
            }
        });
        Self {
            address,
            requests
        }
    }

    /// The base URL of the server.
    fn base_url(&self) -> String {
        format!("http://{}", self.address)
    }

    /// The requests the server received.
    fn requests(&self) -> Vec<String> {
        self.requests
            .lock()
            .expect("the requests are recorded")
            .clone()
    }
}

/// Reads a whole HTTP request from a connection.
async fn read_request(stream: &mut tokio::net::TcpStream) -> String {
    let mut request = Vec::new();
    let mut buffer = [0_u8; 1024];
    loop {
        let read = match stream.read(&mut buffer).await {
            Ok(0) | Err(_) => break,
            Ok(read) => read
        };
        request.extend_from_slice(&buffer[..read]);
        let text = String::from_utf8_lossy(&request);
        if let Some(index) = text.find("\r\n\r\n") {
            let length = text
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.trim()
                        .eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().ok())?
                })
                .unwrap_or_default();
            if request.len() >= index + 4 + length {
                break;
            }
        }
    }
    String::from_utf8_lossy(&request).to_string()
}

#[tokio::test]
async fn the_client_sends_the_bearer_token_and_the_query_parameters() {
    let server = MockServer::start(vec![Canned::ok(r#"{"servers":[],"meta":{"total":0}}"#)]).await;
    let config = timeweb_rs::authenticated_with_base_url("jwt-token", server.base_url());

    let response = servers_api::get_servers(&config, Some(10), Some(20))
        .await
        .expect("the request succeeds");

    assert_eq!(response.servers, Vec::new());
    let request = server
        .requests()
        .first()
        .expect("the request was recorded")
        .clone();
    assert!(request.contains("GET /api/v1/servers?limit=10&offset=20 "));
    assert!(
        request
            .to_lowercase()
            .contains("authorization: bearer jwt-token")
    );
    assert!(request.to_lowercase().contains("user-agent: timeweb-rs/"));
}

#[tokio::test]
async fn a_documented_body_is_deserialized_into_the_model() {
    let body = r#"{
        "response_id": "15095f25-aac3-4d60-a788-96cb5136f186",
        "meta": { "total": 1 },
        "servers": [
            {
                "id": 1,
                "name": "cloud-server",
                "comment": "",
                "created_at": "2026-07-04T05:43:00",
                "os": { "id": 1, "name": "ubuntu", "version": "24.04" },
                "location": "ru-1",
                "boot_mode": "std",
                "status": "on",
                "is_ddos_guard": false,
                "is_master_ssh": false,
                "is_dedicated_cpu": false,
                "gpu": 0,
                "cpu": 2,
                "cpu_frequency": "3.3GHz",
                "ram": 4096,
                "disks": [],
                "vnc_pass": "secret",
                "networks": [],
                "is_qemu_agent": true,
                "availability_zone": "ru-1"
            }
        ]
    }"#;
    let server = MockServer::start(vec![Canned::ok(body)]).await;
    let config = timeweb_rs::authenticated_with_base_url("jwt-token", server.base_url());

    let response = servers_api::get_servers(&config, None, None)
        .await
        .expect("the request succeeds");

    assert_eq!(response.servers.len(), 1);
    assert_eq!(response.servers[0].name, "cloud-server");
    assert_eq!(response.meta.total, 1);
}

#[tokio::test]
async fn an_error_response_becomes_a_response_error() {
    let body = r#"{
        "status_code": 404,
        "error_code": "not_found",
        "message": "Сервер не найден",
        "response_id": "15095f25-aac3-4d60-a788-96cb5136f186"
    }"#;
    let server = MockServer::start(vec![Canned::error("404 Not Found", body)]).await;
    let config = timeweb_rs::authenticated_with_base_url("jwt-token", server.base_url());

    let error = servers_api::get_server(&config, 42)
        .await
        .expect_err("the request fails");

    let Error::ResponseError(response) = error else {
        panic!("expected a response error, got {error:?}");
    };
    assert_eq!(response.status.as_u16(), 404);
    assert!(response.content.contains("not_found"));

    let details = ErrorDetails::from_api_error(&Error::ResponseError(response))
        .expect("the error envelope is parsed");
    assert_eq!(details.status_code, 404);
    assert_eq!(details.error_code, "not_found");
    assert_eq!(details.messages(), vec!["Сервер не найден".to_string()]);
}

#[tokio::test]
async fn the_client_retries_a_server_error() {
    let body = r#"{"servers":[],"meta":{"total":0}}"#;
    let server = MockServer::start(vec![
        Canned::error("503 Service Unavailable", "{}"),
        Canned::ok(body),
    ])
    .await;
    let client =
        TimewebClient::with_base_url("jwt-token", server.base_url()).with_policy(RetryPolicy {
            max_retries: 2,
            base_delay:  Duration::ZERO,
            max_delay:   Duration::ZERO
        });

    let response = client
        .execute(|| servers_api::get_servers(client.config(), None, None))
        .await
        .expect("the retry succeeds");

    assert_eq!(response.servers, Vec::new());
    assert_eq!(server.requests().len(), 2);
}

#[tokio::test]
async fn the_client_honors_the_retry_after_header() {
    let body = r#"{"servers":[],"meta":{"total":0}}"#;
    let server = MockServer::start(vec![
        Canned::error("429 Too Many Requests", "{}").with_retry_after(0),
        Canned::ok(body),
    ])
    .await;
    let client =
        TimewebClient::with_base_url("jwt-token", server.base_url()).with_policy(RetryPolicy {
            max_retries: 1,
            base_delay:  Duration::from_secs(30),
            max_delay:   Duration::from_secs(30)
        });

    let started = std::time::Instant::now();
    client
        .execute(|| servers_api::get_servers(client.config(), None, None))
        .await
        .expect("the retry succeeds");

    assert!(started.elapsed() < Duration::from_secs(5));
}

#[tokio::test]
async fn a_stalled_server_does_not_block_the_client_forever() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("the mock server binds");
    let address = listener
        .local_addr()
        .expect("the mock server has an address");
    tokio::spawn(async move {
        while let Ok((mut stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut buffer = [0_u8; 1024];
                while let Ok(read) = stream.read(&mut buffer).await {
                    if read == 0 {
                        break;
                    }
                }
            });
        }
    });
    let mut config =
        timeweb_rs::authenticated_with_base_url("jwt-token", format!("http://{address}"));
    config.client = reqwest::Client::builder()
        .timeout(Duration::from_millis(200))
        .build()
        .expect("the client builds");

    let error = servers_api::get_servers(&config, None, None)
        .await
        .expect_err("the request times out");

    assert!(matches!(error, Error::Reqwest(_)));
}
