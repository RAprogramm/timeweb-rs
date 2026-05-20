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

/// struct for typed errors of method [`create_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateClusterError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_cluster_node_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateClusterNodeGroupError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteClusterError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_cluster_node`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteClusterNodeError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_cluster_node_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteClusterNodeGroupError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_kubernetes_addons`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteKubernetesAddonsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClusterError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_cluster_kubeconfig`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClusterKubeconfigError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_cluster_node_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClusterNodeGroupError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_cluster_node_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClusterNodeGroupsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_cluster_nodes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClusterNodesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_cluster_nodes_from_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClusterNodesFromGroupError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_cluster_resources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClusterResourcesError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_clusters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClustersError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_k8_s_network_drivers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetK8SNetworkDriversError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_k8_s_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetK8SVersionsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_kubernetes_addons`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetKubernetesAddonsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_kubernetes_addons_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetKubernetesAddonsConfigError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_kubernetes_presets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetKubernetesPresetsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`increase_count_of_nodes_in_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IncreaseCountOfNodesInGroupError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`post_kubernetes_addons`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostKubernetesAddonsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`post_kubernetes_addons_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostKubernetesAddonsUpdateError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`reduce_count_of_nodes_in_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReduceCountOfNodesInGroupError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateClusterError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_cluster_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateClusterVersionError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status403(models::GetAccountStatus403Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// Чтобы создать кластер, отправьте POST-запрос на `/api/v1/k8s/clusters`.
pub async fn create_cluster(
    configuration: &configuration::Configuration,
    cluster_in: models::ClusterIn
) -> Result<models::CreateCluster201Response, Error<CreateClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_cluster_in = cluster_in;

    let uri_str = format!("{}/api/v1/k8s/clusters", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_cluster_in);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateCluster201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateCluster201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы создать группу нод кластера, отправьте POST-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/groups`.
pub async fn create_cluster_node_group(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    node_group_in: models::NodeGroupIn
) -> Result<models::CreateClusterNodeGroup201Response, Error<CreateClusterNodeGroupError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_body_node_group_in = node_group_in;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/groups",
        configuration.base_path,
        cluster_id = p_path_cluster_id
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
    req_builder = req_builder.json(&p_body_node_group_in);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateClusterNodeGroup201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateClusterNodeGroup201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateClusterNodeGroupError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить кластер, отправьте DELETE-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}`
