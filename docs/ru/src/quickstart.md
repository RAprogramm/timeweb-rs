# Быстрый старт

## Установка

```sh
cargo add timeweb-rs
```

По умолчанию крейт работает через `native-tls`. Если нужен `rustls`:

```toml
timeweb-rs = { version = "0.9", default-features = false, features = ["rustls", "full"] }
```

## Токен

API работает по JWT-токену, который выпускается в панели управления Timeweb
Cloud, в разделе «API и Terraform».

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

`authenticated` возвращает `Configuration` с таймаутом запроса в 30 секунд.
Эту конфигурацию нужно передать в операцию: каждая операция — свободная
асинхронная функция в модуле `apis::*_api`, и первый её аргумент — как раз
конфигурация.

## Вызов операции

Параметры пути и тело запроса — обычные аргументы:

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

## Ошибки

Каждая операция возвращает `Result<_, apis::Error<E>>`, где `E` — типизированное
тело ошибки этой операции. Общая структура ошибок API доступна через
`ErrorDetails`:

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

Подробнее — в разделе [Ошибки](./errors.md).

## Повторы

`TimewebClient` сам хранит конфигурацию и повторяет операцию по `RetryPolicy`,
если API ответил `429`, `5xx` или запрос не дошёл:

```rust
use timeweb_rs::{RetryPolicy, TimewebClient, apis::servers_api};

let client = TimewebClient::new(std::env::var("TIMEWEB_CLOUD_TOKEN")?);
let servers = client
    .execute(|| servers_api::get_servers(client.config(), None, None))
    .await?;
```

Повторять стоит только идемпотентные операции: повторный `POST`, создающий
ресурс, может создать его дважды. Подробнее — в разделе
[Повторы и таймауты](./retries.md).
