# Пагинация

Списочные эндпоинты принимают `limit` и `offset` и отвечают объектом `meta`,
в котором есть общее количество элементов. `paginate` превращает это в
`Stream` элементов:

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

Замыкание получает `(limit, offset)` и возвращает элементы этой страницы.
Поток запрашивает следующую страницу только тогда, когда текущая исчерпана,
поэтому большая коллекция никогда не попадает в память целиком.

## Последний запрос

Постраничная навигация по смещению не может знать, что полная страница —
последняя, поэтому поток запрашивает ещё одну страницу, когда общее
количество элементов кратно размеру страницы. Этот последний запрос
возвращает пустую коллекцию.

## Ошибки

Неудачная страница завершает поток и отдаёт ошибку:

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

## Повторяющий клиент

`paginate` не повторяет запросы: это тонкая обёртка над API. Оберните
замыкание в повторяющий клиент, если страницы нужно повторять:

```rust
use timeweb_rs::{TimewebClient, apis::servers_api};

let client = TimewebClient::new("your-jwt-token");
let mut servers = paginate(100, |limit, offset| {
    client.execute(|| servers_api::get_servers(client.config(), Some(limit), Some(offset)))
});
```

Операции списка идемпотентны, поэтому повторять их безопасно.
