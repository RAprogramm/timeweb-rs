# timeweb-rs

Async Rust SDK for the [Timeweb Cloud](https://timeweb.cloud) API.

The client is generated from the vendor's own OpenAPI specification, so it
covers the whole public API surface — **514 operations across 31 areas**:
cloud servers, managed databases, Kubernetes, projects, domains, S3 storage,
load balancers, firewalls, mail, AI agents, AI Gateway, knowledge bases,
floating IPs, VPC, SSH keys, images, dedicated servers, container registry,
network drives and more.

Three things make it more than a generated client:

- **It is always in sync with the vendor.** A scheduled workflow compares the
  vendored specification with the upstream bundle every week, regenerates the
  client when the API changes and opens a pull request with the API diff. The
  SDK never drifts from the API it wraps.
- **It parses what the API really sends.** The specification and the live API
  disagree in a few places — nullable fields the API omits, timestamps without
  a UTC offset, unions the generator renders as empty structs. The generation
  pipeline repairs those cases, so the documented responses deserialize
  instead of failing.
- **It stays small.** Every API area is behind its own Cargo feature, so a
  consumer that needs one area does not compile the other thousand models.

```rust,no_run
# extern crate tokio;
# extern crate timeweb_rs;
use timeweb_rs::apis::servers_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = timeweb_rs::authenticated("your-jwt-token");
    let servers = servers_api::get_servers(&config, None, None).await?;
    println!("{servers:#?}");
    Ok(())
}
```

## Where to go next

- [Quickstart](./quickstart.md) — install the crate and call the API.
- [API reference](./api-index.md) — every operation, grouped by area.
- [Internals](./internals.md) — how the generation pipeline works.
- [Support](./support.md) — how to support the project.
