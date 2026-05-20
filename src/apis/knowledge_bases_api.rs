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
use tokio::fs::File as TokioFile;
use tokio_util::codec::{BytesCodec, FramedRead};

use super::{ContentType, Error, configuration};
use crate::{apis::ResponseContent, models};

/// struct for typed errors of method
/// [`add_additional_token_package_to_knowledgebase`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddAdditionalTokenPackageToKnowledgebaseError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_knowledgebase`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateKnowledgebaseError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_document`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDocumentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_knowledgebase`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteKnowledgebaseError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`download_document`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadDocumentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_knowledgebase`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetKnowledgebaseError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_knowledgebase_documents_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetKnowledgebaseDocumentsV2Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_knowledgebase_statistics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetKnowledgebaseStatisticsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_knowledgebases`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetKnowledgebasesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_knowledgebases_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetKnowledgebasesV2Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`link_knowledgebase_to_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkKnowledgebaseToAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`reindex_document`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReindexDocumentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`unlink_knowledgebase_from_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnlinkKnowledgebaseFromAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_knowledgebase`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateKnowledgebaseError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`upload_files_to_knowledgebase`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadFilesToKnowledgebaseError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// Чтобы добавить дополнительный пакет токенов для базы знаний, отправьте
/// POST-запрос на
/// `/api/v1/cloud-ai/knowledge-bases/{id}/add-additional-token-package`.
pub async fn add_additional_token_package_to_knowledgebase(
    configuration: &configuration::Configuration,
    id: i32,
    add_token_package: Option<models::AddTokenPackage>
) -> Result<(), Error<AddAdditionalTokenPackageToKnowledgebaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_add_token_package = add_token_package;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases/{id}/add-additional-token-package",
        configuration.base_path,
        id = p_path_id
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
    req_builder = req_builder.json(&p_body_add_token_package);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<AddAdditionalTokenPackageToKnowledgebaseError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы создать базу знаний, отправьте POST-запрос на
/// `/api/v1/cloud-ai/knowledge-bases`, задав необходимые атрибуты.  База знаний
/// будет создана с использованием предоставленной информации. Тело ответа будет
/// содержать объект JSON с информацией о созданной базе знаний.
pub async fn create_knowledgebase(
    configuration: &configuration::Configuration,
    create_knowledgebase: models::CreateKnowledgebase
) -> Result<models::CreateKnowledgebase201Response, Error<CreateKnowledgebaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_knowledgebase = create_knowledgebase;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases",
        configuration.base_path
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
    req_builder = req_builder.json(&p_body_create_knowledgebase);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateKnowledgebase201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateKnowledgebase201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateKnowledgebaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить документ из базы знаний, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/knowledge-bases/{id}/documents/{document_id}`.
pub async fn delete_document(
    configuration: &configuration::Configuration,
    id: i32,
    document_id: i32
) -> Result<(), Error<DeleteDocumentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_document_id = document_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases/{id}/documents/{document_id}",
        configuration.base_path,
        id = p_path_id,
        document_id = p_path_document_id
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
        let entity: Option<DeleteDocumentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить базу знаний, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/knowledge-bases/{id}`.
pub async fn delete_knowledgebase(
    configuration: &configuration::Configuration,
    id: i32
) -> Result<(), Error<DeleteKnowledgebaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases/{id}",
        configuration.base_path,
        id = p_path_id
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
        let entity: Option<DeleteKnowledgebaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы скачать документ из базы знаний, отправьте GET-запрос на
/// `/api/v1/cloud-ai/knowledge-bases/{id}/documents/{document_id}/download`.
pub async fn download_document(
    configuration: &configuration::Configuration,
    id: i32,
    document_id: i32
) -> Result<reqwest::Response, Error<DownloadDocumentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_document_id = document_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases/{id}/documents/{document_id}/download",
        configuration.base_path,
        id = p_path_id,
        document_id = p_path_document_id
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

    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text().await?;
        let entity: Option<DownloadDocumentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить информацию о базе знаний, отправьте GET-запрос на
/// `/api/v1/cloud-ai/knowledge-bases/{id}`.
pub async fn get_knowledgebase(
    configuration: &configuration::Configuration,
    id: i32
) -> Result<models::CreateKnowledgebase201Response, Error<GetKnowledgebaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases/{id}",
        configuration.base_path,
        id = p_path_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateKnowledgebase201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateKnowledgebase201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetKnowledgebaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список документов базы знаний, отправьте GET-запрос на