pub async fn delete_cluster(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    hash: Option<&str>,
    code: Option<&str>
) -> Result<models::DeleteCluster200Response, Error<DeleteClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_query_hash = hash;
    let p_query_code = code;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}",
        configuration.base_path,
        cluster_id = p_path_cluster_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref param_value) = p_query_hash {
        req_builder = req_builder.query(&[("hash", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_code {
        req_builder = req_builder.query(&[("code", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::DeleteCluster200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::DeleteCluster200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить ноду, отправьте DELETE-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/nodes/{node_id}`.
pub async fn delete_cluster_node(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    node_id: i32
) -> Result<(), Error<DeleteClusterNodeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_node_id = node_id;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/nodes/{node_id}",
        configuration.base_path,
        cluster_id = p_path_cluster_id,
        node_id = p_path_node_id
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
        let entity: Option<DeleteClusterNodeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить группу нод, отправьте DELETE-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/groups/{group_id}`.
pub async fn delete_cluster_node_group(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    group_id: i32
) -> Result<(), Error<DeleteClusterNodeGroupError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_group_id = group_id;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/groups/{group_id}",
        configuration.base_path,
        cluster_id = p_path_cluster_id,
        group_id = p_path_group_id
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
        let entity: Option<DeleteClusterNodeGroupError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить дополнение, отправьте DELETE-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/addons/{addon_id}`.
pub async fn delete_kubernetes_addons(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    addon_id: i32
) -> Result<(), Error<DeleteKubernetesAddonsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_addon_id = addon_id;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/addons/{addon_id}",
        configuration.base_path,
        cluster_id = p_path_cluster_id,
        addon_id = p_path_addon_id
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
        let entity: Option<DeleteKubernetesAddonsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить информацию о кластере, отправьте GET-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}`
pub async fn get_cluster(
    configuration: &configuration::Configuration,
    cluster_id: i32
) -> Result<models::CreateCluster201Response, Error<GetClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}",
        configuration.base_path,
        cluster_id = p_path_cluster_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateCluster201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateCluster201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить файл kubeconfig, отправьте GET-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/kubeconfig`.
pub async fn get_cluster_kubeconfig(
    configuration: &configuration::Configuration,
    cluster_id: i32
) -> Result<String, Error<GetClusterKubeconfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/kubeconfig",
        configuration.base_path,
        cluster_id = p_path_cluster_id
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
        let entity: Option<GetClusterKubeconfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить информацию о группе нод, отправьте GET-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/groups/{group_id}`.
pub async fn get_cluster_node_group(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    group_id: i32
) -> Result<models::CreateClusterNodeGroup201Response, Error<GetClusterNodeGroupError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_group_id = group_id;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/groups/{group_id}",
        configuration.base_path,
        cluster_id = p_path_cluster_id,
        group_id = p_path_group_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateClusterNodeGroup201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateClusterNodeGroup201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetClusterNodeGroupError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить группы нод кластера, отправьте GET-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/groups`.
pub async fn get_cluster_node_groups(
    configuration: &configuration::Configuration,
    cluster_id: i32
) -> Result<models::GetClusterNodeGroups200Response, Error<GetClusterNodeGroupsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/groups",
        configuration.base_path,
        cluster_id = p_path_cluster_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetClusterNodeGroups200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetClusterNodeGroups200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetClusterNodeGroupsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список нод, отправьте GET-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/nodes`.
pub async fn get_cluster_nodes(
    configuration: &configuration::Configuration,
    cluster_id: i32
) -> Result<models::GetClusterNodesFromGroup200Response, Error<GetClusterNodesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/nodes",
        configuration.base_path,
        cluster_id = p_path_cluster_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetClusterNodesFromGroup200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetClusterNodesFromGroup200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetClusterNodesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список нод принадлежащих группе, отправьте GET-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/groups/{group_id}/nodes`.
pub async fn get_cluster_nodes_from_group(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    group_id: i32,
    limit: Option<i32>,
    offset: Option<i32>
) -> Result<models::GetClusterNodesFromGroup200Response, Error<GetClusterNodesFromGroupError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_group_id = group_id;
    let p_query_limit = limit;
    let p_query_offset = offset;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/groups/{group_id}/nodes",
        configuration.base_path,
        cluster_id = p_path_cluster_id,
        group_id = p_path_group_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetClusterNodesFromGroup200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetClusterNodesFromGroup200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetClusterNodesFromGroupError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Устаревший метод, работает только для старых кластеров. \\  Чтобы получить
/// ресурсы кластера, отправьте GET-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/resources`.
#[deprecated]
pub async fn get_cluster_resources(
    configuration: &configuration::Configuration,
    cluster_id: i32
) -> Result<models::GetClusterResources200Response, Error<GetClusterResourcesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/resources",
        configuration.base_path,
        cluster_id = p_path_cluster_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetClusterResources200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetClusterResources200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetClusterResourcesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список кластеров, отправьте GET-запрос на
/// `/api/v1/k8s/clusters`.
pub async fn get_clusters(
    configuration: &configuration::Configuration,
    limit: Option<i32>,
    offset: Option<i32>
) -> Result<models::GetClusters200Response, Error<GetClustersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_limit = limit;
    let p_query_offset = offset;

    let uri_str = format!("{}/api/v1/k8s/clusters", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetClusters200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetClusters200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetClustersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список сетевых драйверов k8s, отправьте GET-запрос в
/// `/api/v1/k8s/network-drivers`.
pub async fn get_k8_s_network_drivers(
    configuration: &configuration::Configuration
) -> Result<models::GetK8SNetworkDrivers200Response, Error<GetK8SNetworkDriversError>> {
    let uri_str = format!("{}/api/v1/k8s/network-drivers", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetK8SNetworkDrivers200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetK8SNetworkDrivers200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetK8SNetworkDriversError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список версий k8s, отправьте GET-запрос в
/// `/api/v1/k8s/k8s-versions`.
pub async fn get_k8_s_versions(
    configuration: &configuration::Configuration
) -> Result<models::GetK8SVersions200Response, Error<GetK8SVersionsError>> {
    let uri_str = format!("{}/api/v1/k8s/k8s-versions", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetK8SVersions200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetK8SVersions200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetK8SVersionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список установленных дополнений, отправьте GET-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/addons`.
pub async fn get_kubernetes_addons(
    configuration: &configuration::Configuration,
    cluster_id: i32
) -> Result<models::GetKubernetesAddons200Response, Error<GetKubernetesAddonsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/addons",
        configuration.base_path,
        cluster_id = p_path_cluster_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetKubernetesAddons200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetKubernetesAddons200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetKubernetesAddonsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список конфигураций дополнений, отправьте GET-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/addons-configs`.
pub async fn get_kubernetes_addons_config(
    configuration: &configuration::Configuration,
    cluster_id: i32
) -> Result<models::GetKubernetesAddonsConfig200Response, Error<GetKubernetesAddonsConfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/addons-configs",
        configuration.base_path,
        cluster_id = p_path_cluster_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetKubernetesAddonsConfig200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetKubernetesAddonsConfig200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetKubernetesAddonsConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список тарифов, отправьте GET-запрос в `/api/v1/presets/k8s`.
pub async fn get_kubernetes_presets(
    configuration: &configuration::Configuration
) -> Result<models::GetKubernetesPresets200Response, Error<GetKubernetesPresetsError>> {
    let uri_str = format!("{}/api/v1/presets/k8s", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetKubernetesPresets200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetKubernetesPresets200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetKubernetesPresetsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы увеличить количество нод в группе на указанное значение, отправьте
/// POST-запрос на `/api/v1/k8s/clusters/{cluster_id}/groups/{group_id}/nodes`
pub async fn increase_count_of_nodes_in_group(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    group_id: i32,
    increase_nodes: models::IncreaseNodes
) -> Result<models::GetClusterNodesFromGroup200Response, Error<IncreaseCountOfNodesInGroupError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_group_id = group_id;
    let p_body_increase_nodes = increase_nodes;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/groups/{group_id}/nodes",
        configuration.base_path,
        cluster_id = p_path_cluster_id,
        group_id = p_path_group_id
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
    req_builder = req_builder.json(&p_body_increase_nodes);

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
                    "Received `text/plain` content type response that cannot be converted to `models::GetClusterNodesFromGroup200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetClusterNodesFromGroup200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<IncreaseCountOfNodesInGroupError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы установить дополнение, отправьте POST-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/addons`.
pub async fn post_kubernetes_addons(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    cluster_in1: models::ClusterIn1
) -> Result<(), Error<PostKubernetesAddonsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_body_cluster_in1 = cluster_in1;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/addons",
        configuration.base_path,
        cluster_id = p_path_cluster_id
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
    req_builder = req_builder.json(&p_body_cluster_in1);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<PostKubernetesAddonsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы обновить конфигурацию дополнения, отправьте POST-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/addons/{addon_id}`.
pub async fn post_kubernetes_addons_update(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    addon_id: i32,
    cluster_in1: models::ClusterIn1
) -> Result<(), Error<PostKubernetesAddonsUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_addon_id = addon_id;
    let p_body_cluster_in1 = cluster_in1;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/addons/{addon_id}",
        configuration.base_path,
        cluster_id = p_path_cluster_id,
        addon_id = p_path_addon_id
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
    req_builder = req_builder.json(&p_body_cluster_in1);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<PostKubernetesAddonsUpdateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы уменьшить количество нод в группе на указанное значение, отправьте
/// DELETE-запрос в `/api/v1/k8s/clusters/{cluster_id}/groups/{group_id}/nodes`.
pub async fn reduce_count_of_nodes_in_group(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    group_id: i32,
    reduce_nodes: models::ReduceNodes
) -> Result<(), Error<ReduceCountOfNodesInGroupError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_group_id = group_id;
    let p_body_reduce_nodes = reduce_nodes;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/groups/{group_id}/nodes",
        configuration.base_path,
        cluster_id = p_path_cluster_id,
        group_id = p_path_group_id
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
    req_builder = req_builder.json(&p_body_reduce_nodes);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ReduceCountOfNodesInGroupError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы обновить информацию о кластере, отправьте PATCH-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}`
pub async fn update_cluster(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    cluster_edit: models::ClusterEdit
) -> Result<models::CreateCluster201Response, Error<UpdateClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_body_cluster_edit = cluster_edit;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}",
        configuration.base_path,
        cluster_id = p_path_cluster_id
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
    req_builder = req_builder.json(&p_body_cluster_edit);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateCluster201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateCluster201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы обновить версию кластера, отправьте PATCH-запрос в
/// `/api/v1/k8s/clusters/{cluster_id}/versions/update`
pub async fn update_cluster_version(
    configuration: &configuration::Configuration,
    cluster_id: i32,
    cluster_version_edit: models::ClusterVersionEdit
) -> Result<(), Error<UpdateClusterVersionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_body_cluster_version_edit = cluster_version_edit;

    let uri_str = format!(
        "{}/api/v1/k8s/clusters/{cluster_id}/versions/update",
        configuration.base_path,
        cluster_id = p_path_cluster_id
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
    req_builder = req_builder.json(&p_body_cluster_version_edit);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateClusterVersionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}
