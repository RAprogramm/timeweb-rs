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

/// struct for typed errors of method [`add_additional_token_package`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddAdditionalTokenPackageError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`change_agent_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeAgentModelError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_agent_entity_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAgentEntityLimitError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_agent_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAgentV2Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_agent_entity_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAgentEntityLimitError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`export_agent_chats_csv`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExportAgentChatsCsvError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_chat_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentChatListError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_chat_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentChatMessagesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_entity_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentEntityLimitError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_entity_limits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentEntityLimitsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_statistics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentStatisticsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_v3`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentV3Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agents_v3`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentsV3Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`sync_agent_entity_limits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyncAgentEntityLimitsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status410(),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    Status503(),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_agent_entity_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAgentEntityLimitError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// Чтобы докупить дополнительные пакеты токенов для AI агента, отправьте
/// POST-запрос на `/api/v1/cloud-ai/agents/{id}/add-additional-token-package`.
/// Метод относится к устаревшей схеме оплаты пакетами токенов и работает только
/// для агентов с пакетом (`token_package_id` в ответе агента). Для агентов с
/// поресурсной оплатой вернётся 400
/// `per-resource_agents_do_not_support_additional_token_packages`. Стоимость
/// (цена дополнительного пакета модели агента × `count`) сразу списывается с
/// баланса; при нехватке средств — 400 `insufficient_balance`, если подписка
/// агента истекла — 400 `subscription_expired`. После покупки заблокированный
/// агент разблокируется.  Для токенов дополнительных пользователей аккаунта
/// нужна роль `ai:write`.
#[deprecated]
pub async fn add_additional_token_package(
    configuration: &configuration::Configuration,
    id: i32,
    add_token_package: Option<models::AddTokenPackage>
) -> Result<(), Error<AddAdditionalTokenPackageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_add_token_package = add_token_package;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/add-additional-token-package",
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
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<AddAdditionalTokenPackageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы сменить основную модель AI агента, отправьте PATCH-запрос на
/// `/api/v1/cloud-ai/agents/{id}/model`.  Параметры модели из `settings.model`,
/// которые новая модель не поддерживает, отбрасываются. Смена модели доступна
/// только агентам с поресурсной оплатой (для агента с пакетом токенов — 400
/// `model_change_is_allowed_only_for_per-resource_account_and_agent`). Если
/// модель не найдена или недоступна — 404 `model_not_found`.  Для токенов
/// дополнительных пользователей аккаунта нужна роль `ai:write`.
pub async fn change_agent_model(
    configuration: &configuration::Configuration,
    id: i32,
    change_agent_model: models::ChangeAgentModel
) -> Result<models::CreateAgent201Response, Error<ChangeAgentModelError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_change_agent_model = change_agent_model;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/model",
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
    req_builder = req_builder.json(&p_body_change_agent_model);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgent201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgent201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<ChangeAgentModelError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы создать AI агента, отправьте POST-запрос на `/api/v1/cloud-ai/agents`,
/// задав необходимые атрибуты.  Метод устарел — используйте `POST
/// /api/v2/cloud-ai/agents`. Поле `token_package_id` обязательно, но не
/// учитывается: агент всегда создаётся с поресурсной оплатой (оплачиваются
/// фактические запросы к модели).  Для токенов дополнительных пользователей
/// аккаунта нужна роль `ai:write`.
#[deprecated]
pub async fn create_agent(
    configuration: &configuration::Configuration,
    create_agent: models::CreateAgent
) -> Result<models::CreateAgent201Response, Error<CreateAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_agent = create_agent;

    let uri_str = format!("{}/api/v1/cloud-ai/agents", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_agent);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgent201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgent201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<CreateAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы создать лимит AI агента, отправьте POST-запрос на