/// `/api/v2/cloud-ai/knowledge-bases/{id}/documents`.  Тело ответа будет
/// представлять собой объект JSON с ключами `knowledgebase_documents` и `meta`.
pub async fn get_knowledgebase_documents_v2(
    configuration: &configuration::Configuration,
    id: i32,
    limit: Option<i32>,
    offset: Option<i32>,
    sort_by: Option<&str>,
    sort_order: Option<&str>
) -> Result<models::GetKnowledgebaseDocumentsV2200Response, Error<GetKnowledgebaseDocumentsV2Error>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_sort_by = sort_by;
    let p_query_sort_order = sort_order;

    let uri_str = format!(
        "{}/api/v2/cloud-ai/knowledge-bases/{id}/documents",
        configuration.base_path,
        id = p_path_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sort_by {
        req_builder = req_builder.query(&[("sort_by", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sort_order {
        req_builder = req_builder.query(&[("sort_order", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetKnowledgebaseDocumentsV2200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetKnowledgebaseDocumentsV2200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetKnowledgebaseDocumentsV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить статистику использования токенов базы знаний, отправьте
/// GET-запрос на `/api/v1/cloud-ai/knowledge-bases/{id}/statistic`.  Можно
/// указать временной диапазон и интервал агрегации.
pub async fn get_knowledgebase_statistics(
    configuration: &configuration::Configuration,
    id: i32,
    start_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    end_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    interval: Option<f64>
) -> Result<models::GetKnowledgebaseStatistics200Response, Error<GetKnowledgebaseStatisticsError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_query_start_time = start_time;
    let p_query_end_time = end_time;
    let p_query_interval = interval;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases/{id}/statistic",
        configuration.base_path,
        id = p_path_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_start_time {
        req_builder = req_builder.query(&[("startTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_time {
        req_builder = req_builder.query(&[("endTime", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetKnowledgebaseStatistics200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetKnowledgebaseStatistics200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetKnowledgebaseStatisticsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список баз знаний, отправьте GET-запрос на
/// `/api/v1/cloud-ai/knowledge-bases`.  Тело ответа будет представлять собой
/// объект JSON с ключом `knowledgebases`.
#[deprecated]
pub async fn get_knowledgebases(
    configuration: &configuration::Configuration
) -> Result<models::GetKnowledgebases200Response, Error<GetKnowledgebasesError>> {
    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases",
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetKnowledgebases200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetKnowledgebases200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetKnowledgebasesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список баз знаний, отправьте GET-запрос на
/// `/api/v2/cloud-ai/knowledge-bases`.  Версия API v2 возвращает
/// оптимизированный ответ с количеством документов вместо полного списка
/// документов.  Тело ответа будет представлять собой объект JSON с ключом
/// `knowledgebases`.
pub async fn get_knowledgebases_v2(
    configuration: &configuration::Configuration
) -> Result<models::GetKnowledgebasesV2200Response, Error<GetKnowledgebasesV2Error>> {
    let uri_str = format!(
        "{}/api/v2/cloud-ai/knowledge-bases",
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetKnowledgebasesV2200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetKnowledgebasesV2200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetKnowledgebasesV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы привязать базу знаний к AI агенту, отправьте POST-запрос на
/// `/api/v1/cloud-ai/knowledge-bases/{id}/link/{agent_id}`.
pub async fn link_knowledgebase_to_agent(
    configuration: &configuration::Configuration,
    id: i32,
    agent_id: i32
) -> Result<(), Error<LinkKnowledgebaseToAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_agent_id = agent_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases/{id}/link/{agent_id}",
        configuration.base_path,
        id = p_path_id,
        agent_id = p_path_agent_id
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
        let content = resp.text().await?;
        let entity: Option<LinkKnowledgebaseToAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы переиндексировать документ в базе знаний, отправьте POST-запрос на
/// `/api/v1/cloud-ai/knowledge-bases/{id}/documents/{document_id}/reindex`.
pub async fn reindex_document(
    configuration: &configuration::Configuration,
    id: i32,
    document_id: i32
) -> Result<(), Error<ReindexDocumentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_document_id = document_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases/{id}/documents/{document_id}/reindex",
        configuration.base_path,
        id = p_path_id,
        document_id = p_path_document_id
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
        let content = resp.text().await?;
        let entity: Option<ReindexDocumentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы отвязать базу знаний от AI агента, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/knowledge-bases/{id}/link/{agent_id}`.
pub async fn unlink_knowledgebase_from_agent(
    configuration: &configuration::Configuration,
    id: i32,
    agent_id: i32
) -> Result<(), Error<UnlinkKnowledgebaseFromAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_agent_id = agent_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases/{id}/link/{agent_id}",
        configuration.base_path,
        id = p_path_id,
        agent_id = p_path_agent_id
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
        let entity: Option<UnlinkKnowledgebaseFromAgentError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы обновить базу знаний, отправьте PATCH-запрос на
/// `/api/v1/cloud-ai/knowledge-bases/{id}`.
pub async fn update_knowledgebase(
    configuration: &configuration::Configuration,
    id: i32,
    update_knowledgebase: models::UpdateKnowledgebase
) -> Result<models::CreateKnowledgebase201Response, Error<UpdateKnowledgebaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_update_knowledgebase = update_knowledgebase;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases/{id}",
        configuration.base_path,
        id = p_path_id
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
    req_builder = req_builder.json(&p_body_update_knowledgebase);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateKnowledgebase201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateKnowledgebase201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateKnowledgebaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы загрузить файлы в базу знаний, отправьте POST-запрос на
/// `/api/v1/cloud-ai/knowledge-bases/{id}/upload` с файлами в формате
/// multipart/form-data.  Поддерживаемые форматы: CSV, XML, Markdown (md), HTML,
/// TXT. JSON не поддерживается. Максимум 10 файлов, до 10 МБ каждый.
pub async fn upload_files_to_knowledgebase(
    configuration: &configuration::Configuration,
    id: i32,
    files: Vec<std::path::PathBuf>
) -> Result<models::UploadFilesToKnowledgebase200Response, Error<UploadFilesToKnowledgebaseError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_form_files = files;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/knowledge-bases/{id}/upload",
        configuration.base_path,
        id = p_path_id
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
    let mut multipart_form = reqwest::multipart::Form::new();
    for value in &p_form_files {
        let file = TokioFile::open(value).await?;
        let stream = FramedRead::new(file, BytesCodec::new());
        let file_name = value
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let file_part = reqwest::multipart::Part::stream(reqwest::Body::wrap_stream(stream))
            .file_name(file_name);
        multipart_form = multipart_form.part("files", file_part);
    }
    req_builder = req_builder.multipart(multipart_form);

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
                    "Received `text/plain` content type response that cannot be converted to `models::UploadFilesToKnowledgebase200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::UploadFilesToKnowledgebase200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UploadFilesToKnowledgebaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}
