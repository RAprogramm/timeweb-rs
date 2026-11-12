# AI Gateway

The AI Gateway is a second API surface of Timeweb Cloud with its own host,
its own keys and an OpenAI-compatible protocol. It lives at
`https://api.timeweb.ai`, while the rest of the API lives at
`https://api.timeweb.cloud`.

## Two areas, two hosts

| Area | Module | Host | Paths |
| --- | --- | --- | --- |
| Gateway management | `ai_gateway_api` | `api.timeweb.cloud` | `/api/v1/cloud-ai/…` |
| Inference | `ai_gateway_inference_api` | `api.timeweb.ai` | `/v1/…` |

The management of the keys — listing them, their limits, their usage — is part
of the regular Timeweb Cloud API and uses the ordinary configuration. The
inference calls talk to the gateway host.

## The keys

A gateway key is created in the control panel or with
`POST /api/v1/cloud-ai/ai-keys`, and looks like `sk-…`. It is not the JWT
token of the control panel: the two surfaces have separate credentials.

```rust,no_run
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
use timeweb_rs::apis::ai_gateway_inference_api;

let config = timeweb_rs::authenticated_ai_gateway("sk-your-key");

let models = ai_gateway_inference_api::get_ai_gateway_models(&config).await?;
println!("{models:#?}");
# Ok(())
# }
```

`authenticated_ai_gateway` builds a configuration for
[`AI_GATEWAY_BASE_URL`](https://docs.rs/timeweb-rs/latest/timeweb_rs/constant.AI_GATEWAY_BASE_URL.html)
and sends the key as a bearer token, exactly like the control panel token.

## The retrying client

The gateway has its own client, because the base URL differs:

```rust,no_run
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
use timeweb_rs::{apis::ai_gateway_inference_api, TimewebClient};

let client = TimewebClient::ai_gateway("sk-your-key");
let models = client
    .execute(|| ai_gateway_inference_api::get_ai_gateway_models(client.config()))
    .await?;
println!("{models:#?}");
# Ok(())
# }
```

## Streaming and multipart

The inference area includes the streaming chat completions and the multipart
file uploads of the OpenAI-compatible protocol. The generated operations take
the same request and response models as the rest of the SDK, and the file
arguments are ordinary paths — the client reads the file and sends it as a
part.
