# Постраничная навигация

Списочные методы принимают `limit` и `offset` и отвечают объектом `meta`, в
котором есть общее количество элементов. `paginate` превращает это в поток
элементов:

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

Замыкание получает `(limit, offset)` и отдаёт элементы текущей страницы.
Следующая страница запрашивается только тогда, когда текущая закончилась, так
что большая коллекция никогда не попадает в память целиком.

## Лишний запрос

Постраничная навигация по смещению не может знать, что полная страница —
последняя. Поэтому, когда общее количество элементов кратно размеру страницы,
поток запрашивает ещё одну страницу и получает пустую коллекцию.

## Ошибки

Если страница не загрузилась, поток на этом заканчивается и отдаёт ошибку:

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

## Повторы

`paginate` ничего не повторяет: это тонкая обёртка над API. Если страницы
нужно повторять, заверните замыкание в повторяющий клиент:

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

Операции списка идемпотентны, поэтому повторять их безопасно.
