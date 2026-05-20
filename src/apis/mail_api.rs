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

/// struct for typed errors of method [`create_domain_mailbox`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDomainMailboxError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_domain_mailbox_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDomainMailboxV2Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::CreateDatabaseBackup409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_multiple_domain_mailboxes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMultipleDomainMailboxesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_multiple_domain_mailboxes_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMultipleDomainMailboxesV2Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_mailbox`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMailboxError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_all_mailboxes_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllMailboxesV2Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_domain_mail_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDomainMailInfoError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_domain_mailboxes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDomainMailboxesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_mailbox`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMailboxError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_mailbox_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMailboxV2Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_mailboxes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMailboxesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_domain_mail_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDomainMailInfoError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_mailbox`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMailboxError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_mailbox_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMailboxV2Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status409(models::CreateDatabaseBackup409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// Чтобы создать почтовый ящик, отправьте POST-запрос на
/// `/api/v1/mail/domains/{domain}`.
#[deprecated]
pub async fn create_domain_mailbox(
    configuration: &configuration::Configuration,
    domain: &str,
    create_domain_mailbox_request: models::CreateDomainMailboxRequest
) -> Result<models::CreateDomainMailbox201Response, Error<CreateDomainMailboxError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;
    let p_body_create_domain_mailbox_request = create_domain_mailbox_request;

    let uri_str = format!(
        "{}/api/v1/mail/domains/{domain}",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain)
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
    req_builder = req_builder.json(&p_body_create_domain_mailbox_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainMailbox201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainMailbox201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateDomainMailboxError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы создать почтовый ящик, отправьте POST-запрос на
/// `/api/v2/mail/domains/{domain}`.
pub async fn create_domain_mailbox_v2(
    configuration: &configuration::Configuration,
    domain: &str,
    create_domain_mailbox_v2_request: models::CreateDomainMailboxV2Request
) -> Result<models::CreateDomainMailboxV2201Response, Error<CreateDomainMailboxV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;
    let p_body_create_domain_mailbox_v2_request = create_domain_mailbox_v2_request;

    let uri_str = format!(
        "{}/api/v2/mail/domains/{domain}",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain)
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
    req_builder = req_builder.json(&p_body_create_domain_mailbox_v2_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainMailboxV2201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainMailboxV2201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateDomainMailboxV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы создать почтовый ящики, отправьте POST-запрос на
/// `/api/v1/mail/domains/{domain}/batch`.
#[deprecated]
pub async fn create_multiple_domain_mailboxes(
    configuration: &configuration::Configuration,
    domain: &str,
    create_multiple_domain_mailboxes_request: models::CreateMultipleDomainMailboxesRequest
) -> Result<
    models::CreateMultipleDomainMailboxes201Response,
    Error<CreateMultipleDomainMailboxesError>
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;
    let p_body_create_multiple_domain_mailboxes_request = create_multiple_domain_mailboxes_request;

    let uri_str = format!(
        "{}/api/v1/mail/domains/{domain}/batch",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain)
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
    req_builder = req_builder.json(&p_body_create_multiple_domain_mailboxes_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateMultipleDomainMailboxes201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateMultipleDomainMailboxes201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateMultipleDomainMailboxesError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы создать несколько почтовых ящиков одновременно, отправьте POST-запрос
/// на `/api/v2/mail/domains/{domain}/batch`.
pub async fn create_multiple_domain_mailboxes_v2(
    configuration: &configuration::Configuration,
    domain: &str,
    create_multiple_domain_mailboxes_v2_request_inner: Vec<
        models::CreateMultipleDomainMailboxesV2RequestInner
    >
) -> Result<
    models::CreateMultipleDomainMailboxesV2201Response,
    Error<CreateMultipleDomainMailboxesV2Error>
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;
    let p_body_create_multiple_domain_mailboxes_v2_request_inner =
        create_multiple_domain_mailboxes_v2_request_inner;

    let uri_str = format!(
        "{}/api/v2/mail/domains/{domain}/batch",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain)
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
    req_builder = req_builder.json(&p_body_create_multiple_domain_mailboxes_v2_request_inner);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateMultipleDomainMailboxesV2201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateMultipleDomainMailboxesV2201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateMultipleDomainMailboxesV2Error> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить почтовый ящик, отправьте DELETE-запрос на
/// `/api/v1/mail/domains/{domain}/mailboxes/{mailbox}`.
pub async fn delete_mailbox(
    configuration: &configuration::Configuration,
    domain: &str,
    mailbox: &str
) -> Result<(), Error<DeleteMailboxError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;
    let p_path_mailbox = mailbox;

    let uri_str = format!(
        "{}/api/v1/mail/domains/{domain}/mailboxes/{mailbox}",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain),
        mailbox = crate::apis::urlencode(p_path_mailbox)
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
        let entity: Option<DeleteMailboxError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список всех почтовых ящиков, отправьте GET-запрос на
/// `/api/v2/mail`.
pub async fn get_all_mailboxes_v2(
    configuration: &configuration::Configuration,
    limit: Option<i32>,
    offset: Option<i32>,
    search: Option<&str>
) -> Result<models::GetAllMailboxesV2200Response, Error<GetAllMailboxesV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_search = search;

    let uri_str = format!("{}/api/v2/mail", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_search {
        req_builder = req_builder.query(&[("search", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAllMailboxesV2200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAllMailboxesV2200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAllMailboxesV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить почтовую информацию о домене, отправьте GET-запрос на
/// `/api/v1/mail/domains/{domain}/info`.
#[deprecated]
pub async fn get_domain_mail_info(
    configuration: &configuration::Configuration,
    domain: &str
) -> Result<models::GetDomainMailInfo200Response, Error<GetDomainMailInfoError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;

    let uri_str = format!(
        "{}/api/v1/mail/domains/{domain}/info",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain)
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetDomainMailInfo200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetDomainMailInfo200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDomainMailInfoError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список почтовых ящиков домена, отправьте GET-запрос на
/// `/api/v1/mail/domains/{domain}`.
pub async fn get_domain_mailboxes(
    configuration: &configuration::Configuration,
    domain: &str,
    limit: Option<i32>,
    offset: Option<i32>,
    search: Option<&str>
) -> Result<models::GetMailboxes200Response, Error<GetDomainMailboxesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_search = search;

    let uri_str = format!(
        "{}/api/v1/mail/domains/{domain}",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_search {
        req_builder = req_builder.query(&[("search", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetMailboxes200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetMailboxes200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDomainMailboxesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить почтовый ящик, отправьте GET-запрос на
/// `/api/v1/mail/domains/{domain}/mailboxes/{mailbox}`.
#[deprecated]
pub async fn get_mailbox(
    configuration: &configuration::Configuration,
    domain: &str,
    mailbox: &str
) -> Result<models::CreateDomainMailbox201Response, Error<GetMailboxError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;
    let p_path_mailbox = mailbox;

    let uri_str = format!(
        "{}/api/v1/mail/domains/{domain}/mailboxes/{mailbox}",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain),
        mailbox = crate::apis::urlencode(p_path_mailbox)
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainMailbox201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainMailbox201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMailboxError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить почтовый ящик, отправьте GET-запрос на
/// `/api/v2/mail/domains/{domain}/mailboxes/{mailbox}`.
pub async fn get_mailbox_v2(
    configuration: &configuration::Configuration,
    domain: &str,
    mailbox: &str
) -> Result<models::CreateDomainMailboxV2201Response, Error<GetMailboxV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;
    let p_path_mailbox = mailbox;

    let uri_str = format!(
        "{}/api/v2/mail/domains/{domain}/mailboxes/{mailbox}",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain),
        mailbox = crate::apis::urlencode(p_path_mailbox)
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainMailboxV2201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainMailboxV2201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMailboxV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список почтовых ящиков аккаунта, отправьте GET-запрос на
/// `/api/v1/mail`.
#[deprecated]
pub async fn get_mailboxes(
    configuration: &configuration::Configuration,
    limit: Option<i32>,
    offset: Option<i32>,
    search: Option<&str>
) -> Result<models::GetMailboxes200Response, Error<GetMailboxesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_search = search;

    let uri_str = format!("{}/api/v1/mail", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_search {
        req_builder = req_builder.query(&[("search", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetMailboxes200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetMailboxes200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMailboxesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы изменить почтовую информацию о домене, отправьте PATCH-запрос на
/// `/api/v1/mail/domains/{domain}/info`.
#[deprecated]
pub async fn update_domain_mail_info(
    configuration: &configuration::Configuration,
    domain: &str,
    update_domain_mail_info_request: models::UpdateDomainMailInfoRequest
) -> Result<models::GetDomainMailInfo200Response, Error<UpdateDomainMailInfoError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;
    let p_body_update_domain_mail_info_request = update_domain_mail_info_request;

    let uri_str = format!(
        "{}/api/v1/mail/domains/{domain}/info",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain)
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
    req_builder = req_builder.json(&p_body_update_domain_mail_info_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::GetDomainMailInfo200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetDomainMailInfo200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateDomainMailInfoError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы изменить почтовый ящик, отправьте PATCH-запрос на
/// `/api/v1/mail/domains/{domain}/mailboxes/{mailbox}`.
#[deprecated]
pub async fn update_mailbox(
    configuration: &configuration::Configuration,
    domain: &str,
    mailbox: &str,
    update_mailbox: models::UpdateMailbox
) -> Result<models::CreateDomainMailbox201Response, Error<UpdateMailboxError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;
    let p_path_mailbox = mailbox;
    let p_body_update_mailbox = update_mailbox;

    let uri_str = format!(
        "{}/api/v1/mail/domains/{domain}/mailboxes/{mailbox}",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain),
        mailbox = crate::apis::urlencode(p_path_mailbox)
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
    req_builder = req_builder.json(&p_body_update_mailbox);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainMailbox201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainMailbox201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateMailboxError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы изменить почтовый ящик, отправьте PATCH-запрос на
/// `/api/v2/mail/domains/{domain}/mailboxes/{mailbox}`.
pub async fn update_mailbox_v2(
    configuration: &configuration::Configuration,
    domain: &str,
    mailbox: &str,
    update_mailbox_v2: models::UpdateMailboxV2
) -> Result<models::UpdateMailboxV2200Response, Error<UpdateMailboxV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_domain = domain;
    let p_path_mailbox = mailbox;
    let p_body_update_mailbox_v2 = update_mailbox_v2;

    let uri_str = format!(
        "{}/api/v2/mail/domains/{domain}/mailboxes/{mailbox}",
        configuration.base_path,
        domain = crate::apis::urlencode(p_path_domain),
        mailbox = crate::apis::urlencode(p_path_mailbox)
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
    req_builder = req_builder.json(&p_body_update_mailbox_v2);

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
                    "Received `text/plain` content type response that cannot be converted to `models::UpdateMailboxV2200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::UpdateMailboxV2200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateMailboxV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}
