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

/// struct for typed errors of method [`add_provider`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddProviderError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAppError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_deploy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDeployError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAppError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_provider`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteProviderError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`deploy_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeployActionError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAppError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_app_deploys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAppDeploysError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_app_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAppLogsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_app_statistics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAppStatisticsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_apps`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAppsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_apps_presets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAppsPresetsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_branches`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBranchesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_commits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommitsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_deploy_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDeployLogsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_deploy_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDeploySettingsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_frameworks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFrameworksError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_providers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProvidersError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_repositories`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRepositoriesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_app_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAppSettingsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_app_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAppStateError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// Чтобы привязать аккаунт провайдера отправьте POST-запрос в
/// `/api/v1/vcs-provider`, задав необходимые атрибуты.
pub async fn add_provider(
    configuration: &configuration::Configuration,
    add_provider_request: models::AddProviderRequest
) -> Result<models::AddProvider201Response, Error<AddProviderError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_add_provider_request = add_provider_request;

    let uri_str = format!("{}/api/v1/vcs-provider", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_add_provider_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::AddProvider201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AddProvider201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddProviderError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы создать приложение, отправьте POST-запрос в `/api/v1/apps`, задав
/// необходимые атрибуты.
pub async fn create_app(
    configuration: &configuration::Configuration,
    create_app: models::CreateApp
) -> Result<models::CreateApp201Response, Error<CreateAppError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_app = create_app;

    let uri_str = format!("{}/api/v1/apps", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_app);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateApp201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateApp201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAppError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы запустить деплой приложения, отправьте POST-запрос в
/// `/api/v1/apps/{app_id}/deploy`, задав необходимые атрибуты.
pub async fn create_deploy(
    configuration: &configuration::Configuration,
    app_id: &str,
    create_deploy_request: models::CreateDeployRequest
) -> Result<models::CreateDeploy201Response, Error<CreateDeployError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_body_create_deploy_request = create_deploy_request;

    let uri_str = format!(
        "{}/api/v1/apps/{app_id}/deploy",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
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
    req_builder = req_builder.json(&p_body_create_deploy_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDeploy201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDeploy201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateDeployError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить приложение, отправьте DELETE-запрос в `/api/v1/apps/{app_id}`.
pub async fn delete_app(
    configuration: &configuration::Configuration,
    app_id: &str
) -> Result<(), Error<DeleteAppError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;

    let uri_str = format!(
        "{}/api/v1/apps/{app_id}",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
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
        let entity: Option<DeleteAppError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы отвязать vcs провайдера от аккаунта, отправьте DELETE-запрос в
/// `/api/v1/vcs-provider/{provider_id}`.
pub async fn delete_provider(
    configuration: &configuration::Configuration,
    provider_id: &str
) -> Result<(), Error<DeleteProviderError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_provider_id = provider_id;

    let uri_str = format!(
        "{}/api/v1/vcs-provider/{provider_id}",
        configuration.base_path,
        provider_id = crate::apis::urlencode(p_path_provider_id)
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
        let entity: Option<DeleteProviderError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы остановить деплой приложения, отправьте POST-запрос в
/// `api/v1/apps/{app_id}/deploy/{deploy_id}/stop`.
pub async fn deploy_action(
    configuration: &configuration::Configuration,
    app_id: &str,
    deploy_id: &str
) -> Result<models::CreateDeploy201Response, Error<DeployActionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_deploy_id = deploy_id;

    let uri_str = format!(
        "{}/api/v1/apps/{app_id}/deploy/{deploy_id}/stop",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        deploy_id = crate::apis::urlencode(p_path_deploy_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDeploy201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDeploy201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeployActionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить приложение по id, отправьте GET-запрос на
/// `/api/v1/apps/{app_id}`.
pub async fn get_app(
    configuration: &configuration::Configuration,
    app_id: &str
) -> Result<models::CreateApp201Response, Error<GetAppError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;

    let uri_str = format!(
        "{}/api/v1/apps/{app_id}",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateApp201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateApp201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAppError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список деплоев приложения, отправьте GET-запрос на
/// `/api/v1/apps/{app_id}/deploys`.
pub async fn get_app_deploys(
    configuration: &configuration::Configuration,
    app_id: &str,
    limit: Option<i32>,
    offset: Option<i32>
) -> Result<models::GetAppDeploys200Response, Error<GetAppDeploysError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_query_limit = limit;
    let p_query_offset = offset;

    let uri_str = format!(
        "{}/api/v1/apps/{app_id}/deploys",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAppDeploys200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAppDeploys200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAppDeploysError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить логи приложения, отправьте GET-запрос на
/// `/api/v1/apps/{app_id}/logs`.
pub async fn get_app_logs(
    configuration: &configuration::Configuration,
    app_id: &str
) -> Result<models::GetAppLogs200Response, Error<GetAppLogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;

    let uri_str = format!(
        "{}/api/v1/apps/{app_id}/logs",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAppLogs200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAppLogs200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAppLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить статистику сервера, отправьте GET-запрос на
/// `/api/v1/apps/{app_id}/statistics`. Метод поддерживает только приложения
/// `type: backend`.
pub async fn get_app_statistics(
    configuration: &configuration::Configuration,
    app_id: &str,
    date_from: &str,
    date_to: &str
) -> Result<models::GetServerStatistics200Response, Error<GetAppStatisticsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_query_date_from = date_from;
    let p_query_date_to = date_to;

    let uri_str = format!(
        "{}/api/v1/apps/{app_id}/statistics",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("date_from", &p_query_date_from.to_string())]);
    req_builder = req_builder.query(&[("date_to", &p_query_date_to.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetServerStatistics200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetServerStatistics200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAppStatisticsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список приложений, отправьте GET-запрос на `/api/v1/apps`.
pub async fn get_apps(
    configuration: &configuration::Configuration
) -> Result<models::GetApps200Response, Error<GetAppsError>> {
    let uri_str = format!("{}/api/v1/apps", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetApps200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetApps200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAppsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список доступных тарифов, отправьте GET-запрос на
/// `/api/v1/presets/apps`.
pub async fn get_apps_presets(
    configuration: &configuration::Configuration,
    app_id: &str
) -> Result<models::GetAppsPresets200Response, Error<GetAppsPresetsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_app_id = app_id;

    let uri_str = format!("{}/api/v1/presets/apps", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("app_id", &p_query_app_id.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetAppsPresets200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetAppsPresets200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAppsPresetsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список веток репозитория, отправьте GET-запрос на
/// `/api/v1/vcs-provider/{provider_id}/repository/{repository_id}`.
pub async fn get_branches(
    configuration: &configuration::Configuration,
    provider_id: &str,
    repository_id: &str
) -> Result<models::GetBranches200Response, Error<GetBranchesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_provider_id = provider_id;
    let p_path_repository_id = repository_id;

    let uri_str = format!(
        "{}/api/v1/vcs-provider/{provider_id}/repository/{repository_id}",
        configuration.base_path,
        provider_id = crate::apis::urlencode(p_path_provider_id),
        repository_id = crate::apis::urlencode(p_path_repository_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetBranches200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetBranches200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetBranchesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список коммитов ветки репозитория, отправьте GET-запрос на
/// `/api/v1/vcs-provider/{provider_id}/repository/{repository_id}/branch`.
pub async fn get_commits(
    configuration: &configuration::Configuration,
    account_id: &str,
    provider_id: &str,
    repository_id: &str,
    name: &str
) -> Result<models::GetCommits200Response, Error<GetCommitsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_account_id = account_id;
    let p_path_provider_id = provider_id;
    let p_path_repository_id = repository_id;
    let p_query_name = name;

    let uri_str = format!(
        "{}/api/v1/vcs-provider/{provider_id}/repository/{repository_id}/branch",
        configuration.base_path,
        provider_id = crate::apis::urlencode(p_path_provider_id),
        repository_id = crate::apis::urlencode(p_path_repository_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("account_id", &p_query_account_id.to_string())]);
    req_builder = req_builder.query(&[("name", &p_query_name.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetCommits200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetCommits200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCommitsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить информацию о деплое, отправьте GET-запрос на
/// `api/v1/apps/{app_id}/deploy/{deploy_id}/logs`.
pub async fn get_deploy_logs(
    configuration: &configuration::Configuration,
    app_id: &str,
    deploy_id: &str,
    debug: Option<bool>
) -> Result<models::GetDeployLogs200Response, Error<GetDeployLogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_deploy_id = deploy_id;
    let p_query_debug = debug;

    let uri_str = format!(
        "{}/api/v1/apps/{app_id}/deploy/{deploy_id}/logs",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        deploy_id = crate::apis::urlencode(p_path_deploy_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_debug {
        req_builder = req_builder.query(&[("debug", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetDeployLogs200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetDeployLogs200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDeployLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список настроек деплоя, отправьте GET-запрос на
/// `/api/v1/deploy-settings/apps`.
pub async fn get_deploy_settings(
    configuration: &configuration::Configuration,
    app_id: &str
) -> Result<models::GetDeploySettings200Response, Error<GetDeploySettingsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_app_id = app_id;

    let uri_str = format!("{}/api/v1/deploy-settings/apps", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("app_id", &p_query_app_id.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetDeploySettings200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetDeploySettings200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDeploySettingsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список доступных фреймворков, отправьте GET-запрос на
/// `/api/v1/frameworks/apps`.
pub async fn get_frameworks(
    configuration: &configuration::Configuration,
    app_id: &str
) -> Result<models::GetFrameworks200Response, Error<GetFrameworksError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_app_id = app_id;

    let uri_str = format!("{}/api/v1/frameworks/apps", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("app_id", &p_query_app_id.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetFrameworks200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetFrameworks200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFrameworksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список vcs провайдеров, отправьте GET-запрос на
/// `/api/v1/vcs-provider`.
pub async fn get_providers(
    configuration: &configuration::Configuration
) -> Result<models::GetProviders200Response, Error<GetProvidersError>> {
    let uri_str = format!("{}/api/v1/vcs-provider", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetProviders200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetProviders200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetProvidersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список репозиториев vcs провайдера, отправьте GET-запрос на
/// `/api/v1/vcs-provider/{provider_id}`.
pub async fn get_repositories(
    configuration: &configuration::Configuration,
    provider_id: &str
) -> Result<models::GetRepositories200Response, Error<GetRepositoriesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_provider_id = provider_id;

    let uri_str = format!(
        "{}/api/v1/vcs-provider/{provider_id}",
        configuration.base_path,
        provider_id = crate::apis::urlencode(p_path_provider_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetRepositories200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetRepositories200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRepositoriesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы изменить настройки приложения отправьте PATCH-запрос в
/// `/api/v1/apps/{app_id}`, задав необходимые атрибуты.
pub async fn update_app_settings(
    configuration: &configuration::Configuration,
    app_id: &str,
    update_settings: models::UpdateSettings
) -> Result<models::UpdateAppSettings200Response, Error<UpdateAppSettingsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_body_update_settings = update_settings;

    let uri_str = format!(
        "{}/api/v1/apps/{app_id}",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
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
    req_builder = req_builder.json(&p_body_update_settings);

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
                    "Received `text/plain` content type response that cannot be converted to `models::UpdateAppSettings200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::UpdateAppSettings200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateAppSettingsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы изменить состояние приложения отправьте PATCH-запрос в
/// `/api/v1/apps/{app_id}/action/{action}`, задав необходимые атрибуты.
pub async fn update_app_state(
    configuration: &configuration::Configuration,
    app_id: &str,
    action: &str
) -> Result<(), Error<UpdateAppStateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_action = action;

    let uri_str = format!(
        "{}/api/v1/apps/{app_id}/action/{action}",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        action = crate::apis::urlencode(p_path_action)
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

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateAppStateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}
