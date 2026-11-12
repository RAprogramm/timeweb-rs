/*
 * Документация публичного API
 *
 * # Введение API Timeweb Cloud позволяет вам управлять ресурсами в облаке программным способом с использованием обычных HTTP-запросов.  Множество функций, которые доступны в панели управления Timeweb Cloud, также доступны через API, что позволяет вам автоматизировать ваши собственные сценарии.  В этой документации сперва будет описан общий дизайн и принципы работы API, а после этого конкретные конечные точки. Также будут приведены примеры запросов к ним.   ## Запросы Запросы должны выполняться по протоколу `HTTPS`, чтобы гарантировать шифрование транзакций. Поддерживаются следующие методы запроса: |Метод|Применение| |--- |--- | |GET|Извлекает данные о коллекциях и отдельных ресурсах.| |POST|Для коллекций создает новый ресурс этого типа. Также используется для выполнения действий с конкретным ресурсом.| |PUT|Обновляет существующий ресурс.| |PATCH|Некоторые ресурсы поддерживают частичное обновление, то есть обновление только части атрибутов ресурса, в этом случае вместо метода PUT будет использован PATCH.| |DELETE|Удаляет ресурс.|  Методы `POST`, `PUT` и `PATCH` могут включать объект в тело запроса с типом содержимого `application/json`.  ### Параметры в запросах Некоторые коллекции поддерживают пагинацию, поиск или сортировку в запросах. В параметрах запроса требуется передать: - `limit` — обозначает количество записей, которое необходимо вернуть  - `offset` — указывает на смещение, относительно начала списка  - `search` — позволяет указать набор символов для поиска  - `sort` — можно задать правило сортировки коллекции  ## Ответы Запросы вернут один из следующих кодов состояния ответа HTTP:  |Статус|Описание| |--- |--- | |200 OK|Действие с ресурсом было выполнено успешно.| |201 Created|Ресурс был успешно создан. При этом ресурс может быть как уже готовым к использованию, так и находиться в процессе запуска.| |204 No Content|Действие с ресурсом было выполнено успешно, и ответ не содержит дополнительной информации в теле.| |400 Bad Request|Был отправлен неверный запрос, например, в нем отсутствуют обязательные параметры и т. д. Тело ответа будет содержать дополнительную информацию об ошибке.| |401 Unauthorized|Ошибка аутентификации.| |403 Forbidden|Аутентификация прошла успешно, но недостаточно прав для выполнения действия.| |404 Not Found|Запрашиваемый ресурс не найден.| |409 Conflict|Запрос конфликтует с текущим состоянием.| |423 Locked|Ресурс из запроса заблокирован от применения к нему указанного метода.| |429 Too Many Requests|Был достигнут лимит по количеству запросов в единицу времени.| |500 Internal Server Error|При выполнении запроса произошла какая-то внутренняя ошибка. Чтобы решить эту проблему, лучше всего создать тикет в панели управления.|  ### Структура успешного ответа Все конечные точки будут возвращать данные в формате `JSON`. Ответы на `GET`-запросы будут иметь на верхнем уровне следующую структуру атрибутов:  |Название поля|Тип|Описание| |--- |--- |--- | |[entity_name]|object, object[], string[], number[], boolean|Динамическое поле, которое будет меняться в зависимости от запрашиваемого ресурса и будет содержать все атрибуты, необходимые для описания этого ресурса. Например, при запросе списка баз данных будет возвращаться поле `dbs`, а при запросе конкретного облачного сервера `server`. Для некоторых конечных точек в ответе может возвращаться сразу несколько ресурсов.| |meta|object|Опционально. Объект, который содержит вспомогательную информацию о ресурсе. Чаще всего будет встречаться при запросе коллекций и содержать поле `total`, которое будет указывать на количество элементов в коллекции.| |response_id|string|Опционально. В большинстве случаев в ответе будет содержаться ID ответа в формате UUIDv4, который однозначно указывает на ваш запрос внутри нашей системы. Если вам потребуется задать вопрос нашей поддержке, приложите к вопросу этот ID— так мы сможем найти ответ на него намного быстрее. Также вы можете использовать этот ID, чтобы убедиться, что это новый ответ на запрос и результат не был получен из кэша.|  Пример запроса на получение списка SSH-ключей: ```     HTTP/2.0 200 OK     {       \"ssh_keys\":[           {             \"body\":\"ssh-rsa AAAAB3NzaC1sdfghjkOAsBwWhs= example@device.local\",             \"created_at\":\"2021-09-15T19:52:27Z\",             \"expired_at\":null,             \"id\":5297,             \"is_default\":false,             \"name\":\"example@device.local\",             \"used_at\":null,             \"used_by\":[]           }       ],       \"meta\":{           \"total\":1       },       \"response_id\":\"94608d15-8672-4eed-8ab6-28bd6fa3cdf7\"     } ```  ### Структура ответа с ошибкой |Название поля|Тип|Описание| |--- |--- |--- | |status_code|number|Короткий числовой идентификатор ошибки.| |error_code|string|Короткий текстовый идентификатор ошибки, который уточняет числовой идентификатор и удобен для программной обработки. Самый простой пример — это код `not_found` для ошибки 404.| |message|string, string[]|Опционально. В большинстве случаев в ответе будет содержаться человекочитаемое подробное описание ошибки или ошибок, которые помогут понять, что нужно исправить.| |response_id|string|Опционально. В большинстве случае в ответе будет содержаться ID ответа в формате UUIDv4, который однозначно указывает на ваш запрос внутри нашей системы. Если вам потребуется задать вопрос нашей поддержке, приложите к вопросу этот ID — так мы сможем найти ответ на него намного быстрее.|  Пример: ```     HTTP/2.0 403 Forbidden     {       \"status_code\": 403,       \"error_code\":  \"forbidden\",       \"message\":     \"You do not have access for the attempted action\",       \"response_id\": \"94608d15-8672-4eed-8ab6-28bd6fa3cdf7\"     } ```  ## Статусы ресурсов Важно учесть, что при создании большинства ресурсов внутри платформы вам будет сразу возвращен ответ от сервера со статусом `200 OK` или `201 Created` и ID созданного ресурса в теле ответа, но при этом этот ресурс может быть ещё в *состоянии запуска*.  Для того чтобы понять, в каком состоянии сейчас находится ваш ресурс, мы добавили поле `status` в ответ на получение информации о ресурсе.  Список статусов будет отличаться в зависимости от типа ресурса. Увидеть поддерживаемый список статусов вы сможете в описании каждого конкретного ресурса.     ## Ограничение скорости запросов (Rate Limiting) Чтобы обеспечить стабильность для всех пользователей, Timeweb Cloud защищает API от всплесков входящего трафика, анализируя количество запросов c каждого аккаунта к каждой конечной точке.  Если ваше приложение отправляет более 20 запросов в секунду на одну конечную точку, то для этого запроса API может вернуть код состояния HTTP `429 Too Many Requests`.   ## Аутентификация Доступ к API осуществляется с помощью JWT-токена. Токенами можно управлять внутри панели управления Timeweb Cloud в разделе *API и Terraform*.  Токен необходимо передавать в заголовке каждого запроса в формате: ```   Authorization: Bearer $TIMEWEB_CLOUD_TOKEN ```  ## Формат примеров API Примеры в этой документации описаны с помощью `curl`, HTTP-клиента командной строки. На компьютерах `Linux` и `macOS` обычно по умолчанию установлен `curl`, и он доступен для загрузки на всех популярных платформах, включая `Windows`.  Каждый пример разделен на несколько строк символом `\\`, который совместим с `bash`. Типичный пример выглядит так: ```   curl -X PATCH      -H \"Content-Type: application/json\"      -H \"Authorization: Bearer $TIMEWEB_CLOUD_TOKEN\"      -d '{\"name\":\"Cute Corvus\",\"comment\":\"Development Server\"}'      \"https://api.timeweb.cloud/api/v1/dedicated/1051\" ``` - Параметр `-X` задает метод запроса. Для согласованности метод будет указан во всех примерах, даже если он явно не требуется для методов `GET`. - Строки `-H` задают требуемые HTTP-заголовки. - Примеры, для которых требуется объект JSON в теле запроса, передают требуемые данные через параметр `-d`.  Чтобы использовать приведенные примеры, не подставляя каждый раз в них свой токен, вы можете добавить токен один раз в переменные окружения в вашей консоли. Например, на `Linux` это можно сделать с помощью команды:  ``` TIMEWEB_CLOUD_TOKEN=\"token\" ```  После этого токен будет автоматически подставляться в ваши запросы.  Обратите внимание, что все значения в этой документации являются примерами. Не полагайтесь на IDы операционных систем, тарифов и т.д., используемые в примерах. Используйте соответствующую конечную точку для получения значений перед созданием ресурсов.   ## Версионирование API построено согласно принципам [семантического версионирования](https://semver.org/lang/ru). Это значит, что мы гарантируем обратную совместимость всех изменений в пределах одной мажорной версии.  Мажорная версия каждой конечной точки обозначается в пути запроса, например, запрос `/api/v1/servers` указывает, что этот метод имеет версию 1.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: info@timeweb.cloud
 * Generated by: https://openapi-generator.tech
 */

