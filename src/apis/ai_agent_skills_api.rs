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

/// struct for typed errors of method [`apply_agent_skill_catalog_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplyAgentSkillCatalogUpdateError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`connect_agent_skills_to_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConnectAgentSkillsToAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_agent_skill`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAgentSkillError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_agent_skill`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAgentSkillError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`disconnect_agent_skill_from_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisconnectAgentSkillFromAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`duplicate_agent_skill`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DuplicateAgentSkillError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_skill`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentSkillError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_skill_catalog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentSkillCatalogError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_skill_library`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentSkillLibraryError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_skill_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentSkillVersionError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_skill_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentSkillVersionsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_agent_skills_for_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentSkillsForAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`install_agent_skill_from_catalog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InstallAgentSkillFromCatalogError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`install_and_connect_agent_skill`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InstallAndConnectAgentSkillError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`rollback_agent_skill_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RollbackAgentSkillVersionError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`set_agent_skill_for_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetAgentSkillForAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`sync_agent_skill_agents_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyncAgentSkillAgentsVersionsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`sync_agent_skill_versions_for_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyncAgentSkillVersionsForAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_agent_skill`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAgentSkillError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_agent_skill_version_for_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAgentSkillVersionForAgentError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// Чтобы применить к установленному навыку новую версию из каталога, отправьте
/// POST-запрос на `/api/v1/cloud-ai/skills/{skill_id}/apply-catalog-update`.
/// Текст «когда применять» и инструкция заменяются каталожными, при этом
/// создаётся новая версия: прежний текст, в том числе ваши правки, остаётся в
/// истории. Название навыка не меняется. Если обновления нет, ничего не
/// меняется (`meta.version_created` — `false`).  Для навыка, установленного не
/// из каталога, вернётся 409 с `error_code`
/// `skill_is_not_installed_from_catalog`.
pub async fn apply_agent_skill_catalog_update(
    configuration: &configuration::Configuration,
    skill_id: i32
) -> Result<models::CreateAgentSkill201Response, Error<ApplyAgentSkillCatalogUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_skill_id = skill_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/skills/{skill_id}/apply-catalog-update",
        configuration.base_path,
        skill_id = p_path_skill_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentSkill201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentSkill201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<ApplyAgentSkillCatalogUpdateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы подключить и сразу включить у агента несколько навыков, отправьте
