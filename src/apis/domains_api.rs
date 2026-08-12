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

/// struct for typed errors of method [`add_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddDomainError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`add_subdomain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddSubdomainError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`check_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckDomainError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_domain_dns_record`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDomainDnsRecordError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_domain_dns_record_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDomainDnsRecordV2Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_domain_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDomainRequestError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_person`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePersonError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDomainError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_domain_dns_record`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDomainDnsRecordError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_domain_dns_record_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDomainDnsRecordV2Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_person`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePersonError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_subdomain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSubdomainError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDomainError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_domain_default_dns_records`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDomainDefaultDnsRecordsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_domain_dns_records`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDomainDnsRecordsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_domain_name_servers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDomainNameServersError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_domain_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDomainRequestError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_domain_requests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDomainRequestsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_domains`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDomainsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_person`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPersonError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_persons`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPersonsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_tld`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTldError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_tlds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTldsError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_domain_auto_prolongation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDomainAutoProlongationError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_domain_dns_record`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDomainDnsRecordError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_domain_dns_record_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDomainDnsRecordV2Error {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_domain_name_servers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDomainNameServersError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_domain_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDomainRequestError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status409(models::UpdateDatabaseInstance409Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`update_person`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePersonError {
    Status400(models::GetFinances400Response),
    Status401(models::GetFinances401Response),
    Status404(models::GetImage404Response),
    Status429(models::GetFinances429Response),
    Status500(models::GetFinances500Response),
    UnknownValue(serde_json::Value)
}

/// Чтобы добавить домен на свой аккаунт, отправьте запрос POST на
/// `/api/v1/add-domain/{fqdn}`.
pub async fn add_domain(
    configuration: &configuration::Configuration,
    fqdn: &str
) -> Result<(), Error<AddDomainError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;

    let uri_str = format!(
        "{}/api/v1/add-domain/{fqdn}",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn)
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
        let entity: Option<AddDomainError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы добавить поддомен, отправьте запрос POST на
/// `/api/v1/domains/{fqdn}/subdomains/{subdomain}`, задав необходимые атрибуты.
/// Поддомен будет добавлен с использованием предоставленной информации. Тело
/// ответа будет содержать объект JSON с информацией о добавленном поддомене.
pub async fn add_subdomain(
    configuration: &configuration::Configuration,
    fqdn: &str,
    subdomain: &str
) -> Result<models::AddSubdomain201Response, Error<AddSubdomainError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_path_subdomain = subdomain;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}/subdomains/{subdomain}",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn),
        subdomain = crate::apis::urlencode(p_path_subdomain)
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
                    "Received `text/plain` content type response that cannot be converted to `models::AddSubdomain201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AddSubdomain201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddSubdomainError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы проверить, доступен ли домен для регистрации, отправьте запрос GET на
/// `/api/v1/check-domain/{fqdn}`.
pub async fn check_domain(
    configuration: &configuration::Configuration,
    fqdn: &str
) -> Result<models::CheckDomain200Response, Error<CheckDomainError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;

    let uri_str = format!(
        "{}/api/v1/check-domain/{fqdn}",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn)
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
                    "Received `text/plain` content type response that cannot be converted to `models::CheckDomain200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CheckDomain200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CheckDomainError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы добавить информацию о DNS-записи для домена или поддомена, отправьте
/// запрос POST на `/api/v1/domains/{fqdn}/dns-records`, задав необходимые
/// атрибуты.  DNS-запись будет добавлена с использованием предоставленной
/// информации. Тело ответа будет содержать объект JSON с информацией о
/// добавленной DNS-записи.
#[deprecated]
pub async fn create_domain_dns_record(
    configuration: &configuration::Configuration,
    fqdn: &str,
    create_dns: models::CreateDns
) -> Result<models::CreateDomainDnsRecord201Response, Error<CreateDomainDnsRecordError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_body_create_dns = create_dns;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}/dns-records",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn)
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
    req_builder = req_builder.json(&p_body_create_dns);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainDnsRecord201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainDnsRecord201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateDomainDnsRecordError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы добавить информацию о DNS-записи для домена или поддомена, отправьте
