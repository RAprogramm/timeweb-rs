# timeweb-rs

[![crates.io](https://img.shields.io/crates/v/timeweb-rs.svg)](https://crates.io/crates/timeweb-rs)
[![docs.rs](https://img.shields.io/docsrs/timeweb-rs)](https://docs.rs/timeweb-rs)
[![CI](https://github.com/RAprogramm/timeweb-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/RAprogramm/timeweb-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/RAprogramm/timeweb-rs/graph/badge.svg?token=iKNwLuK8au)](https://codecov.io/gh/RAprogramm/timeweb-rs)
[![MSRV](https://img.shields.io/badge/MSRV-1.96-blue.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Async Rust SDK for the [Timeweb Cloud](https://timeweb.cloud/?i=137383) API.

## Overview

`timeweb-rs` covers the full Timeweb Cloud public API — **313 operations across
22 areas**: cloud servers, managed databases, Kubernetes, projects, domains,
S3 storage, load balancers, firewalls, mail, AI agents, knowledge bases,
floating IPs, VPC, SSH keys, images, dedicated servers, container registry,
network drives and more.

The `apis` and `models` modules are generated from the official Timeweb Cloud
OpenAPI specification with [`openapi-generator`](https://openapi-generator.tech)
(the same tool behind the official Go, Python, PHP and Java SDKs). A thin
hand-written layer adds an idiomatic authenticated-client constructor, retries
with exponential backoff, offset pagination as a `Stream` and a uniform view
of the API's error bodies. The generated code is committed to the repository,
so building the crate needs no code generation step and no extra build
dependencies.

## Installation

```sh
cargo add timeweb-rs
```

The crate uses `native-tls` by default. To use `rustls` instead:

```toml
timeweb-rs = { version = "0.1", default-features = false, features = ["rustls", "full"] }
```

By default every API area is compiled (the `full` feature). Each area also
has its own feature named after its module (`servers`, `databases`,
`kubernetes`, `dedicated-servers`, ...), so a consumer that needs one area
can skip compiling the other ~500 generated models:

```toml
timeweb-rs = { version = "0.1", default-features = false, features = ["native-tls", "servers"] }
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

For production use, `TimewebClient` wraps the same operations with retries
(429, 5xx and transport failures, capped exponential backoff), `paginate`
walks `limit`/`offset` collections as a `Stream`, and `ErrorDetails` extracts
the uniform error envelope from any operation error:

```rust,no_run
use timeweb_rs::{ErrorDetails, TimewebClient, apis::servers_api};

#[tokio::main]
async fn main() {
    let client = TimewebClient::new(std::env::var("TIMEWEB_CLOUD_TOKEN").expect("token"));
    match client
        .execute(|| servers_api::get_servers(client.config(), None, None))
        .await
    {
        Ok(servers) => println!("{servers:#?}"),
        Err(error) => match ErrorDetails::from_api_error(&error) {
            Some(details) => eprintln!("{}: {}", details.status_code, details.messages().join("; ")),
            None => eprintln!("{error}"),
        },
    }
}
```

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
5. Refresh the README statistics and module list:
   ```sh
   python3 openapi/update_readme.py
   ```
6. Regenerate the example-based deserialization tests:
   ```sh
   python3 openapi/generate_example_tests.py
   ```
7. Regenerate the per-area feature gates:
   ```sh
   python3 openapi/generate_feature_gates.py
   ```

`openapi/normalize_spec.py` is a small, documented pre-processor: it reconciles
path parameters with their route templates (the upstream spec has a few
mismatches that produce non-compiling code) and swaps the Russian API tags for
the English names the spec already carries in `x-name-i18n`. Request and
response schemas are left untouched.

## Releases

This project follows [Semantic Versioning](https://semver.org); changes are
recorded in [CHANGELOG.md](CHANGELOG.md). Releases are automated with
[release-plz](https://release-plz.dev): every push to `main` is analysed, and a
"release" pull request that bumps `version` in `Cargo.toml` and updates the
changelog is opened (or refreshed) from the [Conventional Commits](https://www.conventionalcommits.org)
in the diff. Merging that PR — or pushing a version bump directly to `main` —
makes the release job publish the crate to crates.io via crates.io trusted
publishing, push the `vX.Y.Z` tag and create the matching GitHub release from
the changelog entry. Use `feat:`/`fix:`/`docs:`/`refactor:` commit prefixes and
`!` for breaking changes so the version bump and changelog are generated
correctly.

## License

[MIT](LICENSE)