/// POST-запрос на `/api/v1/cloud-ai/agents/{agent_id}/skills/connect`.  Навыки,
/// которых нет в аккаунте, пропускаются без ошибки. Уже подключённые навыки
/// включаются; если для них передана версия, агент закрепляется на ней. В
/// ответе — только навыки, которые удалось подключить.
pub async fn connect_agent_skills_to_agent(
    configuration: &configuration::Configuration,
    agent_id: i32,
    connect_skills: models::ConnectSkills
) -> Result<models::GetAgentSkillsForAgent200Response, Error<ConnectAgentSkillsToAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_id = agent_id;
    let p_body_connect_skills = connect_skills;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{agent_id}/skills/connect",
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
    req_builder = req_builder.json(&p_body_connect_skills);

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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentSkillsForAgent200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentSkillsForAgent200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<ConnectAgentSkillsToAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы создать навык, отправьте POST-запрос на `/api/v1/cloud-ai/skills`.
/// Навык создаётся с версией 1 и ни к одному агенту не подключается.
/// Предупреждения о качестве текста приходят в `meta.warnings` и не мешают
/// сохранению. Скрипты передаются в теле запроса. Их общий размер — до 10 МБ, а
/// всё тело запроса — до 16 МБ; запрос с телом больше отклоняется до проверки.
/// Если навык с таким названием уже есть, вернётся 409 с `error_code`
/// `skill_name_already_exists`. Если скрипты не прошли проверку (больше 20
/// файлов, повторяющиеся имена, общий размер больше 10 МБ, файл без
/// содержимого), вернётся 400 с `error_code` `skill_scripts_invalid`.
pub async fn create_agent_skill(
    configuration: &configuration::Configuration,
    create_skill: models::CreateSkill
) -> Result<models::CreateAgentSkill201Response, Error<CreateAgentSkillError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_skill = create_skill;

    let uri_str = format!("{}/api/v1/cloud-ai/skills", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_skill);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentSkill201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentSkill201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<CreateAgentSkillError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы удалить навык, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/skills/{skill_id}`.  Навык отключается от всех агентов,
/// история версий и скрипты удаляются. Срабатывания навыка под старыми ответами
/// агентов сохраняются.
pub async fn delete_agent_skill(
    configuration: &configuration::Configuration,
    skill_id: i32
) -> Result<(), Error<DeleteAgentSkillError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_skill_id = skill_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/skills/{skill_id}",
        configuration.base_path,
        skill_id = p_path_skill_id
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
        let entity: Option<DeleteAgentSkillError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы полностью отключить навык от агента, отправьте DELETE-запрос на
/// `/api/v1/cloud-ai/agents/{agent_id}/skills/{skill_id}`.  Закреплённая версия
/// при этом не сохраняется. Чтобы выключить навык на время, используйте
/// PUT-запрос с `is_enabled: false`. Если навык не подключён к агенту, вернётся
/// 404 с `error_code` `agent_skill_link_not_found`.
pub async fn disconnect_agent_skill_from_agent(
    configuration: &configuration::Configuration,
    agent_id: i32,
    skill_id: i32
) -> Result<(), Error<DisconnectAgentSkillFromAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_id = agent_id;
    let p_path_skill_id = skill_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{agent_id}/skills/{skill_id}",
        configuration.base_path,
        agent_id = p_path_agent_id,
        skill_id = p_path_skill_id
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
        let entity: Option<DisconnectAgentSkillFromAgentError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы создать копию навыка, отправьте POST-запрос на
/// `/api/v1/cloud-ai/skills/{skill_id}/duplicate`.  Копия получает свободное
/// название вида «Название (копия)» или «Название (копия 2)», текст и скрипты
/// текущей версии оригинала и собственную историю с версии 1. К агентам копия
/// не подключается. Копия навыка из каталога становится обычным навыком
/// аккаунта.
pub async fn duplicate_agent_skill(
    configuration: &configuration::Configuration,
    skill_id: i32
) -> Result<models::CreateAgentSkill201Response, Error<DuplicateAgentSkillError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_skill_id = skill_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/skills/{skill_id}/duplicate",
        configuration.base_path,
        skill_id = p_path_skill_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentSkill201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentSkill201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<DuplicateAgentSkillError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить карточку навыка, отправьте GET-запрос на
/// `/api/v1/cloud-ai/skills/{skill_id}`.  В карточке есть скрипты текущей
/// версии (без содержимого), счётчик версий и агенты, к которым подключён
/// навык.
pub async fn get_agent_skill(
    configuration: &configuration::Configuration,
    skill_id: i32
) -> Result<models::GetAgentSkill200Response, Error<GetAgentSkillError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_skill_id = skill_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/skills/{skill_id}",
        configuration.base_path,
        skill_id = p_path_skill_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentSkill200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentSkill200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentSkillError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить каталог готовых навыков Timeweb Cloud, отправьте GET-запрос
/// на `/api/v1/cloud-ai/skill-catalog`.  Навыки приходят в порядке показа,
/// сгруппированные по `category`. Инструкция возвращается полностью, чтобы её
/// можно было прочитать до установки. Для установленных навыков заполнены
/// `is_installed`, `installed_skill_id`, `is_modified` и `has_catalog_update`.
pub async fn get_agent_skill_catalog(
    configuration: &configuration::Configuration
) -> Result<models::GetAgentSkillCatalog200Response, Error<GetAgentSkillCatalogError>> {
    let uri_str = format!("{}/api/v1/cloud-ai/skill-catalog", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentSkillCatalog200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentSkillCatalog200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentSkillCatalogError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить навыки аккаунта, отправьте GET-запрос на
/// `/api/v1/cloud-ai/skills`.  Сначала идут недавно изменённые. Параметр
/// `search` ищет по названию и по тексту инструкции.
pub async fn get_agent_skill_library(
    configuration: &configuration::Configuration,
    search: Option<&str>
) -> Result<models::GetAgentSkillLibrary200Response, Error<GetAgentSkillLibraryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_search = search;

    let uri_str = format!("{}/api/v1/cloud-ai/skills", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentSkillLibrary200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentSkillLibrary200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentSkillLibraryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить текст конкретной версии навыка, отправьте GET-запрос на
/// `/api/v1/cloud-ai/skills/{skill_id}/versions/{version}`.  Номер версии
/// сработавшего навыка есть в срабатываниях под ответами агента.
pub async fn get_agent_skill_version(
    configuration: &configuration::Configuration,
    skill_id: i32,
    version: i32
) -> Result<models::GetAgentSkillVersion200Response, Error<GetAgentSkillVersionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_skill_id = skill_id;
    let p_path_version = version;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/skills/{skill_id}/versions/{version}",
        configuration.base_path,
        skill_id = p_path_skill_id,
        version = p_path_version
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentSkillVersion200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentSkillVersion200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentSkillVersionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить историю версий навыка, отправьте GET-запрос на
/// `/api/v1/cloud-ai/skills/{skill_id}/versions`.
pub async fn get_agent_skill_versions(
    configuration: &configuration::Configuration,
    skill_id: i32
) -> Result<models::GetAgentSkillVersions200Response, Error<GetAgentSkillVersionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_skill_id = skill_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/skills/{skill_id}/versions",
        configuration.base_path,
        skill_id = p_path_skill_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentSkillVersions200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentSkillVersions200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentSkillVersionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы получить навыки агента, отправьте GET-запрос на
/// `/api/v1/cloud-ai/agents/{agent_id}/skills`.  В ответе все навыки аккаунта:
/// неподключённые приходят с `is_connected: false`. `has_newer_version: true`
/// означает, что агент закреплён на устаревшей версии.
pub async fn get_agent_skills_for_agent(
    configuration: &configuration::Configuration,
    agent_id: i32
) -> Result<models::GetAgentSkillsForAgent200Response, Error<GetAgentSkillsForAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_id = agent_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{agent_id}/skills",
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAgentSkillsForAgent200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAgentSkillsForAgent200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<GetAgentSkillsForAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы установить навык из каталога в библиотеку аккаунта, отправьте
/// POST-запрос на `/api/v1/cloud-ai/skill-catalog/{catalog_item_id}/install`.
/// Повторная установка не создаёт копию, а возвращает уже установленный навык с
/// `meta.already_installed: true`. Если название занято вашим навыком, к нему
/// добавляется «(копия)». Чтобы сразу включить навык у агента, используйте
/// `POST /api/v1/cloud-ai/agents/{agent_id}/skills/install-and-connect`.
pub async fn install_agent_skill_from_catalog(
    configuration: &configuration::Configuration,
    catalog_item_id: i32
) -> Result<
    models::InstallAgentSkillFromCatalog200Response,
    Error<InstallAgentSkillFromCatalogError>
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_catalog_item_id = catalog_item_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/skill-catalog/{catalog_item_id}/install",
        configuration.base_path,
        catalog_item_id = p_path_catalog_item_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::InstallAgentSkillFromCatalog200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::InstallAgentSkillFromCatalog200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<InstallAgentSkillFromCatalogError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы установить навык из каталога и сразу включить его у агента, отправьте
/// POST-запрос на
/// `/api/v1/cloud-ai/agents/{agent_id}/skills/install-and-connect`.  Если навык
/// уже установлен, копия не создаётся (`meta.already_installed: true`), навык
/// просто включается у агента.
pub async fn install_and_connect_agent_skill(
    configuration: &configuration::Configuration,
    agent_id: i32,
    install_and_connect_skill: models::InstallAndConnectSkill
) -> Result<models::InstallAndConnectAgentSkill200Response, Error<InstallAndConnectAgentSkillError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_id = agent_id;
    let p_body_install_and_connect_skill = install_and_connect_skill;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{agent_id}/skills/install-and-connect",
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
    req_builder = req_builder.json(&p_body_install_and_connect_skill);

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
                    "Received `text/plain` content type response that cannot be converted to `models::InstallAndConnectAgentSkill200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::InstallAndConnectAgentSkill200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<InstallAndConnectAgentSkillError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы сделать версию из истории текущей, отправьте POST-запрос на