/// запрос POST на `/api/v2/domains/{fqdn}/dns-records`, задав необходимые
/// атрибуты.  DNS-запись будет добавлена с использованием предоставленной
/// информации. Тело ответа будет содержать объект JSON с информацией о
/// добавленной DNS-записи.
pub async fn create_domain_dns_record_v2(
    configuration: &configuration::Configuration,
    fqdn: &str,
    create_dns_v2: models::CreateDnsV2
) -> Result<models::CreateDomainDnsRecordV2201Response, Error<CreateDomainDnsRecordV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_body_create_dns_v2 = create_dns_v2;

    let uri_str = format!(
        "{}/api/v2/domains/{fqdn}/dns-records",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn)
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
    req_builder = req_builder.json(&p_body_create_dns_v2);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainDnsRecordV2201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainDnsRecordV2201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateDomainDnsRecordV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы создать заявку на регистрацию/продление/трансфер домена, отправьте
/// POST-запрос в `api/v1/domains-requests`, задав необходимые атрибуты.  Заявка
/// будет создана с использованием предоставленной информации. Тело ответа будет
/// содержать объект JSON с информацией о созданной заявке.
pub async fn create_domain_request(
    configuration: &configuration::Configuration,
    create_domain_request_request: models::CreateDomainRequestRequest
) -> Result<models::CreateDomainRequest201Response, Error<CreateDomainRequestError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_domain_request_request = create_domain_request_request;

    let uri_str = format!("{}/api/v1/domains-requests", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_domain_request_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainRequest201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainRequest201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateDomainRequestError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы создать администратора доменов, отправьте POST-запрос на
/// `/api/v1/persons`, задав необходимые атрибуты. Набор полей зависит от типа
/// администратора: физическое лицо (`person`), организация (`org`) или
/// индивидуальный предприниматель (`ip`).   Тело ответа будет представлять
/// собой объект JSON с ключом `person`.
pub async fn create_person(
    configuration: &configuration::Configuration,
    create_person_request: models::CreatePersonRequest
) -> Result<models::CreatePerson201Response, Error<CreatePersonError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_person_request = create_person_request;

    let uri_str = format!("{}/api/v1/persons", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_person_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreatePerson201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreatePerson201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreatePersonError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить домен, отправьте запрос DELETE на `/api/v1/domains/{fqdn}`.
pub async fn delete_domain(
    configuration: &configuration::Configuration,
    fqdn: &str
) -> Result<(), Error<DeleteDomainError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn)
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
        let entity: Option<DeleteDomainError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить информацию о DNS-записи для домена или поддомена, отправьте
/// запрос DELETE на `/api/v1/domains/{fqdn}/dns-records/{record_id}`.
#[deprecated]
pub async fn delete_domain_dns_record(
    configuration: &configuration::Configuration,
    fqdn: &str,
    record_id: i32
) -> Result<(), Error<DeleteDomainDnsRecordError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_path_record_id = record_id;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}/dns-records/{record_id}",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn),
        record_id = p_path_record_id
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
        let entity: Option<DeleteDomainDnsRecordError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить информацию о DNS-записи для домена или поддомена, отправьте
/// запрос DELETE на `/api/v2/domains/{fqdn}/dns-records/{record_id}`.
pub async fn delete_domain_dns_record_v2(
    configuration: &configuration::Configuration,
    fqdn: &str,
    record_id: i32
) -> Result<(), Error<DeleteDomainDnsRecordV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_path_record_id = record_id;

    let uri_str = format!(
        "{}/api/v2/domains/{fqdn}/dns-records/{record_id}",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn),
        record_id = p_path_record_id
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
        let entity: Option<DeleteDomainDnsRecordV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить администратора доменов, отправьте DELETE-запрос на
