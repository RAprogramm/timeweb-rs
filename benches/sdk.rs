//! Benchmarks for the CPU-bound parts of the SDK: decoding and encoding API
//! payloads, extracting the error envelope, offset pagination and the retry
//! backoff schedule. No network access is involved.

use std::{future::ready, time::Duration};

use divan::{Bencher, black_box};
use futures_util::StreamExt;
use reqwest::StatusCode;
use serde_json::Value;
use timeweb_rs::{
    ErrorDetails, RetryPolicy,
    apis::{Error, ResponseContent},
    models::{
        GetRouters200Response, GetServers200Response, Meta, Vds, VdsDisksInner, VdsNetworksInner,
        VdsNetworksInnerIpsInner, VdsOs, vds, vds_os
    },
    paginate
};

fn main() {
    divan::main();
}

/// Collection sizes used by the list payload benchmarks.
const SIZES: &[usize] = &[1, 10, 100];

/// A fully populated server, as returned by `GET /api/v1/servers`.
fn server(id: usize) -> Vds {
    let id = i64::try_from(id).expect("server id fits in i64");
    Vds {
        id,
        name: format!("server-{id}"),
        comment: "Production web node".to_owned(),
        created_at: "2026-03-11T13:43:54.000Z".to_owned(),
        os: Box::new(VdsOs {
            id:      79,
            name:    vds_os::Name::Ubuntu,
            version: Some("24.04".to_owned())
        }),
        preset_id: Some(2449),
        location: "ru-1".to_owned(),
        boot_mode: vds::BootMode::Std,
        status: vds::Status::On,
        start_at: Some(
            "2026-03-11T13:45:00+03:00"
                .parse()
                .expect("valid RFC 3339 timestamp")
        ),
        is_dedicated_cpu: false,
        cpu: 4.0,
        cpu_frequency: "3.3".to_owned(),
        ram: 8192.0,
        disks: vec![VdsDisksInner {
            id: id * 10,
            size: 81920.0,
            used: 12345.0,
            is_mounted: true,
            is_system: true,
            system_name: "vda".to_owned(),
            status: "on".to_owned(),
            ..VdsDisksInner::default()
        }],
        avatar_link: Some("https://urltoimage.com/avatars/image.png".to_owned()),
        vnc_pass: "vnc-password".to_owned(),
        networks: vec![VdsNetworksInner {
            bandwidth: Some(Some(1000.0)),
            ips: Some(vec![
                VdsNetworksInnerIpsInner {
                    ip: format!("203.0.113.{}", id % 250),
                    ptr: Some(format!("server-{id}.example.com")),
                    is_main: true,
                    ..VdsNetworksInnerIpsInner::default()
                },
                VdsNetworksInnerIpsInner {
                    ip: format!("2001:db8::{id:x}"),
                    ptr: None,
                    is_main: false,
                    ..VdsNetworksInnerIpsInner::default()
                },
            ]),
            is_ddos_guard: Some(false),
            is_image_mounted: Some(false),
            blocked_ports: Some(vec![25, 465, 587]),
            ..VdsNetworksInner::default()
        }],
        cloud_init: Some("#cloud-config\npackage_update: true\n".to_owned()),
        is_qemu_agent: true,
        ..Vds::default()
    }
}

fn servers_response(count: usize) -> GetServers200Response {
    GetServers200Response::new(
        Meta {
            total: i32::try_from(count).expect("count fits in i32")
        },
        (0..count).map(server).collect(),
        Some(uuid::Uuid::nil())
    )
}

/// The `GET /api/v1/routers` example from the vendored spec.
const ROUTER: &str = r#"{
  "id": "ca79642f-4b03-457b-bc43-714c126eda34",
  "account_id": "1",
  "avatar_link": null,
  "name": "router",
  "comment": "comment",
  "status": "started",
  "zone": "msk-1",
  "ips": [{ "ip": "203.0.113.10", "nat": null }],
  "preset_id": 2009,
  "preset": {
    "id": 2009,
    "node_count": 1,
    "cpu": 1,
    "cpu_frequency": "3.3",
    "ram": 1,
    "bandwidth": 1000,
    "cost": 450,
    "location": "ru-3"
  },
  "nodes": [{ "id": "f4f4a43d-be27-4dec-ae16-cd062540caa3" }],
  "networks": [
    {
      "id": "network-8ea2cef4765c422c9b2a22d6ec283bf3",
      "name": "network",
      "nat_ip": null,
      "gateway": "192.168.0.1",
      "dhcp": { "is_enabled": false, "is_available": true }
    }
  ],
  "created_at": "2026-03-11T13:43:54.000Z",
  "project_id": 1,
  "parent_services": []
}"#;

