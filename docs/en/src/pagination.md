# Pagination

The list endpoints take a `limit` and an `offset` and answer with a `meta`
object that carries the total number of items. `paginate` turns that into a
`Stream` of items:

```rust
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

```rust
let mut servers = paginate(100, |limit, offset| {
    let config = &config;
    async move {
        let page = servers_api::get_servers(config, Some(limit), Some(offset)).await?;
        Ok(page.servers)
    }
});

while let Some(server) = servers.try_next().await {
    match server {
        Ok(server) => println!("{}", server.name),
        Err(error) => {
            eprintln!("{error}");
            break;
        }
    }
}
```

## The retrying client

`paginate` does not retry: it is a thin stream over the API. Wrap the fetch
closure in the retrying client when the pages should be retried:

```rust
use timeweb_rs::{RetryPolicy, TimewebClient, apis::servers_api};

let client = TimewebClient::new("your-jwt-token");
let mut servers = paginate(100, |limit, offset| {
    client.execute(|| servers_api::get_servers(client.config(), Some(limit), Some(offset)))
});
```

The list operations are idempotent, so retrying them is safe.
