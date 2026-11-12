# Повторы и таймауты

## Таймаут

Каждая конфигурация, собранная через `authenticated`, использует HTTP-клиент с
таймаутом в 30 секунд, поэтому зависшее соединение не держит задачу вечно.
Таймаут вынесен в константу:

```rust
# extern crate timeweb_rs;
use std::time::Duration;
use timeweb_rs::DEFAULT_TIMEOUT;

assert_eq!(DEFAULT_TIMEOUT, Duration::from_secs(30));
```

Запрос, который не уложился в таймаут, падает с `Error::Reqwest`.

Если стандартный клиент не подходит — нужен прокси, свой пул соединений или
другой таймаут, — принесите свой:

```rust,no_run
# extern crate reqwest;
# extern crate timeweb_rs;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(5))
    .build()?;
let config = timeweb_rs::authenticated_with_client("your-jwt-token", client);
# let _ = config;
# Ok(())
# }
```

## Политика повторов

`TimewebClient` повторяет операцию, когда сбой имеет смысл повторять:

- запрос не дошёл до API;
- `429 Too Many Requests`;
- любой ответ `5xx`.

Остальные ошибки возвращаются сразу. Задержка растёт экспоненциально от
базовой, удваивается на каждой попытке и ограничена максимальной задержкой;
половина её случайна, чтобы клиенты разных машин не повторяли запросы
одновременно.

```rust
# extern crate timeweb_rs;
# fn main() {
use std::time::Duration;
use timeweb_rs::{RetryPolicy, TimewebClient};

let policy = RetryPolicy {
    max_retries: 5,
    base_delay:  Duration::from_millis(200),
    max_delay:   Duration::from_secs(10),
};

let client = TimewebClient::new("your-jwt-token").with_policy(policy);
assert_eq!(client.config().base_path, timeweb_rs::DEFAULT_BASE_URL);
# }
```

По умолчанию клиент повторяет три раза, начиная с 500 миллисекунд, и
ограничивает задержку восемью секундами.

## Retry-After

В ответе `429` может быть заголовок `Retry-After` — числом секунд или датой в
формате HTTP. Клиент его учитывает: задержка становится ровно такой, какой
попросил API, но не больше максимальной задержки политики.

## Идемпотентность

**Повторяйте только идемпотентные операции.** `POST`, создающий ресурс, может
дойти до API даже тогда, когда ответ не пришёл, — повтор такого запроса может
создать ресурс дважды.

`execute` повторяет всё, что ему дают: политика — свойство клиента, а не
операции. Для неидемпотентной операции есть `execute_once`:

```rust,no_run
# extern crate tokio;
# extern crate timeweb_rs;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
use timeweb_rs::{
    apis::ssh_api,
    models::CreateKeyRequest,
    TimewebClient,
};

let client = TimewebClient::new("your-jwt-token");
let key = client
    .execute_once(|| {
        ssh_api::create_key(
            client.config(),
            CreateKeyRequest {
                name: "deploy".to_owned(),
                body: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIG…".to_owned(),
                ..CreateKeyRequest::default()
            },
        )
    })
    .await?;
println!("создан ключ {}", key.ssh_key.id);
# Ok(())
# }
```

## Политика публична

`RetryPolicy::delay_for` отдаёт номинальную задержку повтора — ту, до которой
ещё не применён случайный разброс. Она детерминирована, поэтому её удобно
проверять в тестах.
