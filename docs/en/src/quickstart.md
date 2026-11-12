# Quickstart

## Install

```sh
cargo add timeweb-rs
```

The crate uses `native-tls` by default. To use `rustls` instead:

```toml
timeweb-rs = { version = "0.9", default-features = false, features = ["rustls", "full"] }
```

Every API area is compiled by default. See [Feature flags](./features.md) to
compile only the areas you need.

## Authenticate

The API authenticates with a JWT token issued in the Timeweb Cloud control
panel, under the "API и Terraform" section.

```rust
use timeweb_rs::apis::servers_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = timeweb_rs::authenticated(std::env::var("TIMEWEB_CLOUD_TOKEN")?);

    let servers = servers_api::get_servers(&config, None, None).await?;
    for server in &servers.servers {
        println!("{} ({})", server.name, server.status);
    }

    Ok(())
}
```

`authenticated` returns a `Configuration` with a 30 second request timeout.
Pass it to any operation — every operation is a free async function in an
`apis::*_api` module and takes the configuration as its first argument.

## Call an operation

Path parameters and the request body are ordinary arguments:

```rust
use timeweb_rs::apis::{databases_api, models::CreateDatabaseRequest};

let database = databases_api::create_database(
    &config,
    &CreateDatabaseRequest {
        name: "orders".to_owned(),
        ..CreateDatabaseRequest::default()
    },
)
.await?;
```

## Handle the errors

Every operation returns `Result<_, apis::Error<T>>`, where `T` is the typed
error body of that operation. The uniform envelope of the API is available
through `ErrorDetails`:

```rust
use timeweb_rs::ErrorDetails;

match servers_api::get_server(&config, 42).await {
    Ok(server) => println!("{server:#?}"),
    Err(error) => match ErrorDetails::from_api_error(&error) {
        Some(details) => eprintln!("{}: {}", details.status_code, details.messages().join("; ")),
        None => eprintln!("{error}"),
    },
}
```

See [Error handling](./errors.md) for the details.

## Use the retries

`TimewebClient` owns the configuration and re-runs an operation under a
`RetryPolicy` when the API answers `429`, `5xx` or the transport fails:

```rust
use timeweb_rs::{RetryPolicy, TimewebClient, apis::servers_api};

let client = TimewebClient::new(std::env::var("TIMEWEB_CLOUD_TOKEN")?);
let servers = client
    .execute(|| servers_api::get_servers(client.config(), None, None))
    .await?;
```

Only wrap **idempotent** operations: a retried `POST` that creates a resource
may create it twice. See [Retries and timeouts](./retries.md).
