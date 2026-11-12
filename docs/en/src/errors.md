# Error handling

Every operation returns `Result<T, apis::Error<E>>`, where `E` is the typed
error body of that operation. The four variants of `apis::Error` say which kind
of failure happened:

```rust,no_run
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let config = timeweb_rs::authenticated("your-jwt-token");
use timeweb_rs::apis::{servers_api, Error};

match servers_api::get_server(&config, 42).await {
    Ok(server) => println!("{server:#?}"),
    Err(Error::Reqwest(error)) => eprintln!("the request never reached the API: {error}"),
    Err(Error::Serde(error)) => eprintln!("the body was not the expected JSON: {error}"),
    Err(Error::Io(error)) => eprintln!("a local file could not be read: {error}"),
    Err(Error::ResponseError(response)) => {
        eprintln!("the API answered with an error: {}", response.status);
    }
}
# Ok(())
# }
```

- `Reqwest` — the request never reached the API: DNS, TLS, a dropped
  connection or a timeout.
- `Serde` — the API answered, but the body is not the expected JSON.
- `Io` — a local file of a multipart upload could not be read.
- `ResponseError` — the API answered with a `4xx` or `5xx`. It carries the
  status, the raw body, the response headers and the parsed error entity.

## The uniform envelope

Timeweb Cloud answers errors with the same envelope: a status code, a machine
readable error code, a message that is either a string or a list of strings,
and a correlation id. `ErrorDetails` extracts it from any operation error
without naming the per-operation entity:

```rust,no_run
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let config = timeweb_rs::authenticated("your-jwt-token");
use timeweb_rs::{apis::servers_api, ErrorDetails};

let error = servers_api::get_server(&config, 42).await.unwrap_err();

if let Some(details) = ErrorDetails::from_api_error(&error) {
    eprintln!("status: {}", details.status_code);
    eprintln!("code:   {}", details.error_code);
    eprintln!("text:   {}", details.messages().join("; "));
    if let Some(id) = &details.response_id {
        eprintln!("id:     {id}"); // quote it in a support ticket
    }
}
# Ok(())
# }
```

`messages()` returns the message of the envelope whatever shape it has — one
string or a list.

## The typed error

When the operation has a documented error body, it is parsed into the typed
entity and reachable through `ResponseContent`:

```rust,no_run
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let config = timeweb_rs::authenticated("your-jwt-token");
use timeweb_rs::apis::{servers_api, Error};

let error = servers_api::get_server(&config, 42).await.unwrap_err();

if let Error::ResponseError(response) = error {
    println!("status: {}", response.status);
    if let Some(entity) = &response.entity {
        println!("{entity:#?}");
    }
}
# Ok(())
# }
```

The entity is `Option`: it is `None` when the body does not match the schema
of that operation.

## Rate limits and retries

A `429` carries a `Retry-After` header. The retrying client reads it and waits
exactly as long as the API asked. See [Retries and timeouts](./retries.md).
