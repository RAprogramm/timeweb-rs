# AI Gateway

AI Gateway — это отдельная поверхность API Timeweb Cloud: свой хост, свои ключи
и протокол, совместимый с OpenAI. Он живёт на `https://api.timeweb.ai`, а
остальной API — на `https://api.timeweb.cloud`.

## Две области, два хоста

| Область | Модуль | Хост | Пути |
| --- | --- | --- | --- |
| Управление шлюзом | `ai_gateway_api` | `api.timeweb.cloud` | `/api/v1/cloud-ai/…` |
| Инференс | `ai_gateway_inference_api` | `api.timeweb.ai` | `/v1/…` |

Управление ключами — их список, лимиты, расход — часть обычного API Timeweb
Cloud, и оно работает с обычной конфигурацией. Инференсные вызовы идут на хост
шлюза.

## Ключи

Ключ шлюза создаётся в панели управления или методом
`POST /api/v1/cloud-ai/ai-keys` и выглядит как `sk-…`. Это не тот же JWT-токен,
что и в панели управления: у двух поверхностей разные учётные данные.

```rust
use timeweb_rs::apis::ai_gateway_inference_api;

let config = timeweb_rs::authenticated_ai_gateway("sk-your-key");

let models = ai_gateway_inference_api::get_ai_gateway_models(&config).await?;
println!("{models:#?}");
```

`authenticated_ai_gateway` собирает конфигурацию для
[`AI_GATEWAY_BASE_URL`](https://docs.rs/timeweb-rs/latest/timeweb_rs/constant.AI_GATEWAY_BASE_URL.html)
и отправляет ключ как bearer-токен — ровно так же, как токен панели управления.

## Повторы

У шлюза свой клиент: базовый URL у него другой.

```rust
use timeweb_rs::TimewebClient;

let client = TimewebClient::ai_gateway("sk-your-key");
let models = client
    .execute(|| ai_gateway_inference_api::get_ai_gateway_models(client.config()))
    .await?;
```

## Стриминг и multipart

В области инференса есть потоковые чат-комплишены и multipart-загрузки
файлов протокола, совместимого с OpenAI. Операции принимают те же модели
запросов и ответов, что и остальной SDK, а файловые аргументы — обычные пути:
клиент читает файл и отправляет его частью запроса.