/// `/api/v1/persons/{person_id}`.
pub async fn delete_person(
    configuration: &configuration::Configuration,
    person_id: i32
) -> Result<(), Error<DeletePersonError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_person_id = person_id;

    let uri_str = format!(
        "{}/api/v1/persons/{person_id}",
        configuration.base_path,
        person_id = p_path_person_id
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
        let entity: Option<DeletePersonError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить поддомен, отправьте запрос DELETE на
/// `/api/v1/domains/{fqdn}/subdomains/{subdomain}`.
pub async fn delete_subdomain(
    configuration: &configuration::Configuration,
    fqdn: &str,
    subdomain: &str
) -> Result<(), Error<DeleteSubdomainError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_path_subdomain = subdomain;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}/subdomains/{subdomain}",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn),
        subdomain = crate::apis::urlencode(p_path_subdomain)
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
        let entity: Option<DeleteSubdomainError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы отобразить информацию об отдельном домене, отправьте запрос GET на
/// `/api/v1/domains/{fqdn}`.
pub async fn get_domain(
    configuration: &configuration::Configuration,
    fqdn: &str
) -> Result<models::GetDomain200Response, Error<GetDomainError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn)
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetDomain200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetDomain200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDomainError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить информацию обо всех DNS-записях по умолчанию домена или
/// поддомена, отправьте запрос GET на
/// `/api/v1/domains/{fqdn}/default-dns-records`.
pub async fn get_domain_default_dns_records(
    configuration: &configuration::Configuration,
    fqdn: &str,
    limit: Option<i32>,
    offset: Option<i32>
) -> Result<models::GetDomainDnsRecords200Response, Error<GetDomainDefaultDnsRecordsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_query_limit = limit;
    let p_query_offset = offset;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}/default-dns-records",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn)
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetDomainDnsRecords200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetDomainDnsRecords200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDomainDefaultDnsRecordsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить информацию обо всех пользовательских DNS-записях домена или
/// поддомена, отправьте запрос GET на `/api/v1/domains/{fqdn}/dns-records`.
pub async fn get_domain_dns_records(
    configuration: &configuration::Configuration,
    fqdn: &str,
    limit: Option<i32>,
    offset: Option<i32>
) -> Result<models::GetDomainDnsRecords200Response, Error<GetDomainDnsRecordsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_query_limit = limit;
    let p_query_offset = offset;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}/dns-records",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn)
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetDomainDnsRecords200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetDomainDnsRecords200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDomainDnsRecordsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список name-серверов домена, отправьте запрос GET на
/// `/api/v1/domains/{fqdn}/name-servers`.
pub async fn get_domain_name_servers(
    configuration: &configuration::Configuration,
    fqdn: &str
) -> Result<models::GetDomainNameServers200Response, Error<GetDomainNameServersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}/name-servers",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn)
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetDomainNameServers200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetDomainNameServers200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDomainNameServersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить заявку на регистрацию/продление/трансфер домена, отправьте
/// запрос GET на `/api/v1/domains-requests/{request_id}`.
pub async fn get_domain_request(
    configuration: &configuration::Configuration,
    request_id: i32
) -> Result<models::CreateDomainRequest201Response, Error<GetDomainRequestError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_request_id = request_id;

    let uri_str = format!(
        "{}/api/v1/domains-requests/{request_id}",
        configuration.base_path,
        request_id = p_path_request_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainRequest201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainRequest201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDomainRequestError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список заявок на регистрацию/продление/трансфер домена,
/// отправьте запрос GET на `/api/v1/domains-requests`.
pub async fn get_domain_requests(
    configuration: &configuration::Configuration,
    person_id: Option<i32>
) -> Result<models::GetDomainRequests200Response, Error<GetDomainRequestsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_person_id = person_id;

    let uri_str = format!("{}/api/v1/domains-requests", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_person_id {
        req_builder = req_builder.query(&[("person_id", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetDomainRequests200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetDomainRequests200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDomainRequestsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список всех доменов на вашем аккаунте, отправьте GET-запрос
/// на `/api/v1/domains`.   Тело ответа будет представлять собой объект JSON с
/// ключом `domains`.
pub async fn get_domains(
    configuration: &configuration::Configuration,
    limit: Option<i32>,
    offset: Option<i32>,
    idn_name: Option<&str>,
    linked_ip: Option<&str>,
    order: Option<&str>,
    sort: Option<&str>
) -> Result<models::GetDomains200Response, Error<GetDomainsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_idn_name = idn_name;
    let p_query_linked_ip = linked_ip;
    let p_query_order = order;
    let p_query_sort = sort;

    let uri_str = format!("{}/api/v1/domains", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_idn_name {
        req_builder = req_builder.query(&[("idn_name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_linked_ip {
        req_builder = req_builder.query(&[("linked_ip", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_order {
        req_builder = req_builder.query(&[("order", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetDomains200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetDomains200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDomainsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить администратора доменов, отправьте GET-запрос на
/// `/api/v1/persons/{person_id}`.   Тело ответа будет представлять собой объект
/// JSON с ключом `person`.
pub async fn get_person(
    configuration: &configuration::Configuration,
    person_id: i32
) -> Result<models::CreatePerson201Response, Error<GetPersonError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_person_id = person_id;

    let uri_str = format!(
        "{}/api/v1/persons/{person_id}",
        configuration.base_path,
        person_id = p_path_person_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::CreatePerson201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreatePerson201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPersonError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список администраторов доменов на вашем аккаунте, отправьте
/// GET-запрос на `/api/v1/persons`.   Тело ответа будет представлять собой
/// объект JSON с ключом `persons`.
pub async fn get_persons(
    configuration: &configuration::Configuration,
    limit: Option<i32>,
    offset: Option<i32>,
    is_closed: Option<bool>
) -> Result<models::GetPersons200Response, Error<GetPersonsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_is_closed = is_closed;

    let uri_str = format!("{}/api/v1/persons", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_is_closed {
        req_builder = req_builder.query(&[("is_closed", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetPersons200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetPersons200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPersonsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить информацию о доменной зоне по ID, отправьте запрос GET на
/// `/api/v1/tlds/{tld_id}`.
pub async fn get_tld(
    configuration: &configuration::Configuration,
    tld_id: i32
) -> Result<models::GetTld200Response, Error<GetTldError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_tld_id = tld_id;

    let uri_str = format!(
        "{}/api/v1/tlds/{tld_id}",
        configuration.base_path,
        tld_id = p_path_tld_id
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetTld200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetTld200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTldError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить информацию о доменных зонах, отправьте запрос GET на
/// `/api/v1/tlds`.
pub async fn get_tlds(
    configuration: &configuration::Configuration,
    is_published: Option<bool>,
    is_registered: Option<bool>
) -> Result<models::GetTlds200Response, Error<GetTldsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_is_published = is_published;
    let p_query_is_registered = is_registered;

    let uri_str = format!("{}/api/v1/tlds", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_is_published {
        req_builder = req_builder.query(&[("is_published", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_is_registered {
        req_builder = req_builder.query(&[("is_registered", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GetTlds200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetTlds200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTldsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы включить/выключить автопродление домена, отправьте запрос PATCH на
/// `/api/v1/domains/{fqdn}`, передав в теле запроса параметр
/// `is_autoprolong_enabled`
pub async fn update_domain_auto_prolongation(
    configuration: &configuration::Configuration,
    fqdn: &str,
    update_domain: models::UpdateDomain
) -> Result<
    models::UpdateDomainAutoProlongation200Response,
    Error<UpdateDomainAutoProlongationError>
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_body_update_domain = update_domain;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn)
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
    req_builder = req_builder.json(&p_body_update_domain);

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
                    "Received `text/plain` content type response that cannot be converted to `models::UpdateDomainAutoProlongation200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::UpdateDomainAutoProlongation200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateDomainAutoProlongationError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы обновить информацию о DNS-записи для домена или поддомена, отправьте
/// запрос PATCH на `/api/v1/domains/{fqdn}/dns-records/{record_id}`, задав
/// необходимые атрибуты.  DNS-запись будет обновлена с использованием
/// предоставленной информации. Тело ответа будет содержать объект JSON с
/// информацией об добавленной DNS-записи.
#[deprecated]
pub async fn update_domain_dns_record(
    configuration: &configuration::Configuration,
    fqdn: &str,
    record_id: i32,
    create_dns: models::CreateDns
) -> Result<models::CreateDomainDnsRecord201Response, Error<UpdateDomainDnsRecordError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_path_record_id = record_id;
    let p_body_create_dns = create_dns;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}/dns-records/{record_id}",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn),
        record_id = p_path_record_id
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
    req_builder = req_builder.json(&p_body_create_dns);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainDnsRecord201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainDnsRecord201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateDomainDnsRecordError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы обновить информацию о DNS-записи для домена или поддомена, отправьте
/// запрос PATCH на `/api/v2/domains/{fqdn}/dns-records/{record_id}`, задав
/// необходимые атрибуты.  DNS-запись будет обновлена с использованием
/// предоставленной информации. Тело ответа будет содержать объект JSON с
/// информацией об обновленной DNS-записи.
pub async fn update_domain_dns_record_v2(
    configuration: &configuration::Configuration,
    fqdn: &str,
    record_id: i32,
    create_dns_v2: models::CreateDnsV2
) -> Result<models::CreateDomainDnsRecordV2201Response, Error<UpdateDomainDnsRecordV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_path_record_id = record_id;
    let p_body_create_dns_v2 = create_dns_v2;

    let uri_str = format!(
        "{}/api/v2/domains/{fqdn}/dns-records/{record_id}",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn),
        record_id = p_path_record_id
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
    req_builder = req_builder.json(&p_body_create_dns_v2);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainDnsRecordV2201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainDnsRecordV2201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateDomainDnsRecordV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы изменить name-серверы домена, отправьте запрос PUT на
/// `/api/v1/domains/{fqdn}/name-servers`, задав необходимые атрибуты.
/// Name-серверы будут изменены с использованием предоставленной информации.
/// Тело ответа будет содержать объект JSON с информацией о name-серверах
/// домена.
pub async fn update_domain_name_servers(
    configuration: &configuration::Configuration,
    fqdn: &str,
    update_domain_name_servers: models::UpdateDomainNameServers
) -> Result<models::GetDomainNameServers200Response, Error<UpdateDomainNameServersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_fqdn = fqdn;
    let p_body_update_domain_name_servers = update_domain_name_servers;

    let uri_str = format!(
        "{}/api/v1/domains/{fqdn}/name-servers",
        configuration.base_path,
        fqdn = crate::apis::urlencode(p_path_fqdn)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_domain_name_servers);

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
                    "Received `text/plain` content type response that cannot be converted to `models::GetDomainNameServers200Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GetDomainNameServers200Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateDomainNameServersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы оплатить/обновить заявку на регистрацию/продление/трансфер домена,
/// отправьте запрос PATCH на `/api/v1/domains-requests/{request_id}`, задав
/// необходимые атрибуты.  Заявка будет обновлена с использованием
/// предоставленной информации. Тело ответа будет содержать объект JSON с
/// информацией о обновленной заявке.
pub async fn update_domain_request(
    configuration: &configuration::Configuration,
    request_id: i32,
    update_domain_request_request: models::UpdateDomainRequestRequest
) -> Result<models::CreateDomainRequest201Response, Error<UpdateDomainRequestError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_request_id = request_id;
    let p_body_update_domain_request_request = update_domain_request_request;

    let uri_str = format!(
        "{}/api/v1/domains-requests/{request_id}",
        configuration.base_path,
        request_id = p_path_request_id
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
    req_builder = req_builder.json(&p_body_update_domain_request_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreateDomainRequest201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreateDomainRequest201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateDomainRequestError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы обновить контактные данные администратора доменов, отправьте
/// PUT-запрос на `/api/v1/persons/{person_id}`.   Тело ответа будет
/// представлять собой объект JSON с ключом `person`.
pub async fn update_person(
    configuration: &configuration::Configuration,
    person_id: i32,
    update_person: models::UpdatePerson
) -> Result<models::CreatePerson201Response, Error<UpdatePersonError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_person_id = person_id;
    let p_body_update_person = update_person;

    let uri_str = format!(
        "{}/api/v1/persons/{person_id}",
        configuration.base_path,
        person_id = p_path_person_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_person);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CreatePerson201Response`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CreatePerson201Response`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdatePersonError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}