use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

use super::{ContentType, Error, configuration};
use crate::{apis::ResponseContent, models};

/// struct for typed errors of method [`add_ai_solution_telegram_bot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddAiSolutionTelegramBotError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status502(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`approve_ai_solution_telegram_pairing`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApproveAiSolutionTelegramPairingError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status502(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_ai_solution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiSolutionError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status402(models::ErrorResponse),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_ai_solution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiSolutionError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_solution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiSolutionError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_solution_chat_full_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiSolutionChatFullHistoryError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_solution_chat_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiSolutionChatHistoryError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_solution_chat_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiSolutionChatStatusError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_solution_main_chat_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiSolutionMainChatHistoryError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`list_ai_solution_conversations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiSolutionConversationsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`list_ai_solution_telegram_bots`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiSolutionTelegramBotsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status502(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`list_ai_solution_types`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiSolutionTypesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`list_ai_solutions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiSolutionsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`regenerate_ai_solution_ai_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegenerateAiSolutionAiKeyError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status502(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`remove_ai_solution_telegram_bot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveAiSolutionTelegramBotError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status502(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`reset_ai_solution_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetAiSolutionChatError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status502(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`send_ai_solution_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendAiSolutionMessageError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status402(models::ErrorResponse),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status502(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`shutdown_ai_solution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShutdownAiSolutionError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status502(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`start_ai_solution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StartAiSolutionError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status502(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`stop_ai_solution_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopAiSolutionChatError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status502(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`stream_ai_solution_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamAiSolutionChatError {
    Status400(models::StreamError),
    Status403(models::StreamError),
    Status404(models::StreamError),
    Status500(models::StreamError),
    Status503(models::StreamError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_ai_solution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiSolutionError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status502(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value)
}