/// `/api/v1/cloud-ai/skills/{skill_id}/versions/{version}/rollback`.  Новая
/// версия не создаётся (`meta.version_created` — `false`), название навыка не
/// меняется. Агенты остаются на своих закреплённых версиях. Если выбранная
/// версия уже текущая, ничего не меняется.
pub async fn rollback_agent_skill_version(
    configuration: &configuration::Configuration,
    skill_id: i32,
    version: i32
) -> Result<models::CreateAgentSkill201Response, Error<RollbackAgentSkillVersionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_skill_id = skill_id;
    let p_path_version = version;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/skills/{skill_id}/versions/{version}/rollback",
        configuration.base_path,
        skill_id = p_path_skill_id,
        version = p_path_version
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentSkill201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentSkill201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<RollbackAgentSkillVersionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы включить или выключить навык у агента, отправьте PUT-запрос на
/// `/api/v1/cloud-ai/agents/{agent_id}/skills/{skill_id}`.  Первое включение
/// подключает навык и закрепляет агента на текущей версии навыка или на версии
/// из `version`. Выключение сохраняет закреплённую версию, поэтому повторное
/// включение вернёт то же поведение. Изменения действуют сразу. Выключение
/// неподключённого навыка ничего не меняет.
pub async fn set_agent_skill_for_agent(
    configuration: &configuration::Configuration,
    agent_id: i32,
    skill_id: i32,
    set_agent_skill: models::SetAgentSkill
) -> Result<models::SetAgentSkillForAgent200Response, Error<SetAgentSkillForAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_id = agent_id;
    let p_path_skill_id = skill_id;
    let p_body_set_agent_skill = set_agent_skill;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{agent_id}/skills/{skill_id}",
        configuration.base_path,
        agent_id = p_path_agent_id,
        skill_id = p_path_skill_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_set_agent_skill);

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
                    "Received `text/plain` content type response that cannot be converted to `models::SetAgentSkillForAgent200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SetAgentSkillForAgent200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<SetAgentSkillForAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы перевести всех агентов, у которых включён навык, на его текущую
