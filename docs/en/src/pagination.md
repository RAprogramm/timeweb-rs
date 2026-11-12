# Pagination

The list endpoints take a `limit` and an `offset` and answer with a `meta`
object that carries the total number of items. `paginate` turns that into a
`Stream` of items:

```rust,no_run
# extern crate futures_util;
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
use futures_util::TryStreamExt;
use timeweb_rs::{apis::servers_api, paginate};

let config = timeweb_rs::authenticated("your-jwt-token");

let mut servers = paginate(100, |limit, offset| {
    let config = &config;
    async move {
        let page = servers_api::get_servers(config, Some(limit), Some(offset)).await?;
        Ok(page.servers)
    }
});

while let Some(server) = servers.try_next().await? {
    println!("{}", server.name);
}
# Ok(())
# }
```

The closure receives `(limit, offset)` and returns the items of that page. The
stream asks for the next page only when the current one is exhausted, so a
large collection never lands in memory at once.

## The last request

Offset pagination cannot know that a full page is the last one, so the stream
asks for one more page when the total number of items is a multiple of the
page size. That last request answers with an empty collection.

## Errors

A failed page ends the stream and yields the error:

```rust,no_run
# extern crate futures_util;
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# use timeweb_rs::{apis::servers_api, paginate};
# let config = timeweb_rs::authenticated("your-jwt-token");
use futures_util::TryStreamExt;

let mut servers = paginate(100, |limit, offset| {
    let config = &config;
    async move {
        let page = servers_api::get_servers(config, Some(limit), Some(offset)).await?;
        Ok(page.servers)
    }
});

loop {
    match servers.try_next().await {
        Ok(Some(server)) => println!("{}", server.name),
        Ok(None) => break,
        Err(error) => {
            eprintln!("{error}");
            break;
        }
    }
}
# Ok(())
# }
```

## The retrying client

`paginate` does not retry: it is a thin stream over the API. Wrap the fetch
closure in the retrying client when the pages should be retried:

```rust,no_run
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
use timeweb_rs::{apis::servers_api, paginate, TimewebClient};

let client = TimewebClient::new("your-jwt-token");
let retrying = &client;
let mut servers = paginate(100, |limit, offset| {
    async move {
        let page = retrying
            .execute(|| servers_api::get_servers(retrying.config(), Some(limit), Some(offset)))
            .await?;
        Ok(page.servers)
    }
});
# let _ = servers;
# Ok(())
# }
```

The list operations are idempotent, so retrying them is safe.