/// Чтобы подключить Telegram-бота к решению, отправьте POST-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/telegram/bots`.  Принимает токен
/// бота от @BotFather, проверяет его в Telegram и настраивает бота на сервере
/// решения. Владелец подтверждается кодом: пока он не подтверждён, бот никому
/// не отвечает. Напишите боту в личные сообщения, получите в ответ код и
/// передайте его в метод подтверждения владельца.  К решению можно подключить
/// только одного бота: повторный запрос для того же бота обновляет его токен,
/// для другого бота — возвращает 409
/// `ai_solution_already_has_a_connected_telegram_bot`. Если Telegram отклонил
/// токен, ответ — 400 `ai_solution_telegram_token_is_invalid`.  Перед вызовом
/// убедитесь, что чат готов (`ready: true` в методе получения статуса чата):
/// пока сервер решения разворачивается или выключен, запрос завершается
/// ошибкой.
pub async fn add_ai_solution_telegram_bot(
    configuration: &configuration::Configuration,
    solution_id: i32,
    connect_telegram: models::ConnectTelegram
) -> Result<models::AddAiSolutionTelegramBot201Response, Error<AddAiSolutionTelegramBotError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;
    let p_body_connect_telegram = connect_telegram;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/telegram/bots",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_connect_telegram);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::AddAiSolutionTelegramBot201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AddAiSolutionTelegramBot201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<AddAiSolutionTelegramBotError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы подтвердить владельца Telegram-бота, отправьте POST-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/telegram/bots/{account_key}/
