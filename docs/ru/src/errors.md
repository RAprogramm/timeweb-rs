# Ошибки

Каждая операция возвращает `Result<T, apis::Error<E>>`, где `E` — типизированное
тело ошибки этой операции. Четыре варианта `apis::Error` говорят, что именно
случилось:

```rust,no_run
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let config = timeweb_rs::authenticated("your-jwt-token");
use timeweb_rs::apis::{servers_api, Error};

match servers_api::get_server(&config, 42).await {
    Ok(server) => println!("{server:#?}"),
    Err(Error::Reqwest(error)) => eprintln!("запрос не дошёл до API: {error}"),
    Err(Error::Serde(error)) => eprintln!("тело не похоже на ожидаемый JSON: {error}"),
    Err(Error::Io(error)) => eprintln!("не удалось прочитать локальный файл: {error}"),
    Err(Error::ResponseError(response)) => {
        eprintln!("API ответил ошибкой: {}", response.status);
    }
}
# Ok(())
# }
```

- `Reqwest` — запрос не дошёл до API: DNS, TLS, оборванное соединение или
  таймаут.
- `Serde` — API ответил, но тело оказалось не тем JSON, которого ждали.
- `Io` — не удалось прочитать локальный файл для multipart-загрузки.
- `ResponseError` — API ответил `4xx` или `5xx`. Здесь лежат статус, сырое
  тело, заголовки ответа и разобранная сущность ошибки.

## Общая структура ошибок

Timeweb Cloud отвечает ошибками по одному образцу: код статуса,
машиночитаемый код ошибки, сообщение (строка или список строк) и
идентификатор для обращения в поддержку. `ErrorDetails` достаёт это из любой
ошибки, не подстраиваясь под тип ошибки конкретной операции:

```rust,no_run
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let config = timeweb_rs::authenticated("your-jwt-token");
use timeweb_rs::{apis::servers_api, ErrorDetails};

let error = servers_api::get_server(&config, 42).await.unwrap_err();

if let Some(details) = ErrorDetails::from_api_error(&error) {
    eprintln!("status: {}", details.status_code);
    eprintln!("code:   {}", details.error_code);
    eprintln!("text:   {}", details.messages().join("; "));
    if let Some(id) = &details.response_id {
        eprintln!("id:     {id}"); // приложите его к обращению в поддержку
    }
}
# Ok(())
# }
```

`messages()` вернёт сообщение в любом виде — одна строка или список.

## Типизированная ошибка

Если у операции задокументировано тело ошибки, оно разбирается в типизированную
сущность и доступно через `ResponseContent`:

```rust,no_run
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let config = timeweb_rs::authenticated("your-jwt-token");
use timeweb_rs::apis::{servers_api, Error};

let error = servers_api::get_server(&config, 42).await.unwrap_err();

if let Error::ResponseError(response) = error {
    println!("status: {}", response.status);
    if let Some(entity) = &response.entity {
        println!("{entity:#?}");
    }
}
# Ok(())
# }
```

Сущность — это `Option`: она равна `None`, если тело не совпало со схемой этой
операции.

## Лимиты и повторы

В ответе `429` есть заголовок `Retry-After`. Повторяющий клиент его читает и
ждёт ровно столько, сколько попросил API. Подробнее — в разделе
[Повторы и таймауты](./retries.md).