/// `/api/v1/cloud-ai/agents/{id}/entity-limits`.  Лимиты агента ограничивают
/// расход на его запросы за период (`hour`, `day`, `week`, `month`) — в токенах
/// (`limit`) или в деньгах (`money_limit`, приоритетнее). Лимит действует на
/// все модели агента (`model_id` = `null`) или на одну модель. При достижении
/// лимита запросы к агенту отклоняются до конца периода.  Нужно передать
/// `limit` или `money_limit` (иначе — 400 `entity_keys_limit_value_required`).
/// Если лимит с тем же периодом и моделью уже есть — 409
/// `entity_keys_limit_already_exists` (измените его или используйте
/// синхронизацию).  Для токенов дополнительных пользователей аккаунта нужна
/// роль `ai:write`.
pub async fn create_agent_entity_limit(
    configuration: &configuration::Configuration,
    id: i32,
    create_agent_entity_limit: models::CreateAgentEntityLimit
) -> Result<models::CreateAgentEntityLimit201Response, Error<CreateAgentEntityLimitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_create_agent_entity_limit = create_agent_entity_limit;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/entity-limits",
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
    req_builder = req_builder.json(&p_body_create_agent_entity_limit);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentEntityLimit201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentEntityLimit201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<CreateAgentEntityLimitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы создать AI агента, отправьте POST-запрос на `/api/v2/cloud-ai/agents`, задав необходимые атрибуты.  Агент создаётся с поресурсной оплатой: оплачиваются фактические запросы к модели. Если не переданы настройки виджета, подставляются значения по умолчанию. Дневной лимит расходов можно задать сразу полем `money_limit`, остальные лимиты — через лимиты агента (`/api/v1/cloud-ai/agents/{id}/entity-limits`).  Если модель не найдена или недоступна, вернётся 404 `model_not_found`; если модель не поддерживается — 400 `model_not_supported`; недопустимые параметры модели в `settings.model` — 400.  Вызывать созданного агента из своих приложений можно через OpenAI-совместимое API по его `access_id` — см. [документацию](https://agent.timeweb.cloud/docs).  Для токенов дополнительных пользователей аккаунта нужна роль `ai:write`.
pub async fn create_agent_v2(
    configuration: &configuration::Configuration,
    create_agent_v2: models::CreateAgentV2
) -> Result<models::CreateAgent201Response, Error<CreateAgentV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_agent_v2 = create_agent_v2;

    let uri_str = format!("{}/api/v2/cloud-ai/agents", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_agent_v2);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgent201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgent201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<CreateAgentV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы удалить AI агента, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/agents/{id}`.  Вместе с агентом отключаются его базы
/// знаний, навыки и интеграции (Telegram, MAX, Jivo, Crisp). В теле запроса
/// можно необязательно указать причину удаления.  Для токенов дополнительных
/// пользователей аккаунта нужна роль `ai:write`.
pub async fn delete_agent(
    configuration: &configuration::Configuration,
    id: i32,
    delete_agent: Option<models::DeleteAgent>
) -> Result<(), Error<DeleteAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_delete_agent = delete_agent;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}",
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
    req_builder = req_builder.json(&p_body_delete_agent);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<DeleteAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы удалить лимит AI агента, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/agents/{id}/entity-limits/{limit_id}`.  Для токенов
/// дополнительных пользователей аккаунта нужна роль `ai:write`.
pub async fn delete_agent_entity_limit(
    configuration: &configuration::Configuration,
    id: i32,
    limit_id: &str
) -> Result<(), Error<DeleteAgentEntityLimitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_limit_id = limit_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/entity-limits/{limit_id}",
        configuration.base_path,
        id = p_path_id,
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
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<DeleteAgentEntityLimitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы выгрузить чаты AI агента в формате CSV, отправьте GET-запрос на
/// `/api/v1/cloud-ai/agents/{id}/chats/docs`.  CSV возвращается строкой в поле
/// `chats_csv.csv` тела JSON. `limit` и `offset` применяются к сообщениям, а не
/// к чатам. Если передан `from`, выгружаются сообщения от `from` до `to` (по
/// умолчанию — до текущего момента). Чаты внешнего чата не выгружаются.  Для
/// токенов дополнительных пользователей аккаунта нужна роль `ai:read`.
pub async fn export_agent_chats_csv(
    configuration: &configuration::Configuration,
    id: i32,
    offset: Option<i32>,
    limit: Option<i32>,
    order: Option<&str>,
    from: Option<chrono::DateTime<chrono::FixedOffset>>,
    to: Option<chrono::DateTime<chrono::FixedOffset>>,
    locale: Option<&str>
) -> Result<models::ExportAgentChatsCsv200Response, Error<ExportAgentChatsCsvError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_query_offset = offset;
    let p_query_limit = limit;
    let p_query_order = order;
    let p_query_from = from;
    let p_query_to = to;
    let p_query_locale = locale;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/chats/docs",
        configuration.base_path,
        id = p_path_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_order {
        req_builder = req_builder.query(&[("order", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_from {
        req_builder = req_builder.query(&[("from", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_to {
        req_builder = req_builder.query(&[("to", &param_value.to_string())]);
    }
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
                    "Received `text/plain` content type response that cannot be converted to `models::ExportAgentChatsCsv200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ExportAgentChatsCsv200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<ExportAgentChatsCsvError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить информацию об AI агенте, отправьте GET-запрос на
/// `/api/v1/cloud-ai/agents/{id}`.  Для токенов дополнительных пользователей
/// аккаунта нужна роль `ai:read`.
pub async fn get_agent(
    configuration: &configuration::Configuration,
    id: i32
) -> Result<models::CreateAgent201Response, Error<GetAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}",
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgent201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgent201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить список чатов (диалогов) AI агента, отправьте GET-запрос на
/// `/api/v1/cloud-ai/agents/{id}/chat-list`.  Для каждого чата возвращаются
/// первое сообщение пользователя, количество оценок и среднее время ответа.
/// Чаты внешнего чата сюда не входят. Фильтр по периоду применяется, только
/// если переданы оба параметра `from` и `to`. В `meta.metrics` — сводка по
/// чатам для фильтров.  Для токенов дополнительных пользователей аккаунта нужна
/// роль `ai:read`.
pub async fn get_agent_chat_list(
    configuration: &configuration::Configuration,
    id: i32,
    offset: Option<i32>,
    limit: Option<i32>,
    order: Option<&str>,
    from: Option<chrono::DateTime<chrono::FixedOffset>>,
    to: Option<chrono::DateTime<chrono::FixedOffset>>,
    reaction_type: Option<&str>,
    response_time_ranges: Option<Vec<String>>
) -> Result<models::GetAgentChatList200Response, Error<GetAgentChatListError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_query_offset = offset;
    let p_query_limit = limit;
    let p_query_order = order;
    let p_query_from = from;
    let p_query_to = to;
    let p_query_reaction_type = reaction_type;
    let p_query_response_time_ranges = response_time_ranges;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/chat-list",
        configuration.base_path,
        id = p_path_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_order {
        req_builder = req_builder.query(&[("order", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_from {
        req_builder = req_builder.query(&[("from", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_to {
        req_builder = req_builder.query(&[("to", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_reaction_type {
        req_builder = req_builder.query(&[("reaction_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_response_time_ranges {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .into_iter()
                    .map(|p| ("response_time_ranges".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>()
            ),
            _ => req_builder.query(&[(
                "response_time_ranges",
                &param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string()
            )])
        };
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentChatList200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentChatList200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentChatListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить сообщения чата AI агента, отправьте GET-запрос на
/// `/api/v1/cloud-ai/agents/{id}/chat-list/{chat_id}/messages`.  Возвращаются
/// сообщения пользователя и ответы агента. Фильтр по периоду применяется,
/// только если переданы оба параметра `from` и `to`.  Для токенов
/// дополнительных пользователей аккаунта нужна роль `ai:read`.
pub async fn get_agent_chat_messages(
    configuration: &configuration::Configuration,
    id: i32,
    chat_id: &str,
    offset: Option<i32>,
    limit: Option<i32>,
    order: Option<&str>,
    from: Option<chrono::DateTime<chrono::FixedOffset>>,
    to: Option<chrono::DateTime<chrono::FixedOffset>>
) -> Result<models::GetAgentChatMessages200Response, Error<GetAgentChatMessagesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_chat_id = chat_id;
    let p_query_offset = offset;
    let p_query_limit = limit;
    let p_query_order = order;
    let p_query_from = from;
    let p_query_to = to;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/chat-list/{chat_id}/messages",
        configuration.base_path,
        id = p_path_id,
        chat_id = crate::apis::urlencode(p_path_chat_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_order {
        req_builder = req_builder.query(&[("order", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_from {
        req_builder = req_builder.query(&[("from", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_to {
        req_builder = req_builder.query(&[("to", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentChatMessages200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentChatMessages200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentChatMessagesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить лимит AI агента, отправьте GET-запрос на
/// `/api/v1/cloud-ai/agents/{id}/entity-limits/{limit_id}`.  Для токенов
/// дополнительных пользователей аккаунта нужна роль `ai:read`.
pub async fn get_agent_entity_limit(
    configuration: &configuration::Configuration,
    id: i32,
    limit_id: &str
) -> Result<models::CreateAgentEntityLimit201Response, Error<GetAgentEntityLimitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_limit_id = limit_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/entity-limits/{limit_id}",
        configuration.base_path,
        id = p_path_id,
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentEntityLimit201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentEntityLimit201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentEntityLimitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить лимиты AI агента, отправьте GET-запрос на
/// `/api/v1/cloud-ai/agents/{id}/entity-limits`.  Лимиты агента ограничивают
/// расход на его запросы за период (`hour`, `day`, `week`, `month`) — в токенах
/// (`limit`) или в деньгах (`money_limit`, приоритетнее). Лимит действует на
/// все модели агента (`model_id` = `null`) или на одну модель. При достижении
/// лимита запросы к агенту отклоняются до конца периода.  Для токенов
/// дополнительных пользователей аккаунта нужна роль `ai:read`.
pub async fn get_agent_entity_limits(
    configuration: &configuration::Configuration,
    id: i32,
    limit_type: Option<&str>
) -> Result<models::GetAgentEntityLimits200Response, Error<GetAgentEntityLimitsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_query_limit_type = limit_type;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/entity-limits",
        configuration.base_path,
        id = p_path_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentEntityLimits200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentEntityLimits200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentEntityLimitsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить статистику использования токенов AI агента, отправьте
/// GET-запрос на `/api/v1/cloud-ai/agents/{id}/statistic`.  Статистика разбита
/// на интервалы длиной `interval` минут: для каждого интервала возвращается по
/// точке на каждую модель, к которой были запросы за период, а для интервалов
/// без запросов — одна нулевая точка без `model_name`. По умолчанию
/// возвращаются последние 24 часа.  Для токенов дополнительных пользователей
/// аккаунта нужна роль `ai:read`.
pub async fn get_agent_statistics(
    configuration: &configuration::Configuration,
    id: i32,
    start_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    end_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    interval: Option<i32>
) -> Result<models::GetAgentStatistics200Response, Error<GetAgentStatisticsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_query_start_time = start_time;
    let p_query_end_time = end_time;
    let p_query_interval = interval;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/statistic",
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentStatistics200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentStatistics200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentStatisticsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить информацию об AI агенте, отправьте GET-запрос на
/// `/api/v3/cloud-ai/agents/{id}`.  Отличается от
/// `/api/v1/cloud-ai/agents/{id}` тем, что вместо `knowledge_bases_ids`
/// возвращает `knowledge_bases` — подключённые базы знаний с режимом поиска.
/// Для токенов дополнительных пользователей аккаунта нужна роль `ai:read`.
pub async fn get_agent_v3(
    configuration: &configuration::Configuration,
    id: i32
) -> Result<models::GetAgentV3200Response, Error<GetAgentV3Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/api/v3/cloud-ai/agents/{id}",
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentV3200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentV3200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentV3Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить список AI агентов, отправьте GET-запрос на
/// `/api/v1/cloud-ai/agents`.  Возвращаются все агенты аккаунта (без
/// пагинации), включая приостановленных и заблокированных. Тело ответа будет
/// представлять собой объект JSON с ключами `agents` и `meta`. Подключённые
/// базы знаний вместе с режимом поиска возвращает метод
/// `/api/v3/cloud-ai/agents`.  Для токенов дополнительных пользователей
/// аккаунта нужна роль `ai:read`.
pub async fn get_agents(
    configuration: &configuration::Configuration
) -> Result<models::GetAgents200Response, Error<GetAgentsError>> {
    let uri_str = format!("{}/api/v1/cloud-ai/agents", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgents200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgents200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить список AI агентов, отправьте GET-запрос на
/// `/api/v3/cloud-ai/agents`.  Отличается от `/api/v1/cloud-ai/agents` тем, что
/// вместо `knowledge_bases_ids` возвращает `knowledge_bases` — подключённые
/// базы знаний с режимом поиска.  Для токенов дополнительных пользователей
/// аккаунта нужна роль `ai:read`.
pub async fn get_agents_v3(
    configuration: &configuration::Configuration
) -> Result<models::GetAgentsV3200Response, Error<GetAgentsV3Error>> {
    let uri_str = format!("{}/api/v3/cloud-ai/agents", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentsV3200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentsV3200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentsV3Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы задать полный набор лимитов AI агента за период, отправьте PUT-запрос
/// на `/api/v1/cloud-ai/agents/{id}/entity-limits/sync`.  Лимиты из запроса
/// создаются или обновляются (сопоставление по `model_id`), а лимиты этого
/// периода, которых нет в запросе, удаляются. Лимиты других периодов не
/// затрагиваются. В ответе — актуальные лимиты периода. Если лимиты
/// одновременно изменил другой запрос, вернётся 409
/// `entity_keys_limits_sync_conflict` — повторите запрос.  Для токенов
/// дополнительных пользователей аккаунта нужна роль `ai:write`.
pub async fn sync_agent_entity_limits(
    configuration: &configuration::Configuration,
    id: i32,
    sync_agent_entity_limits: models::SyncAgentEntityLimits
) -> Result<models::GetAgentEntityLimits200Response, Error<SyncAgentEntityLimitsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_sync_agent_entity_limits = sync_agent_entity_limits;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/entity-limits/sync",
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
    req_builder = req_builder.json(&p_body_sync_agent_entity_limits);

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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentEntityLimits200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentEntityLimits200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<SyncAgentEntityLimitsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы обновить AI агента, отправьте PATCH-запрос на
/// `/api/v1/cloud-ai/agents/{id}`, передав изменяемые поля.  Особенности: -
/// `settings` заменяются целиком (кроме `web_search_allowed_sites`,
/// `additional_models` и `memory_enabled`, которые сохраняются, если не
/// переданы) — передавайте полный объект настроек, включая `widget`; - основную
/// модель меняет отдельный метод `PATCH /api/v1/cloud-ai/agents/{id}/model`; -
/// поле `token_limit` больше не поддерживается: запрос с ним отклоняется с
/// кодом 410 — лимиты задаются через
/// `/api/v1/cloud-ai/agents/{id}/entity-limits`; - ошибки привязки
/// `ai_gateway_key_id`: 400 `ai_gateway_key_not_found`,
/// `agent_is_already_migrated_to_another_ai_gateway_key`,
/// `agent_is_not_eligible_for_ai_gateway_migration`; 503 — проверку временно не
/// удалось выполнить; - изменить статус заблокированного или удалённого агента
/// нельзя (400 `agent_can't_change_status`).  Для токенов дополнительных
/// пользователей аккаунта нужна роль `ai:write`.
pub async fn update_agent(
    configuration: &configuration::Configuration,
    id: i32,
    update_agent: models::UpdateAgent
) -> Result<models::CreateAgent201Response, Error<UpdateAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_update_agent = update_agent;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}",
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
    req_builder = req_builder.json(&p_body_update_agent);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgent201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgent201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<UpdateAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы изменить значение лимита AI агента, отправьте PATCH-запрос на
/// `/api/v1/cloud-ai/agents/{id}/entity-limits/{limit_id}`.  Непереданные поля
/// не меняются. Снять денежный лимит (`money_limit`: `null`) можно только
/// вместе с передачей `limit`.  Для токенов дополнительных пользователей
/// аккаунта нужна роль `ai:write`.
pub async fn update_agent_entity_limit(
    configuration: &configuration::Configuration,
    id: i32,
    limit_id: &str,
    update_agent_entity_limit: models::UpdateAgentEntityLimit
) -> Result<models::CreateAgentEntityLimit201Response, Error<UpdateAgentEntityLimitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_limit_id = limit_id;
    let p_body_update_agent_entity_limit = update_agent_entity_limit;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{id}/entity-limits/{limit_id}",
        configuration.base_path,
        id = p_path_id,
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
    req_builder = req_builder.json(&p_body_update_agent_entity_limit);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentEntityLimit201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentEntityLimit201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<UpdateAgentEntityLimitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}
