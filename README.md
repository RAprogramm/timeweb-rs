# timeweb-rs

Unofficial async Rust SDK for the [Timeweb Cloud](https://timeweb.cloud) API.

## Overview

`timeweb-rs` covers the full Timeweb Cloud public API — **313 operations across
22 areas**: cloud servers, managed databases, Kubernetes, projects, domains,
S3 storage, load balancers, firewalls, mail, AI agents, knowledge bases,
floating IPs, VPC, SSH keys, images, dedicated servers, container registry,
network drives and more.

The `apis` and `models` modules are generated from the official Timeweb Cloud
OpenAPI specification with [`openapi-generator`](https://openapi-generator.tech)
(the same tool behind the official Go, Python, PHP and Java SDKs). A thin
hand-written layer adds an idiomatic authenticated-client constructor. The
generated code is committed to the repository, so building the crate needs no
code generation step and no extra build dependencies.

## Installation

```sh
cargo add timeweb-rs
```

The crate uses `native-tls` by default. To use `rustls` instead:

```toml
timeweb-rs = { version = "0.1", default-features = false, features = ["rustls"] }
```

## Usage

Authentication uses a JWT token issued in the Timeweb Cloud control panel under
the "API и Terraform" section.

```rust
use timeweb_rs::apis::servers_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("TIMEWEB_CLOUD_TOKEN")?;
    let config = timeweb_rs::authenticated(token);

    let servers = servers_api::get_servers(&config, None, None).await?;
    println!("{servers:#?}");

    Ok(())
}
```

Every operation is a free async function in an `apis::*_api` module and takes a
reference to [`apis::configuration::Configuration`] as its first argument.
Build that configuration with `timeweb_rs::authenticated`.

## API coverage

One module per API area: `account_api`, `ai_agents_api`, `apps_api`,
`balancers_api`, `container_registry_api`, `databases_api`,
`dedicated_servers_api`, `domains_api`, `firewall_api`, `floating_ip_api`,
`images_api`, `knowledge_bases_api`, `kubernetes_api`, `locations_api`,
`mail_api`, `network_drives_api`, `payments_api`, `projects_api`, `s3_api`,
`servers_api`, `ssh_api`, `vpc_api`.

## Regenerating from the spec

The generated code is committed. To refresh it after an upstream API update:

1. Download the latest spec:
   ```sh
   curl -o openapi/timeweb-cloud.json https://timeweb.cloud/api-docs-data/bundle.json
   ```
2. Normalize it:
   ```sh
   python3 openapi/normalize_spec.py openapi/timeweb-cloud.json /tmp/normalized.json
   ```
3. Generate the client:
   ```sh
   npx @openapitools/openapi-generator-cli generate \
       -i /tmp/normalized.json -g rust -o /tmp/twgen \
       -p packageName=timeweb_client,library=reqwest
   ```
4. Replace `src/apis` and `src/models` with the freshly generated directories.

`openapi/normalize_spec.py` is a small, documented pre-processor: it reconciles
path parameters with their route templates (the upstream spec has a few
mismatches that produce non-compiling code) and swaps the Russian API tags for
the English names the spec already carries in `x-name-i18n`. Request and
response schemas are left untouched.

## License

[MIT](LICENSE)
