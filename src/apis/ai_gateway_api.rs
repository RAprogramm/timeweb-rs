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

/// struct for typed errors of method [`create_ai_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiKeyError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_ai_key_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiKeyLimitError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_ai_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiKeyError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_ai_key_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiKeyLimitError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_key_calls_statistic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiKeyCallsStatisticError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_key_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiKeyLimitError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_key_limits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiKeyLimitsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_key_models_statistic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiKeyModelsStatisticError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_key_token_usage_statistic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiKeyTokenUsageStatisticError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiKeysError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`regenerate_ai_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegenerateAiKeyError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`sync_ai_key_limits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyncAiKeyLimitsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_ai_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiKeyError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_ai_key_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiKeyLimitError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// Чтобы создать ключ AI Gateway, отправьте POST-запрос на
/// `/api/v1/cloud-ai/ai-keys`.  Значение ключа (`sk-...`) возвращается в поле
/// `key` только в ответе на этот запрос — сохраните его. Если значение утеряно,
/// перевыпустите ключ.  Если список `fallback_models` некорректен, вернётся 400
/// с `error_code` `fallback_chain_invalid` (больше двух моделей или повторы,
/// правило — в `details.reason`) или `fallback_model_not_found` (модели нет
/// среди действующих текстовых моделей, список — в `details.model_names`).
pub async fn create_ai_key(
    configuration: &configuration::Configuration,
    create_ai_key: models::CreateAiKey
) -> Result<models::CreateAiKey201Response, Error<CreateAiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_ai_key = create_ai_key;

    let uri_str = format!("{}/api/v1/cloud-ai/ai-keys", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_ai_key);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAiKey201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAiKey201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы ограничить расход ключа AI Gateway, отправьте POST-запрос на
