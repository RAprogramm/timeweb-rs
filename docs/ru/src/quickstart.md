# Быстрый старт

## Установка

```sh
cargo add timeweb-rs
```

По умолчанию крейт использует `native-tls`. Чтобы использовать `rustls`:

```toml
timeweb-rs = { version = "0.9", default-features = false, features = ["rustls", "full"] }
```

По умолчанию компилируются все области API. Чтобы оставить только нужные,
см. [Cargo-фичи](./features.md).

## Аутентификация

API аутентифицируется JWT-токеном, который выпускается в панели управления
Timeweb Cloud, в разделе «API и Terraform».

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
Передавайте её в любую операцию: каждая операция — свободная асинхронная
функция в модуле `apis::*_api`, первый аргумент которой — конфигурация.

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

## Обработка ошибок

Каждая операция возвращает `Result<_, apis::Error<T>>`, где `T` — типизированное
тело ошибки этой операции. Единый конверт ошибок API доступен через
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

Подробности — в разделе [Обработка ошибок](./errors.md).

## Повторы запросов

`TimewebClient` владеет конфигурацией и повторяет операцию по `RetryPolicy`,
если API отвечает `429`, `5xx` или запрос не дошёл:

```rust
use timeweb_rs::{RetryPolicy, TimewebClient, apis::servers_api};

let client = TimewebClient::new(std::env::var("TIMEWEB_CLOUD_TOKEN")?);
let servers = client
    .execute(|| servers_api::get_servers(client.config(), None, None))
    .await?;
```

Оборачивайте только **идемпотентные** операции: повторный `POST`, создающий
ресурс, может создать его дважды. Подробности — в разделе
[Повторы и таймауты](./retries.md).
