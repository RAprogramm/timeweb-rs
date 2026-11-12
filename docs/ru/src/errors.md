# Обработка ошибок

Каждая операция возвращает `Result<T, apis::Error<E>>`, где `E` — типизированное
тело ошибки этой операции:

```rust
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}
```

- `Reqwest` — запрос не дошёл до API: DNS, TLS, оборванное соединение или
  таймаут.
- `Serde` — API ответил, но тело оказалось не тем JSON, который ожидался.
- `Io` — не удалось прочитать локальный файл multipart-загрузки.
- `ResponseError` — API ответил `4xx` или `5xx`. Он несёт статус, сырое тело,
  заголовки ответа и разобранную сущность ошибки.

## Единый конверт

Timeweb Cloud отвечает ошибками одним и тем же конвертом: код статуса,
машиночитаемый код ошибки, сообщение (строка или список строк) и идентификатор
корреляции. `ErrorDetails` извлекает его из любой ошибки операции, не называя
типизированную сущность конкретной операции:

```rust
use timeweb_rs::ErrorDetails;

let error = servers_api::get_server(&config, 42).await.unwrap_err();

if let Some(details) = ErrorDetails::from_api_error(&error) {
    eprintln!("status: {}", details.status_code);
    eprintln!("code:   {}", details.error_code);
    eprintln!("text:   {}", details.messages().join("; "));
    if let Some(id) = &details.response_id {
        eprintln!("id:     {id}"); // приложите его к обращению в поддержку
    }
}
```

`messages()` вернёт сообщение конверта в любой форме — одну строку или список.

## Типизированная ошибка

Если у операции задокументировано тело ошибки, оно разбирается в типизированную
сущность и доступно через `ResponseContent`:

```rust
use timeweb_rs::apis::Error;

let error = servers_api::get_server(&config, 42).await.unwrap_err();

if let Error::ResponseError(response) = error {
    println!("status: {}", response.status);
    if let Some(entity) = &response.entity {
        println!("{entity:#?}");
    }
}
```

Сущность — это `Option`: она равна `None`, если тело не соответствует схеме
этой операции.

## Лимиты и повторы

Ответ `429` несёт заголовок `Retry-After`. Повторяющий клиент читает его и
ждёт ровно столько, сколько попросил API. Подробности — в разделе
[Повторы и таймауты](./retries.md).