fn routers_body(count: usize) -> String {
    let router: Value = serde_json::from_str(ROUTER).expect("valid router fixture");
    serde_json::json!({
        "meta": { "total": count },
        "routers": vec![router; count],
        "response_id": "64c76353-f111-455b-a4e9-52734f877887"
    })
    .to_string()
}

mod servers {
    use super::{Bencher, GetServers200Response, SIZES, black_box, servers_response};

    #[divan::bench(args = SIZES)]
    fn deserialize(bencher: Bencher, count: usize) {
        let body = serde_json::to_string(&servers_response(count)).expect("serialize");
        bencher.bench(|| {
            serde_json::from_str::<GetServers200Response>(black_box(&body)).expect("deserialize")
        });
    }

    #[divan::bench(args = SIZES)]
    fn serialize(bencher: Bencher, count: usize) {
        let response = servers_response(count);
        bencher.bench(|| serde_json::to_string(black_box(&response)).expect("serialize"));
    }
}

mod routers {
    use super::{Bencher, GetRouters200Response, SIZES, black_box, routers_body};

    #[divan::bench(args = SIZES)]
    fn deserialize(bencher: Bencher, count: usize) {
        let body = routers_body(count);
        bencher.bench(|| {
            serde_json::from_str::<GetRouters200Response>(black_box(&body)).expect("deserialize")
        });
    }
}

mod error_details {
    use super::{Bencher, Error, ErrorDetails, ResponseContent, StatusCode, black_box};

    fn response_error(content: &str) -> Error<()> {
        Error::ResponseError(ResponseContent {
            status:  StatusCode::UNPROCESSABLE_ENTITY,
            content: content.to_owned(),
            entity:  None,
            headers: reqwest::header::HeaderMap::new()
        })
    }

    #[divan::bench]
    fn single_message(bencher: Bencher) {
        let error = response_error(
            r#"{"status_code":403,"error_code":"forbidden","message":"You do not have access for the attempted action","response_id":"94608d15-8672-4eed-8ab6-28bd6fa3cdf7"}"#
        );
        bencher.bench(|| {
            ErrorDetails::from_api_error(black_box(&error))
                .expect("envelope")
                .messages()
        });
    }

    #[divan::bench]
    fn many_messages(bencher: Bencher) {
        let error = response_error(
            r#"{"status_code":400,"error_code":"bad_request","message":["name must be a string","preset_id must be a number","os_id must be a number","bandwidth must not be less than 100"],"response_id":"94608d15-8672-4eed-8ab6-28bd6fa3cdf7"}"#
        );
        bencher.bench(|| {
            ErrorDetails::from_api_error(black_box(&error))
                .expect("envelope")
                .messages()
        });
    }
}

mod pagination {
    use super::{Bencher, Error, StreamExt, black_box, paginate, ready};

    /// Streams `total` in-memory items in pages of `page_size`.
    #[divan::bench(args = [10, 100, 1000])]
    fn stream_items(bencher: Bencher, page_size: i32) {
        const TOTAL: i32 = 10_000;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("tokio runtime");
        bencher.bench(|| {
            runtime.block_on(async {
                let stream = paginate(black_box(page_size), |limit, offset| {
                    let end = (offset + limit).min(TOTAL);
                    ready(Ok::<_, Error<()>>((offset..end).collect::<Vec<_>>()))
                });
                stream
                    .fold(0i64, |sum, item| {
                        ready(sum + i64::from(item.expect("in-memory page")))
                    })
                    .await
            })
        });
    }
}

mod retry {
    use super::{Duration, RetryPolicy, black_box};

    #[divan::bench]
    fn backoff_schedule() -> Duration {
        let policy = RetryPolicy::default();
        (0..64u32)
            .map(|index| policy.delay_for(black_box(index)))
            .sum()
    }
}
