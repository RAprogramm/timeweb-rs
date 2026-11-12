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

/// struct for typed errors of method [`connect_agent_tool_to_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConnectAgentToolToAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_agent_tool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAgentToolError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_agent_tool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAgentToolError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_mcp_search_session`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMcpSearchSessionError {
    Status401(models::GetFinances401Response),
    Status429(),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_mcp_session`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMcpSessionError {
    Status401(models::GetFinances401Response),
    Status429(),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`disconnect_agent_tool_from_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisconnectAgentToolFromAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_tool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentToolError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_tool_o_auth2_authorize_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentToolOAuth2AuthorizeUrlError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_tool_template_prices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentToolTemplatePricesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_tool_templates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentToolTemplatesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_tools`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentToolsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_tools_by_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentToolsByAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`install_agent_tool_from_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InstallAgentToolFromTemplateError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`open_mcp_search_stream`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenMcpSearchStreamError {
    Status401(models::GetFinances401Response),
    Status406(models::JsonRpcErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`open_mcp_stream`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenMcpStreamError {
    Status401(models::GetFinances401Response),
    Status406(models::JsonRpcErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`refresh_agent_tool_methods`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RefreshAgentToolMethodsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`send_mcp_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendMcpMessageError {
    Status400(models::JsonRpcErrorResponse),
    Status401(models::GetFinances401Response),
    Status406(models::JsonRpcErrorResponse),
    Status415(models::JsonRpcErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`send_mcp_search_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendMcpSearchMessageError {
    Status400(models::JsonRpcErrorResponse),
    Status401(models::GetFinances401Response),
    Status406(models::JsonRpcErrorResponse),
    Status415(models::JsonRpcErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_agent_tool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAgentToolError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_agent_tool_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAgentToolConnectionError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// Чтобы подключить MCP-инструмент к агенту, отправьте POST-запрос на
/// `/api/v1/cloud-ai/agents/{agent_id}/tools`.  В `allowed_methods` перечислите
/// методы, которые агенту разрешено вызывать. Подходят только имена из
/// `methods[].name` инструмента, иначе вернётся 400 с `error_code`
/// `invalid_methods_provided`. Повторно подключить уже подключённый инструмент
/// нельзя. Чтобы изменить список методов, используйте PUT-запрос.
pub async fn connect_agent_tool_to_agent(
    configuration: &configuration::Configuration,
    agent_id: i32,
    connect_agent_tool: models::ConnectAgentTool
) -> Result<(), Error<ConnectAgentToolToAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_id = agent_id;
    let p_body_connect_agent_tool = connect_agent_tool;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{agent_id}/tools",
        configuration.base_path,
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
    req_builder = req_builder.json(&p_body_connect_agent_tool);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ConnectAgentToolToAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы подключить собственный MCP-сервер, отправьте POST-запрос на
/// `/api/v1/cloud-ai/tools`.  Платформа сразу подключается к серверу и
/// сохраняет список его методов. Если сервер недоступен или отвечает ошибкой,
/// инструмент не создаётся: вернётся 400 с `error_code`
/// `invalid_methods_provided`. Адреса частных сетей и localhost отклоняются с
/// ошибкой 400.  Для `auth_data.type = oauth2` подключение к серверу при
/// создании не проверяется. Инструмент создаётся в статусе `pending_auth` с
/// пустым списком методов, а методы загружаются после авторизации пользователя:
/// см. «Получение ссылки на OAuth2-авторизацию инструмента». Если не удалось
/// определить адреса авторизации или зарегистрировать клиента, вернётся 401 с
/// `error_code` `tool_authentication_failed`.  Чтобы агент мог пользоваться
/// инструментом, подключите его к агенту.
pub async fn create_agent_tool(
    configuration: &configuration::Configuration,
    create_tool: models::CreateTool
) -> Result<models::CreateAgentTool201Response, Error<CreateAgentToolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_tool = create_tool;

    let uri_str = format!("{}/api/v1/cloud-ai/tools", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_tool);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentTool201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentTool201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAgentToolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить MCP-инструмент, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/tools/{id}`.  Инструмент автоматически отключается от всех
/// агентов.
pub async fn delete_agent_tool(
    configuration: &configuration::Configuration,
    id: i32
) -> Result<(), Error<DeleteAgentToolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/tools/{id}",
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
        let entity: Option<DeleteAgentToolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы завершить сессию сервера MCP `timeweb-cloud-mcp-search`, отправьте
/// DELETE-запрос на `/api/v1/mcp/search`.  Сервер не хранит сессий (не выдаёт
/// `Mcp-Session-Id`), поэтому завершать нечего: ответ всегда 200 без тела.
pub async fn delete_mcp_search_session(
    configuration: &configuration::Configuration
) -> Result<(), Error<DeleteMcpSearchSessionError>> {
    let uri_str = format!("{}/api/v1/mcp/search", configuration.base_path);
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
        let entity: Option<DeleteMcpSearchSessionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы завершить сессию сервера MCP `timeweb-cloud-mcp`, отправьте
/// DELETE-запрос на `/api/v1/mcp`.  Сервер не хранит сессий (не выдаёт
/// `Mcp-Session-Id`), поэтому завершать нечего: ответ всегда 200 без тела.
pub async fn delete_mcp_session(
    configuration: &configuration::Configuration
) -> Result<(), Error<DeleteMcpSessionError>> {
    let uri_str = format!("{}/api/v1/mcp", configuration.base_path);
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
        let entity: Option<DeleteMcpSessionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы отключить MCP-инструмент от агента, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/agents/{agent_id}/tools/{tool_id}`.  Сам инструмент
/// остаётся в аккаунте и у других агентов.
pub async fn disconnect_agent_tool_from_agent(
    configuration: &configuration::Configuration,
    agent_id: i32,
    tool_id: i32
) -> Result<(), Error<DisconnectAgentToolFromAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_id = agent_id;
    let p_path_tool_id = tool_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{agent_id}/tools/{tool_id}",
        configuration.base_path,
        agent_id = p_path_agent_id,
        tool_id = p_path_tool_id
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
        let entity: Option<DisconnectAgentToolFromAgentError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить MCP-инструмент, отправьте GET-запрос на
/// `/api/v1/cloud-ai/tools/{id}`.  По полю `status` можно отследить завершение
/// OAuth2-авторизации: после неё статус меняется с `pending_auth` на `active`.
pub async fn get_agent_tool(
    configuration: &configuration::Configuration,
    id: i32
) -> Result<models::CreateAgentTool201Response, Error<GetAgentToolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/tools/{id}",
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentTool201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentTool201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAgentToolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить ссылку на OAuth2-авторизацию MCP-инструмента, отправьте GET-запрос на `/api/v1/cloud-ai/tools/oauth2/authorize-url/{tool_id}`.  Откройте `authorize_url` в браузере пользователя. Ссылка действует 30 минут. После входа и согласия провайдер вернёт пользователя на адрес возврата платформы `https://api.timeweb.cloud/api/v1/cloud-ai/oauth2/callback`. Платформа обменяет код на токены, переведёт инструмент в статус `active` и загрузит список методов.  Чтобы узнать результат, запрашивайте инструмент, пока его статус не сменится с `pending_auth` на `active`. Этим же запросом можно повторно авторизовать уже активный инструмент.
pub async fn get_agent_tool_o_auth2_authorize_url(
    configuration: &configuration::Configuration,
    tool_id: i32,
    redirect_url: Option<&str>
) -> Result<
    models::GetAgentToolOAuth2AuthorizeUrl200Response,
    Error<GetAgentToolOAuth2AuthorizeUrlError>
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_tool_id = tool_id;
    let p_query_redirect_url = redirect_url;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/tools/oauth2/authorize-url/{tool_id}",
        configuration.base_path,
        tool_id = p_path_tool_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_redirect_url {
        req_builder = req_builder.query(&[("redirect_url", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentToolOAuth2AuthorizeUrl200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentToolOAuth2AuthorizeUrl200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAgentToolOAuth2AuthorizeUrlError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить цены системных инструментов платформы, например веб-поиска,
/// отправьте GET-запрос на `/api/v1/cloud-ai/tools/templates/prices`.
pub async fn get_agent_tool_template_prices(
    configuration: &configuration::Configuration,
    locale: Option<&str>
) -> Result<models::GetAgentToolTemplatePrices200Response, Error<GetAgentToolTemplatePricesError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_locale = locale;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/tools/templates/prices",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_locale {
        req_builder = req_builder.query(&[("locale", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentToolTemplatePrices200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentToolTemplatePrices200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAgentToolTemplatePricesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить готовые интеграции маркетплейса, отправьте GET-запрос на
/// `/api/v1/cloud-ai/tools/templates/marketplace`.  Сначала идут самые
/// популярные шаблоны. Поле `config_schema` описывает, что нужно передать в
/// `config_values` при установке.
pub async fn get_agent_tool_templates(
    configuration: &configuration::Configuration
) -> Result<models::GetAgentToolTemplates200Response, Error<GetAgentToolTemplatesError>> {
    let uri_str = format!(
        "{}/api/v1/cloud-ai/tools/templates/marketplace",
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentToolTemplates200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentToolTemplates200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAgentToolTemplatesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список MCP-инструментов аккаунта, отправьте GET-запрос на
/// `/api/v1/cloud-ai/tools`.  В списке есть и собственные MCP-серверы, и
/// инструменты, установленные из маркетплейса. Сначала идут новые. Удалённые
/// инструменты не возвращаются. Секреты в `auth_data` заменены на `********`.
pub async fn get_agent_tools(
    configuration: &configuration::Configuration
) -> Result<models::GetAgentTools200Response, Error<GetAgentToolsError>> {
    let uri_str = format!("{}/api/v1/cloud-ai/tools", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentTools200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentTools200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAgentToolsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить MCP-инструменты, подключённые к агенту, отправьте GET-запрос
/// на `/api/v1/cloud-ai/agents/{agent_id}/tools`.  В ответе — инструменты
/// целиком, в том же виде, что и в списке инструментов аккаунта. Списка
/// разрешённых агенту методов в ответе нет.
pub async fn get_agent_tools_by_agent(
    configuration: &configuration::Configuration,
    agent_id: i32
) -> Result<models::GetAgentToolsByAgent200Response, Error<GetAgentToolsByAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_id = agent_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{agent_id}/tools",
        configuration.base_path,
        agent_id = p_path_agent_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentToolsByAgent200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentToolsByAgent200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAgentToolsByAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы установить MCP-инструмент из шаблона маркетплейса, отправьте
/// POST-запрос на `/api/v1/cloud-ai/tools/install`.  Значения полей из
/// `config_schema` шаблона передайте в `config_values`. Платформа соберёт из
/// них заголовки авторизации, подключится к MCP-серверу и сохранит список
/// методов. Если у шаблона есть `config_schema`, поле `auth_data` не
/// используется. Если подключиться к серверу не удалось, вернётся 400 с
/// `error_code` `invalid_methods_provided`.  Один шаблон можно установить
/// несколько раз, например с разными ключами.
pub async fn install_agent_tool_from_template(
    configuration: &configuration::Configuration,
    install_tool: models::InstallTool
) -> Result<models::CreateAgentTool201Response, Error<InstallAgentToolFromTemplateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_install_tool = install_tool;

    let uri_str = format!("{}/api/v1/cloud-ai/tools/install", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_install_tool);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentTool201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentTool201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<InstallAgentToolFromTemplateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы открыть поток сообщений, которые сервер MCP `timeweb-cloud-mcp-search`
/// отправляет по своей инициативе, отправьте GET-запрос на
/// `/api/v1/mcp/search`.  Сервер не отправляет сообщений по своей инициативе,
/// поэтому поток не содержит событий и остаётся открытым, пока его не закроет
/// шлюз. Через шлюз заголовки ответа на этот запрос могут долго не приходить.
/// Открывать поток не нужно: работайте с сервером только POST-запросами — если
/// клиент позволяет выбрать режим, отключите в нём GET-поток.
pub async fn open_mcp_search_stream(
    configuration: &configuration::Configuration,
    accept: &str,
    mcp_protocol_version: Option<&str>
) -> Result<String, Error<OpenMcpSearchStreamError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_accept = accept;
    let p_header_mcp_protocol_version = mcp_protocol_version;

    let uri_str = format!("{}/api/v1/mcp/search", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Accept", p_header_accept.to_string());
    if let Some(param_value) = p_header_mcp_protocol_version {
        req_builder = req_builder.header("MCP-Protocol-Version", param_value.to_string());
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
        let content = resp.text().await?;
        let entity: Option<OpenMcpSearchStreamError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы открыть поток сообщений, которые сервер MCP `timeweb-cloud-mcp`
/// отправляет по своей инициативе, отправьте GET-запрос на `/api/v1/mcp`.
/// Сервер не отправляет сообщений по своей инициативе, поэтому поток не
/// содержит событий и остаётся открытым, пока его не закроет шлюз. Через шлюз
/// заголовки ответа на этот запрос могут долго не приходить. Открывать поток не
/// нужно: работайте с сервером только POST-запросами — если клиент позволяет
/// выбрать режим, отключите в нём GET-поток.
pub async fn open_mcp_stream(
    configuration: &configuration::Configuration,
    accept: &str,
    mcp_protocol_version: Option<&str>
) -> Result<String, Error<OpenMcpStreamError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_accept = accept;
    let p_header_mcp_protocol_version = mcp_protocol_version;

    let uri_str = format!("{}/api/v1/mcp", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Accept", p_header_accept.to_string());
    if let Some(param_value) = p_header_mcp_protocol_version {
        req_builder = req_builder.header("MCP-Protocol-Version", param_value.to_string());
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
        let content = resp.text().await?;
        let entity: Option<OpenMcpStreamError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы заново получить список методов с MCP-сервера, отправьте POST-запрос на
/// `/api/v1/cloud-ai/tools/{id}/refresh-methods`.  Список `methods` заменяется
/// полученным с сервера, а методы, которых больше нет, убираются из разрешённых
/// у всех агентов. Если обновление прошло успешно, инструмент переходит в
/// статус `active`. Если сервер недоступен, инструмент переходит в статус
/// `error` с причиной в `status_info`, а запрос возвращает 400 с `error_code`
/// `invalid_methods_provided`.
pub async fn refresh_agent_tool_methods(
    configuration: &configuration::Configuration,
    id: i32
) -> Result<models::CreateAgentTool201Response, Error<RefreshAgentToolMethodsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/tools/{id}/refresh-methods",
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentTool201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentTool201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RefreshAgentToolMethodsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы отправить запрос или уведомление серверу MCP `timeweb-cloud-mcp`,
/// отправьте POST-запрос на `/api/v1/mcp` с сообщением JSON-RPC 2.0 в теле.
/// Через этот метод идут все запросы MCP: `initialize`, `tools/list` (все
/// инструменты сразу), `tools/call`, `resources/list`, `resources/read`,
/// `ping`. Ответ на запрос приходит потоком `text/event-stream` с одним
/// событием `message`, уведомление принимается с кодом 202 без тела. Изменяющие
/// инструменты выполняются в два вызова с `confirm_token` — см. описание
/// раздела.  Заголовки `x-tool-access` и `x-allowed-tools` здесь не действуют —
/// используйте `/api/v1/mcp/search`.
pub async fn send_mcp_message(
    configuration: &configuration::Configuration,
    accept: &str,
    send_mcp_message_request: models::SendMcpMessageRequest,
    mcp_protocol_version: Option<&str>,
    x_mcp_confirmation_ui: Option<&str>
) -> Result<models::SendMcpMessage200Response, Error<SendMcpMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_accept = accept;
    let p_body_send_mcp_message_request = send_mcp_message_request;
    let p_header_mcp_protocol_version = mcp_protocol_version;
    let p_header_x_mcp_confirmation_ui = x_mcp_confirmation_ui;

    let uri_str = format!("{}/api/v1/mcp", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Accept", p_header_accept.to_string());
    if let Some(param_value) = p_header_mcp_protocol_version {
        req_builder = req_builder.header("MCP-Protocol-Version", param_value.to_string());
    }
    if let Some(param_value) = p_header_x_mcp_confirmation_ui {
        req_builder = req_builder.header("x-mcp-confirmation-ui", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_send_mcp_message_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::SendMcpMessage200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SendMcpMessage200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SendMcpMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы отправить запрос или уведомление серверу MCP
/// `timeweb-cloud-mcp-search`, отправьте POST-запрос на `/api/v1/mcp/search` с
/// сообщением JSON-RPC 2.0 в теле.  `tools/list` возвращает четыре инструмента:
/// `search_tools`, `get_tool_definition`, `execute_tool` и
/// `request_user_confirmation`. Все операции облака находятся через
/// `search_tools` и выполняются через `execute_tool`; изменяющие операции — в
/// два вызова `execute_tool` с `confirm_token` внутри `arguments`.  Доступ
/// модели можно сузить заголовками `x-tool-access` и `x-allowed-tools`.
/// Недопустимое значение `x-tool-access` — ответ 400 с ошибкой `-32600`.
pub async fn send_mcp_search_message(
    configuration: &configuration::Configuration,
    accept: &str,
    send_mcp_message_request: models::SendMcpMessageRequest,
    mcp_protocol_version: Option<&str>,
    x_tool_access: Option<&str>,
    x_allowed_tools: Option<&str>,
    x_mcp_confirmation_ui: Option<&str>
) -> Result<models::SendMcpMessage200Response, Error<SendMcpSearchMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_accept = accept;
    let p_body_send_mcp_message_request = send_mcp_message_request;
    let p_header_mcp_protocol_version = mcp_protocol_version;
    let p_header_x_tool_access = x_tool_access;
    let p_header_x_allowed_tools = x_allowed_tools;
    let p_header_x_mcp_confirmation_ui = x_mcp_confirmation_ui;

    let uri_str = format!("{}/api/v1/mcp/search", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Accept", p_header_accept.to_string());
    if let Some(param_value) = p_header_mcp_protocol_version {
        req_builder = req_builder.header("MCP-Protocol-Version", param_value.to_string());
    }
    if let Some(param_value) = p_header_x_tool_access {
        req_builder = req_builder.header("x-tool-access", param_value.to_string());
    }
    if let Some(param_value) = p_header_x_allowed_tools {
        req_builder = req_builder.header("x-allowed-tools", param_value.to_string());
    }
    if let Some(param_value) = p_header_x_mcp_confirmation_ui {
        req_builder = req_builder.header("x-mcp-confirmation-ui", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_send_mcp_message_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::SendMcpMessage200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SendMcpMessage200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SendMcpSearchMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы изменить MCP-инструмент, отправьте PUT-запрос на
/// `/api/v1/cloud-ai/tools/{id}`, передав только изменяемые поля.  Если
/// передать в `auth_data` секрет `********` из ответа API, сохранённый секрет
/// не изменится. Изменение адреса или авторизации не обновляет список методов:
/// после него вызовите обновление методов. Статус `inactive` выключает
/// инструмент у всех агентов, к которым он подключён.
pub async fn update_agent_tool(
    configuration: &configuration::Configuration,
    id: i32,
    update_tool: models::UpdateTool
) -> Result<models::CreateAgentTool201Response, Error<UpdateAgentToolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_update_tool = update_tool;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/tools/{id}",
        configuration.base_path,
        id = p_path_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_tool);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentTool201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentTool201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateAgentToolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы изменить список методов, которые агенту разрешено вызывать, отправьте
/// PUT-запрос на `/api/v1/cloud-ai/agents/{agent_id}/tools/{tool_id}`.  Список
/// заменяется целиком. Подходят только имена из `methods[].name` инструмента,
/// иначе вернётся 400 с `error_code` `invalid_methods_provided`. Если
/// инструмент не подключён к агенту, вернётся 404 с `error_code`
/// `agent-tool_link_not_found`.
pub async fn update_agent_tool_connection(
    configuration: &configuration::Configuration,
    agent_id: i32,
    tool_id: i32,
    update_agent_tool: models::UpdateAgentTool
) -> Result<(), Error<UpdateAgentToolConnectionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_id = agent_id;
    let p_path_tool_id = tool_id;
    let p_body_update_agent_tool = update_agent_tool;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{agent_id}/tools/{tool_id}",
        configuration.base_path,
        agent_id = p_path_agent_id,
        tool_id = p_path_tool_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_agent_tool);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateAgentToolConnectionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}