/// pairing/approve`.  Передайте код, который бот прислал в личные сообщения в
/// ответ на первое сообщение. После подтверждения агент отвечает этому
/// пользователю Telegram в боте, а бот переходит в состояние `connected`.
/// Владелец привязывается к конкретному боту. Повторный запрос с кодом, уже
/// подтверждённым в последние 15 минут, считается успешным. Некорректный
/// `account_key` — ответ 400 `ai_solution_telegram_token_is_invalid`.
pub async fn approve_ai_solution_telegram_pairing(
    configuration: &configuration::Configuration,
    solution_id: i32,
    account_key: &str,
    approve_telegram_pairing: models::ApproveTelegramPairing
) -> Result<(), Error<ApproveAiSolutionTelegramPairingError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;
    let p_path_account_key = account_key;
    let p_body_approve_telegram_pairing = approve_telegram_pairing;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/telegram/bots/{account_key}/pairing/approve",
        configuration.base_path,
        solution_id = p_path_solution_id,
        account_key = crate::apis::urlencode(p_path_account_key)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_approve_telegram_pairing);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<ApproveAiSolutionTelegramPairingError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы создать AI-решение, отправьте POST-запрос на
/// `/api/v1/cloud-ai/solutions`.  Выпускает для решения ключ AI Gateway и
/// создаёт облачный сервер из образа решения; агент на сервере настраивается на
/// выбранную модель через этот ключ. Сервер тарифицируется по выбранному
/// тарифу, обращения к модели — через ключ AI Gateway.  Решение возвращается в
/// статусе `creating`: развёртывание сервера и настройка агента занимают
/// несколько минут. Дождитесь статуса `active`, а затем `ready: true` в методе
/// получения статуса чата.  Тариф (`preset_id`) должен быть из той же локации,
/// что и зона доступности (`availability_zone` или зона по умолчанию для
/// региона аккаунта), иначе — 400 `invalid_location_zone`. Соответствие тарифа
/// `requirements` типа решения при создании не проверяется — выбирайте тариф не
/// ниже требований сами. Ответ 409 `no_free_node` означает, что в выбранной
/// зоне сейчас нет свободных ресурсов под этот тариф — выберите другой тариф
/// или зону. Неподходящая или устаревшая модель — 404 `model_not_found`.
pub async fn create_ai_solution(
    configuration: &configuration::Configuration,
    create_ai_solution: models::CreateAiSolution
) -> Result<models::CreateAiSolution201Response, Error<CreateAiSolutionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_ai_solution = create_ai_solution;

    let uri_str = format!("{}/api/v1/cloud-ai/solutions", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_ai_solution);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAiSolution201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAiSolution201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<CreateAiSolutionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы удалить AI-решение, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}`.  Решение удаляется вместе с
/// сервером и ключом AI Gateway. Действие необратимо: решение и история его
/// чата становятся недоступны.
pub async fn delete_ai_solution(
    configuration: &configuration::Configuration,
    solution_id: i32
) -> Result<(), Error<DeleteAiSolutionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<DeleteAiSolutionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить AI-решение, отправьте GET-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}`.  Статус решения выводится из
/// текущего состояния его сервера. Тело ответа будет представлять собой объект
/// JSON с ключом `ai_solution`.
pub async fn get_ai_solution(
    configuration: &configuration::Configuration,
    solution_id: i32
) -> Result<models::CreateAiSolution201Response, Error<GetAiSolutionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAiSolution201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAiSolution201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAiSolutionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить сообщения всех разговоров решения, отправьте GET-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/chat/history`.  В отличие от
/// `chat/messages` (только основной разговор) и
/// `chat/conversations/{conversation_id}/messages` (конкретный разговор),
/// возвращает сообщения из всех разговоров сразу. Сообщения отсортированы по
/// дате создания от новых к старым. Тело ответа будет представлять собой объект
/// JSON с ключами `ai_solution_chat_messages` и `meta`.
pub async fn get_ai_solution_chat_full_history(
    configuration: &configuration::Configuration,
    solution_id: i32,
    limit: Option<i32>,
    offset: Option<i32>
) -> Result<
    models::GetAiSolutionMainChatHistory200Response,
    Error<GetAiSolutionChatFullHistoryError>
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;
    let p_query_limit = limit;
    let p_query_offset = offset;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/chat/history",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::GetAiSolutionMainChatHistory200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAiSolutionMainChatHistory200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAiSolutionChatFullHistoryError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить историю разговора, отправьте GET-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/chat/conversations/
/// {conversation_id}/messages`.  Сообщения отсортированы по дате создания от
/// новых к старым: первая страница — последние сообщения разговора. Для
/// несуществующего разговора возвращается пустой список. Тело ответа будет
/// представлять собой объект JSON с ключами `ai_solution_chat_messages` и
/// `meta`.
pub async fn get_ai_solution_chat_history(
    configuration: &configuration::Configuration,
    solution_id: i32,
    conversation_id: &str,
    limit: Option<i32>,
    offset: Option<i32>
) -> Result<models::GetAiSolutionMainChatHistory200Response, Error<GetAiSolutionChatHistoryError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;
    let p_path_conversation_id = conversation_id;
    let p_query_limit = limit;
    let p_query_offset = offset;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/chat/conversations/{conversation_id}/messages",
        configuration.base_path,
        solution_id = p_path_solution_id,
        conversation_id = crate::apis::urlencode(p_path_conversation_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::GetAiSolutionMainChatHistory200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAiSolutionMainChatHistory200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAiSolutionChatHistoryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы узнать, готов ли агент решения принимать сообщения, отправьте
/// GET-запрос на `/api/v1/cloud-ai/solutions/{solution_id}/chat/status`.
/// Готовность чата отличается от статуса решения: тот выводится из состояния
/// сервера, а сервер включается задолго до того, как будет установлено и
/// настроено ПО агента. После создания или запуска решения опрашивайте этот
/// метод, пока `ready` не станет `true`. Тело ответа будет представлять собой
/// объект JSON с ключом `ai_solution_chat_status`.
pub async fn get_ai_solution_chat_status(
    configuration: &configuration::Configuration,
    solution_id: i32
) -> Result<models::GetAiSolutionChatStatus200Response, Error<GetAiSolutionChatStatusError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/chat/status",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::GetAiSolutionChatStatus200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAiSolutionChatStatus200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAiSolutionChatStatusError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить историю основного разговора решения, отправьте GET-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/chat/messages`.  ID основного
/// разговора знать не нужно; для других разговоров используйте метод с явным
/// `conversation_id`. Сообщения отсортированы по дате создания от новых к
/// старым: первая страница — последние сообщения. Тело ответа будет
/// представлять собой объект JSON с ключами `ai_solution_chat_messages` и
/// `meta` (`meta.total` — общее количество сообщений).
pub async fn get_ai_solution_main_chat_history(
    configuration: &configuration::Configuration,
    solution_id: i32,
    limit: Option<i32>,
    offset: Option<i32>
) -> Result<
    models::GetAiSolutionMainChatHistory200Response,
    Error<GetAiSolutionMainChatHistoryError>
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;
    let p_query_limit = limit;
    let p_query_offset = offset;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/chat/messages",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::GetAiSolutionMainChatHistory200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAiSolutionMainChatHistory200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAiSolutionMainChatHistoryError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить список разговоров с агентом решения, отправьте GET-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/chat/conversations`.  Возвращает
/// все разговоры решения, в которых есть сообщения, включая основной,
/// отсортированные по дате последнего сообщения от новых к старым. Тело ответа
/// будет представлять собой объект JSON с ключами `ai_solution_conversations` и
/// `meta`.
pub async fn list_ai_solution_conversations(
    configuration: &configuration::Configuration,
    solution_id: i32
) -> Result<models::ListAiSolutionConversations200Response, Error<ListAiSolutionConversationsError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/chat/conversations",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::ListAiSolutionConversations200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ListAiSolutionConversations200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<ListAiSolutionConversationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить список Telegram-ботов решения, отправьте GET-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/telegram/bots`.  Возвращает
/// подключённых к решению ботов и состояние их подтверждения. Список читается с
/// сервера решения. Тело ответа будет представлять собой объект JSON с ключами
/// `ai_solution_telegram_bots` и `meta`.  Перед вызовом убедитесь, что чат
/// готов (`ready: true` в методе получения статуса чата): пока сервер решения
/// разворачивается или выключен, запрос завершается ошибкой.
pub async fn list_ai_solution_telegram_bots(
    configuration: &configuration::Configuration,
    solution_id: i32
) -> Result<models::ListAiSolutionTelegramBots200Response, Error<ListAiSolutionTelegramBotsError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/telegram/bots",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::ListAiSolutionTelegramBots200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ListAiSolutionTelegramBots200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<ListAiSolutionTelegramBotsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить список типов AI-решений, отправьте GET-запрос на
/// `/api/v1/cloud-ai/solutions/types`.  Возвращает каталог готовых решений,
/// которые можно развернуть на облачном сервере, с минимальными требованиями к
/// конфигурации сервера (`requirements`). Тело ответа будет представлять собой
/// объект JSON с ключами `ai_solution_types` и `meta`.
pub async fn list_ai_solution_types(
    configuration: &configuration::Configuration
) -> Result<models::ListAiSolutionTypes200Response, Error<ListAiSolutionTypesError>> {
    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/types",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::ListAiSolutionTypes200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ListAiSolutionTypes200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<ListAiSolutionTypesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить список AI-решений аккаунта, отправьте GET-запрос на
/// `/api/v1/cloud-ai/solutions`.  Возвращает все решения аккаунта, кроме
/// удалённых. Статус каждого решения выводится из текущего состояния его
/// сервера. Тело ответа будет представлять собой объект JSON с ключами
/// `ai_solutions` и `meta`.
pub async fn list_ai_solutions(
    configuration: &configuration::Configuration
) -> Result<models::ListAiSolutions200Response, Error<ListAiSolutionsError>> {
    let uri_str = format!("{}/api/v1/cloud-ai/solutions", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::ListAiSolutions200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ListAiSolutions200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<ListAiSolutionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы перевыпустить ключ AI Gateway решения, отправьте POST-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/ai-key/regenerate`.  Единственный
/// способ сменить этот ключ: он выпущен платформой для решения и в разделе AI
/// Gateway доступен только для чтения. Новый ключ сразу записывается в
/// конфигурацию агента на сервере решения, старый перестаёт работать
/// немедленно. Сервер должен быть включён и полностью развёрнут — это
/// проверяется до перевыпуска, потому что отменить его нельзя. Если записать
/// новый ключ на сервер не удалось, повторите запрос.
pub async fn regenerate_ai_solution_ai_key(
    configuration: &configuration::Configuration,
    solution_id: i32
) -> Result<models::RegenerateAiSolutionAiKey200Response, Error<RegenerateAiSolutionAiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/ai-key/regenerate",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::RegenerateAiSolutionAiKey200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::RegenerateAiSolutionAiKey200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<RegenerateAiSolutionAiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы отключить Telegram-бота от решения, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/telegram/bots/{account_key}`.
/// Удаляет бота из конфигурации сервера решения: агент перестаёт отвечать в
/// этом боте. Если бот не подключён, запрос всё равно возвращает 204.
/// Некорректный `account_key` — ответ 400
/// `ai_solution_telegram_token_is_invalid`.  Перед вызовом убедитесь, что чат
/// готов (`ready: true` в методе получения статуса чата): пока сервер решения
/// разворачивается или выключен, запрос завершается ошибкой.
pub async fn remove_ai_solution_telegram_bot(
    configuration: &configuration::Configuration,
    solution_id: i32,
    account_key: &str
) -> Result<(), Error<RemoveAiSolutionTelegramBotError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;
    let p_path_account_key = account_key;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/telegram/bots/{account_key}",
        configuration.base_path,
        solution_id = p_path_solution_id,
        account_key = crate::apis::urlencode(p_path_account_key)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<RemoveAiSolutionTelegramBotError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы сбросить контекст разговора, отправьте POST-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/chat/reset`.  Контекст
/// сбрасывается на месте: разговор сохраняет `conversation_id`, но агент больше
/// не видит предыдущих сообщений. Без `conversation_id` сбрасывается основной
/// разговор решения. В историю добавляется служебное сообщение с ролью
/// `context-reset`, отмечающее границу текущего контекста, — оно и возвращается
/// в ответе (с ключом `ai_solution_chat_message`). Отметка добавляется и для
/// разговора, в котором ещё не было сообщений.  Перед вызовом убедитесь, что
/// чат готов (`ready: true` в методе получения статуса чата): пока сервер
/// решения разворачивается или выключен, запрос завершается ошибкой.
pub async fn reset_ai_solution_chat(
    configuration: &configuration::Configuration,
    solution_id: i32,
    reset_ai_solution_chat: Option<models::ResetAiSolutionChat>
) -> Result<models::ResetAiSolutionChat200Response, Error<ResetAiSolutionChatError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;
    let p_body_reset_ai_solution_chat = reset_ai_solution_chat;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/chat/reset",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_reset_ai_solution_chat);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::ResetAiSolutionChat200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ResetAiSolutionChat200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<ResetAiSolutionChatError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы отправить сообщение агенту решения, отправьте POST-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/chat/messages`.  Ответ
/// возвращается целиком после завершения хода агента (ожидание — до 3 минут).
/// Без `conversation_id` сообщение уходит в основной разговор решения; новый
/// разговор начинается только при явной передаче нового ID. Без `message` агент
/// сам здоровается — так можно показать первую реплику чата. Чтобы получать
/// ответ по мере генерации, используйте потоковый метод.  Перед вызовом
/// убедитесь, что чат готов (`ready: true` в методе получения статуса чата):
/// пока сервер решения разворачивается или выключен, запрос завершается
/// ошибкой.
pub async fn send_ai_solution_message(
    configuration: &configuration::Configuration,
    solution_id: i32,
    send_ai_solution_message: models::SendAiSolutionMessage
) -> Result<models::SendAiSolutionMessage200Response, Error<SendAiSolutionMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;
    let p_body_send_ai_solution_message = send_ai_solution_message;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/chat/messages",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_send_ai_solution_message);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::SendAiSolutionMessage200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SendAiSolutionMessage200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<SendAiSolutionMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы выключить AI-решение, отправьте POST-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/shutdown`.  Выключает сервер
/// решения. Действие асинхронное: следите за результатом по статусу решения
/// (обычно 1–2 минуты до `stopped`). Пока сервер выключен, чат недоступен, а
/// сервер продолжает тарифицироваться. Выключение уже выключенного решения
/// возвращает 204. Пока над сервером выполняется другое действие, ответ — 409
/// `ai_solution_is_busy`.
pub async fn shutdown_ai_solution(
    configuration: &configuration::Configuration,
    solution_id: i32
) -> Result<(), Error<ShutdownAiSolutionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/shutdown",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<ShutdownAiSolutionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы запустить AI-решение, отправьте POST-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/start`.  Включает сервер решения.
/// Действие асинхронное: следите за результатом по статусу решения, а
/// готовность агента проверяйте методом получения статуса чата. Пока над
/// сервером выполняется действие, запуск и выключение возвращают 409
/// `ai_solution_is_busy`.  Не вызывайте метод для работающего решения: запрос
/// будет принят (204) и запустит действие над сервером, а решение на несколько
/// минут станет занятым.
pub async fn start_ai_solution(
    configuration: &configuration::Configuration,
    solution_id: i32
) -> Result<(), Error<StartAiSolutionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/start",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<StartAiSolutionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы остановить текущий ход агента, отправьте POST-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}/chat/stop`.  Прерывает генерацию
/// ответа — например, если ответ зациклился или тратит слишком много токенов.
/// Без `conversation_id` останавливается текущий ход основного разговора
/// решения. Ход прерывается на стороне агента: закрыть соединение недостаточно,
/// потому что принятый агентом ход продолжает выполняться независимо от
/// клиента. Если хода нет, запрос всё равно возвращает 204.  Перед вызовом
/// убедитесь, что чат готов (`ready: true` в методе получения статуса чата):
/// пока сервер решения разворачивается или выключен, запрос завершается
/// ошибкой.
pub async fn stop_ai_solution_chat(
    configuration: &configuration::Configuration,
    solution_id: i32,
    stop_ai_solution_chat: Option<models::StopAiSolutionChat>
) -> Result<(), Error<StopAiSolutionChatError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;
    let p_body_stop_ai_solution_chat = stop_ai_solution_chat;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/chat/stop",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_stop_ai_solution_chat);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<StopAiSolutionChatError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы отправить сообщение агенту решения и получать ответ потоком, отправьте POST-запрос на `https://agent.timeweb.cloud/api/v1/cloud-ai/solutions/{solution_id}/chat/stream`.  Метод работает на `https://agent.timeweb.cloud`, а не на `https://api.timeweb.cloud`: шлюз публичного API накапливает ответ целиком и не может передавать его по частям. Авторизация та же — API-токен в заголовке `Authorization`; токену с ограниченными правами нужен доступ на запись к AI-сервисам. Тело запроса такое же, как у метода отправки сообщения.  Ответ — `text/event-stream`: каждое событие приходит как `event: <тип>` и `data: <JSON>`. Отправка сообщения и подписка на ответ — один запрос, поэтому начало ответа не теряется. Строки, начинающиеся с `:`, — комментарии для поддержания соединения (каждые 15 секунд), их нужно игнорировать. ID разговора возвращается в заголовке ответа `X-Conversation-Id`.  Типы событий: - `subscribed` `{}` — ход принят; - `delta` `{text, seq, runId}` — очередной фрагмент ответа; - `tool` `{name, target, phase, status, seq, runId}` — агент использует инструмент; - `message` `{text, seq, id, stopReason}` — финальный ответ целиком, всегда после последнего `delta`; - `done` `{phase, seq, runId}` — ход завершён; - `error` `{reason, message, budget?}` — ход не удался. `reason`: `empty_reply` — агент не выдал текста; `stream_failed` — поток прервался; `agent_failed` — ход упал на стороне агента при исправном соединении, повтор имеет смысл; `budget_exceeded` — исчерпан лимит расходов ключа AI Gateway решения, в `budget` `{max, spent, resetAt}` — цифры лимита, если они известны. `error` всегда последнее событие потока и может прийти после `done` (например, с `empty_reply`).  Ошибки, возникшие до начала потока (решение не найдено, некорректный запрос, нет доступа), возвращаются обычным JSON-ответом с соответствующим HTTP-статусом. Если сервер решения выключен или недоступен, поток может открыться и сразу завершиться событием `error` с `reason: stream_failed` — проверяйте готовность чата заранее методом получения статуса чата.
pub async fn stream_ai_solution_chat(
    configuration: &configuration::Configuration,
    solution_id: i32,
    send_ai_solution_message: models::SendAiSolutionMessage
) -> Result<String, Error<StreamAiSolutionChatError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;
    let p_body_send_ai_solution_message = send_ai_solution_message;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}/chat/stream",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_send_ai_solution_message);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `String`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `String`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<StreamAiSolutionChatError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы изменить AI-решение, отправьте PATCH-запрос на
/// `/api/v1/cloud-ai/solutions/{solution_id}`.  Меняются только переданные
/// поля; `description: null` очищает описание. Смена модели переписывает
/// конфигурацию агента на сервере решения и перезапускает агента, поэтому
/// сервер должен быть включён и полностью развёрнут — иначе запрос завершится
/// ошибкой и ничего не изменится. Смена только названия или описания сервер не
/// затрагивает. Неподходящая модель — 404 `model_not_found`.
pub async fn update_ai_solution(
    configuration: &configuration::Configuration,
    solution_id: i32,
    update_ai_solution: models::UpdateAiSolution
) -> Result<models::CreateAiSolution201Response, Error<UpdateAiSolutionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_solution_id = solution_id;
    let p_body_update_ai_solution = update_ai_solution;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/solutions/{solution_id}",
        configuration.base_path,
        solution_id = p_path_solution_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_ai_solution);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAiSolution201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAiSolution201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<UpdateAiSolutionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}