/// версию, отправьте POST-запрос на
/// `/api/v1/cloud-ai/skills/{skill_id}/agents/sync-versions`.  Агенты, у
/// которых навык выключен, остаются на своих версиях. Сколько агентов
/// переведено, видно в `meta.updated_count`.
pub async fn sync_agent_skill_agents_versions(
    configuration: &configuration::Configuration,
    skill_id: i32
) -> Result<
    models::SyncAgentSkillAgentsVersions200Response,
    Error<SyncAgentSkillAgentsVersionsError>
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_skill_id = skill_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/skills/{skill_id}/agents/sync-versions",
        configuration.base_path,
        skill_id = p_path_skill_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::SyncAgentSkillAgentsVersions200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SyncAgentSkillAgentsVersions200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<SyncAgentSkillAgentsVersionsError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы перевести все включённые у агента навыки на их текущие версии,
/// отправьте POST-запрос на
/// `/api/v1/cloud-ai/agents/{agent_id}/skills/sync-versions`.  В ответе все
/// навыки аккаунта с состоянием у агента, как в списке навыков агента. Сколько
/// навыков переведено, видно в `updated_count` в корне ответа.
pub async fn sync_agent_skill_versions_for_agent(
    configuration: &configuration::Configuration,
    agent_id: i32
) -> Result<
    models::SyncAgentSkillVersionsForAgent200Response,
    Error<SyncAgentSkillVersionsForAgentError>
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_id = agent_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{agent_id}/skills/sync-versions",
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
                    "Received `text/plain` content type response that cannot be converted to `models::SyncAgentSkillVersionsForAgent200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SyncAgentSkillVersionsForAgent200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<SyncAgentSkillVersionsForAgentError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы изменить навык, отправьте PATCH-запрос на
/// `/api/v1/cloud-ai/skills/{skill_id}`, передав только изменяемые поля.
/// Изменение «когда применять», инструкции или скриптов создаёт новую версию.
/// Переименование и сохранение без изменений версию не создают
/// (`meta.version_created`). Подключённые агенты остаются на своих версиях.
/// Сколько агентов затронуто, видно в `meta.affected_agents_count`. Если в
/// истории уже 50 версий, самая старая свободная версия удаляется
/// (`meta.pruned_versions`). Скрипты передаются в теле запроса. Их общий размер
/// — до 10 МБ, а всё тело запроса — до 16 МБ; запрос с телом больше отклоняется
/// до проверки.  Возможные ошибки 409: `skill_name_already_exists` — название
/// занято; `skill_versions_limit_exceeded` — все 50 версий заняты текущей и
/// закреплёнными за агентами; `skill_was_modified_concurrently` — навык
/// одновременно изменён другим запросом, повторите сохранение.
pub async fn update_agent_skill(
    configuration: &configuration::Configuration,
    skill_id: i32,
    update_skill: models::UpdateSkill
) -> Result<models::CreateAgentSkill201Response, Error<UpdateAgentSkillError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_skill_id = skill_id;
    let p_body_update_skill = update_skill;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/skills/{skill_id}",
        configuration.base_path,
        skill_id = p_path_skill_id
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
    req_builder = req_builder.json(&p_body_update_skill);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateAgentSkill201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateAgentSkill201Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<UpdateAgentSkillError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}

/// Чтобы перевести агента на текущую версию навыка, отправьте POST-запрос на
/// `/api/v1/cloud-ai/agents/{agent_id}/skills/{skill_id}/update-version`.  Если
/// навык не подключён к агенту, вернётся 404 с `error_code`
/// `agent_skill_link_not_found`.
pub async fn update_agent_skill_version_for_agent(
    configuration: &configuration::Configuration,
    agent_id: i32,
    skill_id: i32
) -> Result<models::SetAgentSkillForAgent200Response, Error<UpdateAgentSkillVersionForAgentError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_id = agent_id;
    let p_path_skill_id = skill_id;

    let uri_str = format!(
        "{}/api/v1/cloud-ai/agents/{agent_id}/skills/{skill_id}/update-version",
        configuration.base_path,
        agent_id = p_path_agent_id,
        skill_id = p_path_skill_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::SetAgentSkillForAgent200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SetAgentSkillForAgent200Response`"
                ))));
            }
        }
    } else {
        let headers = resp.headers().clone();
        let content = resp.text().await?;
        let entity: Option<UpdateAgentSkillVersionForAgentError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            headers,
            entity
        }))
    }
}