/// `/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits`.  Лимит задаётся в токенах
/// (`limit`) или в деньгах (`money_limit`, приоритетнее) на период `limit_type`
/// — для всех моделей или для одной (`model_id`). Когда лимит исчерпан, запросы
/// ключа к AI Gateway API завершаются ошибкой 422 до начала следующего периода.
/// Если от лимита осталась часть, максимальный размер ответа текстового запроса
/// уменьшается так, чтобы уложиться в остаток.  На один период и одну модель
/// (или на все модели) можно завести только один лимит — повторный вернёт 409 с
/// `error_code` `entity_keys_limit_already_exists`. Если не передан ни `limit`,
/// ни `money_limit`, вернётся 400 с `error_code`
/// `entity_keys_limit_value_required`.
pub async fn create_ai_key_limit(
    configuration: &configuration::Configuration,
    key_id: &str,
    create_ai_key_limit: models::CreateAiKeyLimit
) -> Result<models::CreateAiKeyLimit201Response, Error<CreateAiKeyLimitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_key_id = key_id;
    let p_body_create_ai_key_limit = create_ai_key_limit;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits",
        configuration.base_path,
        key_id = crate::apis::urlencode(p_path_key_id)
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
    req_builder = req_builder.json(&p_body_create_ai_key_limit);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAiKeyLimit201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAiKeyLimit201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAiKeyLimitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить ключ AI Gateway, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/ai-keys/{key_id}`.  После удаления запросы с этим ключом
/// перестают приниматься. Ключи, которыми управляет платформа (`is_readonly:
/// true`), удалить нельзя: вернётся 403 с `error_code` `ai_key_is_readonly`.
pub async fn delete_ai_key(
    configuration: &configuration::Configuration,
    key_id: &str
) -> Result<(), Error<DeleteAiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_key_id = key_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/{key_id}",
        configuration.base_path,
        key_id = crate::apis::urlencode(p_path_key_id)
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
        let content = resp.text().await?;
        let entity: Option<DeleteAiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить лимит ключа AI Gateway, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits/{limit_id}`.  Если лимит
/// относится к другому ключу, вернётся 403.
pub async fn delete_ai_key_limit(
    configuration: &configuration::Configuration,
    key_id: &str,
    limit_id: &str
) -> Result<(), Error<DeleteAiKeyLimitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_key_id = key_id;
    let p_path_limit_id = limit_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits/{limit_id}",
        configuration.base_path,
        key_id = crate::apis::urlencode(p_path_key_id),
        limit_id = crate::apis::urlencode(p_path_limit_id)
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
        let content = resp.text().await?;
        let entity: Option<DeleteAiKeyLimitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить количество запросов ключа по интервалам времени, отправьте
/// GET-запрос на `/api/v1/cloud-ai/ai-keys/statistic/calls`.  Ключ задаётся
/// параметрами `entity_id` (ID ключа) и `entity_type` (тип ключа). Если ключ не
/// найден, вернётся 404. Период по умолчанию — последние 24 часа. Точки
/// возвращаются для каждого интервала периода, включая интервалы без запросов.
/// Тело ответа будет представлять собой объект JSON с ключами
/// `ai_key_calls_statistics` и `meta`.
pub async fn get_ai_key_calls_statistic(
    configuration: &configuration::Configuration,
    entity_id: &str,
    entity_type: &str,
    model_name: Option<&str>,
    start_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    end_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    interval: Option<i32>
) -> Result<models::GetAiKeyCallsStatistic200Response, Error<GetAiKeyCallsStatisticError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_entity_id = entity_id;
    let p_query_entity_type = entity_type;
    let p_query_model_name = model_name;
    let p_query_start_time = start_time;
    let p_query_end_time = end_time;
    let p_query_interval = interval;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/statistic/calls",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("entity_id", &p_query_entity_id.to_string())]);
    req_builder = req_builder.query(&[("entity_type", &p_query_entity_type.to_string())]);
    if let Some(ref param_value) = p_query_model_name {
        req_builder = req_builder.query(&[("model_name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_time {
        req_builder = req_builder.query(&[("start_time", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_time {
        req_builder = req_builder.query(&[("end_time", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_interval {
        req_builder = req_builder.query(&[("interval", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAiKeyCallsStatistic200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAiKeyCallsStatistic200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAiKeyCallsStatisticError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить лимит ключа AI Gateway, отправьте GET-запрос на
/// `/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits/{limit_id}`.  Если лимит
/// относится к другому ключу, вернётся 403 с `error_code`
/// `entity_keys_limit_forbidden`.
pub async fn get_ai_key_limit(
    configuration: &configuration::Configuration,
    key_id: &str,
    limit_id: &str
) -> Result<models::CreateAiKeyLimit201Response, Error<GetAiKeyLimitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_key_id = key_id;
    let p_path_limit_id = limit_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits/{limit_id}",
        configuration.base_path,
        key_id = crate::apis::urlencode(p_path_key_id),
        limit_id = crate::apis::urlencode(p_path_limit_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAiKeyLimit201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAiKeyLimit201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAiKeyLimitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить лимиты расхода ключа AI Gateway, отправьте GET-запрос на
/// `/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits`.  Тело ответа будет
/// представлять собой объект JSON с ключами `ai_key_entity_limits` и `meta`.
pub async fn get_ai_key_limits(
    configuration: &configuration::Configuration,
    key_id: &str,
    entity_type: Option<&str>,
    limit_type: Option<&str>
) -> Result<models::GetAiKeyLimits200Response, Error<GetAiKeyLimitsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_key_id = key_id;
    let p_query_entity_type = entity_type;
    let p_query_limit_type = limit_type;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits",
        configuration.base_path,
        key_id = crate::apis::urlencode(p_path_key_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_entity_type {
        req_builder = req_builder.query(&[("entity_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit_type {
        req_builder = req_builder.query(&[("limit_type", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAiKeyLimits200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAiKeyLimits200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAiKeyLimitsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список моделей, к которым обращался ключ за период, отправьте
/// GET-запрос на `/api/v1/cloud-ai/ai-keys/statistic/models`.  Ключ задаётся
/// параметрами `entity_id` (ID ключа) и `entity_type` (тип ключа). Если ключ не
/// найден, вернётся 404. Период по умолчанию — последние 24 часа. Модели
/// отсортированы по алфавиту.  Тело ответа будет представлять собой объект JSON
/// с ключами `ai_key_models_statistics` и `meta`.
pub async fn get_ai_key_models_statistic(
    configuration: &configuration::Configuration,
    entity_id: &str,
    entity_type: &str,
    model_name: Option<&str>,
    start_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    end_time: Option<chrono::DateTime<chrono::FixedOffset>>
) -> Result<models::GetAiKeyModelsStatistic200Response, Error<GetAiKeyModelsStatisticError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_entity_id = entity_id;
    let p_query_entity_type = entity_type;
    let p_query_model_name = model_name;
    let p_query_start_time = start_time;
    let p_query_end_time = end_time;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/statistic/models",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("entity_id", &p_query_entity_id.to_string())]);
    req_builder = req_builder.query(&[("entity_type", &p_query_entity_type.to_string())]);
    if let Some(ref param_value) = p_query_model_name {
        req_builder = req_builder.query(&[("model_name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_time {
        req_builder = req_builder.query(&[("start_time", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_time {
        req_builder = req_builder.query(&[("end_time", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAiKeyModelsStatistic200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAiKeyModelsStatistic200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAiKeyModelsStatisticError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить расход токенов и стоимость запросов ключа по интервалам
/// времени, отправьте GET-запрос на
/// `/api/v1/cloud-ai/ai-keys/statistic/token-usage`.  Ключ задаётся параметрами
/// `entity_id` (ID ключа) и `entity_type` (тип ключа). Если ключ не найден,
/// вернётся 404. Период по умолчанию — последние 24 часа. Точки возвращаются
/// для каждого интервала периода, включая интервалы без запросов.  Тело ответа
/// будет представлять собой объект JSON с ключами `ai_key_token_statistics` и
/// `meta`.
pub async fn get_ai_key_token_usage_statistic(
    configuration: &configuration::Configuration,
    entity_id: &str,
    entity_type: &str,
    model_name: Option<&str>,
    start_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    end_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    interval: Option<i32>
) -> Result<models::GetAiKeyTokenUsageStatistic200Response, Error<GetAiKeyTokenUsageStatisticError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_entity_id = entity_id;
    let p_query_entity_type = entity_type;
    let p_query_model_name = model_name;
    let p_query_start_time = start_time;
    let p_query_end_time = end_time;
    let p_query_interval = interval;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/statistic/token-usage",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("entity_id", &p_query_entity_id.to_string())]);
    req_builder = req_builder.query(&[("entity_type", &p_query_entity_type.to_string())]);
    if let Some(ref param_value) = p_query_model_name {
        req_builder = req_builder.query(&[("model_name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_time {
        req_builder = req_builder.query(&[("start_time", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_time {
        req_builder = req_builder.query(&[("end_time", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_interval {
        req_builder = req_builder.query(&[("interval", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAiKeyTokenUsageStatistic200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAiKeyTokenUsageStatistic200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAiKeyTokenUsageStatisticError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список ключей AI Gateway, отправьте GET-запрос на
/// `/api/v1/cloud-ai/ai-keys`.  В списке — ключи, созданные пользователем, и
/// ключи кластеров Kubernetes, выпущенные платформой. Значения ключей
/// (`sk-...`) не возвращаются.  Тело ответа будет представлять собой объект
/// JSON с ключами `ai_keys` и `meta`.
pub async fn get_ai_keys(
    configuration: &configuration::Configuration
) -> Result<models::GetAiKeys200Response, Error<GetAiKeysError>> {
    let uri_str = format!("{}/api/v1/cloud-ai/ai-keys", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAiKeys200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAiKeys200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAiKeysError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы перевыпустить ключ AI Gateway, отправьте PUT-запрос на
/// `/api/v1/cloud-ai/ai-keys/{key_id}/regenerate`.  Ключ получает новое
/// значение, старое перестаёт действовать сразу. Название, лимиты, запасные
/// модели и статистика сохраняются. Новое значение (`sk-...`) возвращается в
/// поле `key` только в ответе на этот запрос — сохраните его. Поле `expires_at`
/// в ответе не заполняется.  Ключи, которыми управляет платформа (`is_readonly:
/// true`), перевыпустить нельзя: вернётся 403 с `error_code`
/// `ai_key_is_readonly`.
pub async fn regenerate_ai_key(
    configuration: &configuration::Configuration,
    key_id: &str
) -> Result<models::CreateAiKey201Response, Error<RegenerateAiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_key_id = key_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/{key_id}/regenerate",
        configuration.base_path,
        key_id = crate::apis::urlencode(p_path_key_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAiKey201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAiKey201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RegenerateAiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы задать все лимиты ключа AI Gateway на один период одним запросом,
/// отправьте PUT-запрос на
/// `/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits/sync`.  Лимиты из `limits`
/// создаются или обновляются (по `model_id`), а лимиты того же периода, которых
/// нет в запросе, удаляются. Изменения применяются целиком или не применяются
/// совсем.  Если в записи не задан ни `limit`, ни `money_limit` или
/// `money_limit` снимается без `limit`, вернётся 400 с `error_code`
/// `entity_keys_limit_value_required`. Если лимиты одновременно изменил другой
/// запрос, вернётся 409 с `error_code` `entity_keys_limits_sync_conflict` —
/// повторите запрос.  Тело ответа будет представлять собой объект JSON с
/// ключами `ai_key_entity_limits` и `meta` — лимиты периода после
/// синхронизации.
pub async fn sync_ai_key_limits(
    configuration: &configuration::Configuration,
    key_id: &str,
    sync_ai_key_limits: models::SyncAiKeyLimits
) -> Result<models::GetAiKeyLimits200Response, Error<SyncAiKeyLimitsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_key_id = key_id;
    let p_body_sync_ai_key_limits = sync_ai_key_limits;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits/sync",
        configuration.base_path,
        key_id = crate::apis::urlencode(p_path_key_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_sync_ai_key_limits);

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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAiKeyLimits200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAiKeyLimits200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SyncAiKeyLimitsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы изменить ключ AI Gateway, отправьте PATCH-запрос на
/// `/api/v1/cloud-ai/ai-keys/{key_id}`, передав изменяемые поля.  Ключи,
/// которыми управляет платформа (`is_readonly: true` — ключи AI-решений и
/// кластеров Kubernetes), изменить нельзя: вернётся 403 с `error_code`
/// `ai_key_is_readonly`.  Ошибки в `fallback_models` — как при создании ключа
/// (400). Поле `expires_at` в ответе этого запроса не заполняется — срок
/// действия ключа смотрите в списке ключей.
pub async fn update_ai_key(
    configuration: &configuration::Configuration,
    key_id: &str,
    update_ai_key: models::UpdateAiKey
) -> Result<models::UpdateAiKey200Response, Error<UpdateAiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_key_id = key_id;
    let p_body_update_ai_key = update_ai_key;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/{key_id}",
        configuration.base_path,
        key_id = crate::apis::urlencode(p_path_key_id)
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
    req_builder = req_builder.json(&p_body_update_ai_key);

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
                    "Received `text/plain` content type response that cannot be converted to `models::UpdateAiKey200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::UpdateAiKey200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateAiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы изменить лимит ключа AI Gateway, отправьте PATCH-запрос на
/// `/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits/{limit_id}`.  Не переданные
/// поля не меняются. Если не передан ни `limit`, ни `money_limit` или
/// `money_limit` снимается без `limit`, вернётся 400 с `error_code`
/// `entity_keys_limit_value_required`. Если лимит относится к другому ключу,
/// вернётся 403.
pub async fn update_ai_key_limit(
    configuration: &configuration::Configuration,
    key_id: &str,
    limit_id: &str,
    update_ai_key_limit: models::UpdateAiKeyLimit
) -> Result<models::CreateAiKeyLimit201Response, Error<UpdateAiKeyLimitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_key_id = key_id;
    let p_path_limit_id = limit_id;
    let p_body_update_ai_key_limit = update_ai_key_limit;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/ai-keys/{key_id}/entity-limits/{limit_id}",
        configuration.base_path,
        key_id = crate::apis::urlencode(p_path_key_id),
        limit_id = crate::apis::urlencode(p_path_limit_id)
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
    req_builder = req_builder.json(&p_body_update_ai_key_limit);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAiKeyLimit201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAiKeyLimit201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateAiKeyLimitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}
