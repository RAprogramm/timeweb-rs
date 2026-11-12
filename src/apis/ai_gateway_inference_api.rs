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

/// struct for typed errors of method [`cancel_ai_gateway_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelAiGatewayBatchError {
    Status401(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status404(models::AiGatewayError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`cancel_ai_gateway_response`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelAiGatewayResponseError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status500(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_ai_gateway_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayBatchError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status402(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status404(models::AiGatewayError),
    Status405(models::AiGatewayError),
    Status410(models::AiGatewayError),
    Status422(models::AiGatewayError),
    Status429(models::AiGatewayError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_ai_gateway_chat_completion`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayChatCompletionError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status402(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status404(models::AiGatewayError),
    Status410(models::AiGatewayError),
    Status422(models::AiGatewayError),
    Status429(models::AiGatewayError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_ai_gateway_completion`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayCompletionError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status402(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status404(models::AiGatewayError),
    Status422(models::AiGatewayError),
    Status429(models::AiGatewayError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_ai_gateway_embedding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayEmbeddingError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status402(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status404(models::AiGatewayError),
    Status410(models::AiGatewayError),
    Status422(models::AiGatewayError),
    Status429(models::AiGatewayError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_ai_gateway_image`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayImageError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status402(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status404(models::AiGatewayError),
    Status410(models::AiGatewayError),
    Status422(models::AiGatewayError),
    Status429(models::AiGatewayError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_ai_gateway_image_edit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayImageEditError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status402(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status405(models::AiGatewayError),
    Status410(models::AiGatewayError),
    Status422(models::AiGatewayError),
    Status429(models::AiGatewayError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_ai_gateway_response`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayResponseError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status402(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status404(models::AiGatewayError),
    Status410(models::AiGatewayError),
    Status422(models::AiGatewayError),
    Status429(models::AiGatewayError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_ai_gateway_speech`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewaySpeechError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status402(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status410(models::AiGatewayError),
    Status422(models::AiGatewayError),
    Status429(models::AiGatewayError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`create_ai_gateway_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayTranscriptionError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status402(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status405(models::AiGatewayError),
    Status410(models::AiGatewayError),
    Status422(models::CreateAiGatewayTranscription422Response),
    Status429(models::AiGatewayError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_ai_gateway_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiGatewayFileError {
    Status401(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status404(models::AiGatewayError),
    Status409(models::AiGatewayError),
    Status500(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`delete_ai_gateway_response`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiGatewayResponseError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status500(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`download_ai_gateway_file_content`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadAiGatewayFileContentError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status404(models::AiGatewayError),
    Status500(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_gateway_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayBatchError {
    Status401(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status404(models::AiGatewayError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_gateway_batches`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayBatchesError {
    Status401(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status422(models::AiGatewayValidationError),
    Status500(models::AiGatewayError),
    Status503(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_gateway_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayFileError {
    Status401(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status404(models::AiGatewayError),
    Status500(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_gateway_models`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayModelsError {
    Status401(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status500(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`get_ai_gateway_response`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayResponseError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status500(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// struct for typed errors of method [`upload_ai_gateway_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadAiGatewayFileError {
    Status400(models::AiGatewayError),
    Status401(models::AiGatewayError),
    Status403(models::AiGatewayError),
    Status422(models::AiGatewayValidationError),
    Status500(models::AiGatewayError),
    UnknownValue(serde_json::Value)
}

/// Чтобы отменить батч, отправьте POST-запрос на
/// `/v1/batches/{batch_id}/cancel`.  Батч переходит в статус `cancelling`,
/// отмена выполняется асинхронно и может занять несколько минут; итоговый
/// статус `cancelled` проверяйте через `GET /v1/batches/{batch_id}`. Пока
/// отмена не завершилась, входной файл батча удалить нельзя.
pub async fn cancel_ai_gateway_batch(
    configuration: &configuration::Configuration,
    batch_id: &str
) -> Result<models::AiGatewayBatch, Error<CancelAiGatewayBatchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_batch_id = batch_id;

    let uri_str = format!(
        "{}/v1/batches/{batch_id}/cancel",
        configuration.base_path,
        batch_id = crate::apis::urlencode(p_path_batch_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayBatch`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayBatch`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CancelAiGatewayBatchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы отменить ответ, запущенный с `\"background\": true`, отправьте
/// POST-запрос на `/v1/responses/{response_id}/cancel`.  Отмена может
/// завершиться не сразу: итоговый статус `cancelled` проверяйте через `GET
/// /v1/responses/{response_id}`. Ответ, созданный без `background`, отменить
/// нельзя — вернётся ошибка 400. Неизвестный `response_id` также возвращает
/// 400.
pub async fn cancel_ai_gateway_response(
    configuration: &configuration::Configuration,
    response_id: &str
) -> Result<models::AiGatewayResponse, Error<CancelAiGatewayResponseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_response_id = response_id;

    let uri_str = format!(
        "{}/v1/responses/{response_id}/cancel",
        configuration.base_path,
        response_id = crate::apis::urlencode(p_path_response_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayResponse`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CancelAiGatewayResponseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы запустить асинхронную обработку входного файла, загруженного с `purpose=batch`, отправьте POST-запрос на `/v1/batches`.  При создании батча его оценочная стоимость резервируется на балансе, фактическая стоимость списывается по цене Batch API после выполнения. Модели, эндпоинты и коэффициенты цены Batch API — в поле `batch` каталога моделей (`GET https://api.timeweb.cloud/api/v3/cloud-ai/models`). Batch API доступен только для ключей AI Gateway.
pub async fn create_ai_gateway_batch(
    configuration: &configuration::Configuration,
    ai_gateway_batch_create_request: models::AiGatewayBatchCreateRequest
) -> Result<models::AiGatewayBatch, Error<CreateAiGatewayBatchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_ai_gateway_batch_create_request = ai_gateway_batch_create_request;

    let uri_str = format!("{}/v1/batches", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_ai_gateway_batch_create_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayBatch`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayBatch`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAiGatewayBatchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить следующий ответ модели в диалоге, отправьте POST-запрос на
/// `/v1/chat/completions`.  Запрос не хранит состояние: передавайте всю историю
/// диалога в `messages`. Поддерживаются вызов функций (`tools`),
/// структурированный ответ (`response_format`) и изображения во входных
/// сообщениях (для моделей с поддержкой зрения).  С `\"stream\": true` ответ
/// приходит потоком Server-Sent Events: события `chat.completion.chunk` в
/// строках `data:`, в конце — `data: [DONE]`. С `\"stream_options\":
/// {\"include_usage\": true}` последний чанк содержит расход токенов.  Если у
/// ключа настроены запасные модели и запрошенная модель не ответила, запрос
/// повторяется на запасных. Заголовок ответа `x-timeweb-attempted-fallbacks`
/// показывает, сколько запасных моделей было опробовано.
pub async fn create_ai_gateway_chat_completion(
    configuration: &configuration::Configuration,
    ai_gateway_chat_completion_request: models::AiGatewayChatCompletionRequest
) -> Result<models::AiGatewayChatCompletion, Error<CreateAiGatewayChatCompletionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_ai_gateway_chat_completion_request = ai_gateway_chat_completion_request;

    let uri_str = format!("{}/v1/chat/completions", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_ai_gateway_chat_completion_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayChatCompletion`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayChatCompletion`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAiGatewayChatCompletionError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить продолжение текста `prompt`, отправьте POST-запрос на
/// `/v1/completions`.  Это устаревший API OpenAI — для новых интеграций
/// используйте `/v1/chat/completions`. С `\"stream\": true` ответ приходит
/// потоком Server-Sent Events и заканчивается `data: [DONE]`.
pub async fn create_ai_gateway_completion(
    configuration: &configuration::Configuration,
    ai_gateway_completion_request: models::AiGatewayCompletionRequest
) -> Result<models::AiGatewayCompletion, Error<CreateAiGatewayCompletionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_ai_gateway_completion_request = ai_gateway_completion_request;

    let uri_str = format!("{}/v1/completions", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_ai_gateway_completion_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayCompletion`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayCompletion`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAiGatewayCompletionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить векторные представления текстов, отправьте POST-запрос на
/// `/v1/embeddings`.  Используйте модель с `mode: embedding` из `GET
/// /v1/models`. Эмбеддинги возвращаются в порядке входных текстов.
pub async fn create_ai_gateway_embedding(
    configuration: &configuration::Configuration,
    ai_gateway_embedding_request: models::AiGatewayEmbeddingRequest
) -> Result<models::AiGatewayEmbeddingList, Error<CreateAiGatewayEmbeddingError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_ai_gateway_embedding_request = ai_gateway_embedding_request;

    let uri_str = format!("{}/v1/embeddings", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_ai_gateway_embedding_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayEmbeddingList`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayEmbeddingList`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAiGatewayEmbeddingError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы сгенерировать изображения по текстовому описанию, отправьте
/// POST-запрос на `/v1/images/generations`.  Используйте модель с `mode:
/// image_generation` из `GET /v1/models`.
pub async fn create_ai_gateway_image(
    configuration: &configuration::Configuration,
    ai_gateway_image_generation_request: models::AiGatewayImageGenerationRequest
) -> Result<models::AiGatewayImagesResponse, Error<CreateAiGatewayImageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_ai_gateway_image_generation_request = ai_gateway_image_generation_request;

    let uri_str = format!("{}/v1/images/generations", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_ai_gateway_image_generation_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayImagesResponse`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayImagesResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAiGatewayImageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы изменить или дополнить изображения по описанию, отправьте POST-запрос
/// на `/v1/images/edits` (multipart/form-data).  Доступно для моделей GPT Image
/// (`openai/gpt-image-*`) и FLUX.2 (`black_forest_labs/flux-2-*`). Для других
/// моделей, в том числе неизвестных, возвращается ошибка 405.
pub async fn create_ai_gateway_image_edit(
    configuration: &configuration::Configuration,
    model: &str,
    image: std::path::PathBuf,
    prompt: &str,
    mask: Option<std::path::PathBuf>,
    n: Option<i32>,
    size: Option<&str>,
    quality: Option<&str>
) -> Result<models::AiGatewayImagesResponse, Error<CreateAiGatewayImageEditError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_form_model = model;
    let p_form_image = image;
    let p_form_prompt = prompt;
    let p_form_mask = mask;
    let p_form_n = n;
    let p_form_size = size;
    let p_form_quality = quality;

    let uri_str = format!("{}/v1/images/edits", configuration.base_path);
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
    multipart_form = multipart_form.text("model", p_form_model.to_string());
    let file = TokioFile::open(&p_form_image).await?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_name = p_form_image
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let file_part =
        reqwest::multipart::Part::stream(reqwest::Body::wrap_stream(stream)).file_name(file_name);
    multipart_form = multipart_form.part("image", file_part);
    if let Some(ref param_value) = p_form_mask {
        let file = TokioFile::open(param_value).await?;
        let stream = FramedRead::new(file, BytesCodec::new());
        let file_name = param_value
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let file_part = reqwest::multipart::Part::stream(reqwest::Body::wrap_stream(stream))
            .file_name(file_name);
        multipart_form = multipart_form.part("mask", file_part);
    }
    multipart_form = multipart_form.text("prompt", p_form_prompt.to_string());
    if let Some(param_value) = p_form_n {
        multipart_form = multipart_form.text("n", param_value.to_string());
    }
    if let Some(param_value) = p_form_size {
        multipart_form = multipart_form.text("size", param_value.to_string());
    }
    if let Some(param_value) = p_form_quality {
        multipart_form = multipart_form.text("quality", param_value.to_string());
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayImagesResponse`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayImagesResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAiGatewayImageEditError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить ответ модели через OpenAI Responses API, отправьте
/// POST-запрос на `/v1/responses`.  Диалог можно продолжить по
/// `previous_response_id`, долгие запросы — выполнить в фоне с `\"background\":
/// true` и получать результат через `GET /v1/responses/{response_id}`.  С
/// `\"stream\": true` ответ приходит потоком типизированных событий Server-Sent
/// Events (`response.created`, `response.output_text.delta`,
/// `response.completed` и др.), в конце потока — `data: [DONE]`.  Если у ключа
/// настроены запасные модели и запрошенная модель не ответила, запрос
/// повторяется на запасных (заголовок `x-timeweb-attempted-fallbacks`).
pub async fn create_ai_gateway_response(
    configuration: &configuration::Configuration,
    ai_gateway_response_create_request: models::AiGatewayResponseCreateRequest
) -> Result<models::AiGatewayResponse, Error<CreateAiGatewayResponseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_ai_gateway_response_create_request = ai_gateway_response_create_request;

    let uri_str = format!("{}/v1/responses", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_ai_gateway_response_create_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayResponse`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAiGatewayResponseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы озвучить текст, отправьте POST-запрос на `/v1/audio/speech`.
/// Используйте модель с `mode: audio_speech` из `GET /v1/models`. Ответ —
/// аудиофайл в формате `response_format` (по умолчанию `mp3`).
pub async fn create_ai_gateway_speech(
    configuration: &configuration::Configuration,
    ai_gateway_speech_request: models::AiGatewaySpeechRequest
) -> Result<reqwest::Response, Error<CreateAiGatewaySpeechError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_ai_gateway_speech_request = ai_gateway_speech_request;

    let uri_str = format!("{}/v1/audio/speech", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_ai_gateway_speech_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAiGatewaySpeechError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы распознать речь в аудиофайле, отправьте POST-запрос на
/// `/v1/audio/transcriptions` (multipart/form-data).  Доступно для моделей
/// распознавания речи `*-transcribe` (например,
/// `openai/gpt-4o-mini-transcribe`). Для других моделей, в том числе
/// неизвестных, возвращается ошибка 405. Ответ возвращается в JSON.
pub async fn create_ai_gateway_transcription(
    configuration: &configuration::Configuration,
    file: std::path::PathBuf,
    model: &str,
    language: Option<&str>,
    prompt: Option<&str>,
    response_format: Option<&str>,
    temperature: Option<f64>
) -> Result<models::AiGatewayTranscription, Error<CreateAiGatewayTranscriptionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_form_file = file;
    let p_form_model = model;
    let p_form_language = language;
    let p_form_prompt = prompt;
    let p_form_response_format = response_format;
    let p_form_temperature = temperature;

    let uri_str = format!("{}/v1/audio/transcriptions", configuration.base_path);
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
    let file = TokioFile::open(&p_form_file).await?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_name = p_form_file
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let file_part =
        reqwest::multipart::Part::stream(reqwest::Body::wrap_stream(stream)).file_name(file_name);
    multipart_form = multipart_form.part("file", file_part);
    multipart_form = multipart_form.text("model", p_form_model.to_string());
    if let Some(param_value) = p_form_language {
        multipart_form = multipart_form.text("language", param_value.to_string());
    }
    if let Some(param_value) = p_form_prompt {
        multipart_form = multipart_form.text("prompt", param_value.to_string());
    }
    if let Some(param_value) = p_form_response_format {
        multipart_form = multipart_form.text("response_format", param_value.to_string());
    }
    if let Some(param_value) = p_form_temperature {
        multipart_form = multipart_form.text("temperature", param_value.to_string());
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayTranscription`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayTranscription`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAiGatewayTranscriptionError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить файл, отправьте DELETE-запрос на `/v1/files/{file_id}`.
/// Входной файл батча, который ещё выполняется или отменяется, и результаты
/// батча, расход которого ещё не списан, удалить нельзя — вернётся ошибка 409.
pub async fn delete_ai_gateway_file(
    configuration: &configuration::Configuration,
    file_id: &str
) -> Result<models::AiGatewayFileDeleted, Error<DeleteAiGatewayFileError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_file_id = file_id;

    let uri_str = format!(
        "{}/v1/files/{file_id}",
        configuration.base_path,
        file_id = crate::apis::urlencode(p_path_file_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayFileDeleted`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayFileDeleted`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteAiGatewayFileError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы удалить сохранённый ответ, отправьте DELETE-запрос на
/// `/v1/responses/{response_id}`.  Неизвестный или уже удалённый `response_id`
/// возвращает ошибку 400.
pub async fn delete_ai_gateway_response(
    configuration: &configuration::Configuration,
    response_id: &str
) -> Result<models::AiGatewayResponseDeleted, Error<DeleteAiGatewayResponseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_response_id = response_id;

    let uri_str = format!(
        "{}/v1/responses/{response_id}",
        configuration.base_path,
        response_id = crate::apis::urlencode(p_path_response_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayResponseDeleted`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayResponseDeleted`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteAiGatewayResponseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы скачать содержимое файла — например, результаты выполненного батча
/// (`output_file_id`), — отправьте GET-запрос на `/v1/files/{file_id}/content`.
/// Перед скачиванием результатов получите батч через `GET
/// /v1/batches/{batch_id}` и убедитесь, что он в статусе `completed`. Файлы с
/// `purpose=user_data` скачать нельзя — вернётся ошибка 400.
pub async fn download_ai_gateway_file_content(
    configuration: &configuration::Configuration,
    file_id: &str
) -> Result<reqwest::Response, Error<DownloadAiGatewayFileContentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_file_id = file_id;

    let uri_str = format!(
        "{}/v1/files/{file_id}/content",
        configuration.base_path,
        file_id = crate::apis::urlencode(p_path_file_id)
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
        let entity: Option<DownloadAiGatewayFileContentError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить статус батча, отправьте GET-запрос на
/// `/v1/batches/{batch_id}`.  Когда статус станет `completed`, скачайте
/// результаты по `output_file_id` через `GET /v1/files/{file_id}/content`.
pub async fn get_ai_gateway_batch(
    configuration: &configuration::Configuration,
    batch_id: &str
) -> Result<models::AiGatewayBatch, Error<GetAiGatewayBatchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_batch_id = batch_id;

    let uri_str = format!(
        "{}/v1/batches/{batch_id}",
        configuration.base_path,
        batch_id = crate::apis::urlencode(p_path_batch_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayBatch`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayBatch`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAiGatewayBatchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список батчей, отправьте GET-запрос на `/v1/batches`.  Для
/// следующей страницы передайте в `after` значение `last_id`, пока `has_more`
/// равно `true`.
pub async fn get_ai_gateway_batches(
    configuration: &configuration::Configuration,
    limit: Option<i32>,
    after: Option<&str>
) -> Result<models::AiGatewayBatchList, Error<GetAiGatewayBatchesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_limit = limit;
    let p_query_after = after;

    let uri_str = format!("{}/v1/batches", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayBatchList`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayBatchList`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAiGatewayBatchesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить информацию о загруженном файле, отправьте GET-запрос на
/// `/v1/files/{file_id}`.
pub async fn get_ai_gateway_file(
    configuration: &configuration::Configuration,
    file_id: &str
) -> Result<models::AiGatewayFile, Error<GetAiGatewayFileError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_file_id = file_id;

    let uri_str = format!(
        "{}/v1/files/{file_id}",
        configuration.base_path,
        file_id = crate::apis::urlencode(p_path_file_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayFile`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayFile`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAiGatewayFileError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить список моделей, доступных ключу, отправьте GET-запрос на `/v1/models`.  Поле `mode` показывает назначение модели. В списке могут оставаться модели, снятые с обслуживания: запросы к ним возвращают ошибку 410. Актуальный статус, цены и возможности моделей — в каталоге `GET https://api.timeweb.cloud/api/v3/cloud-ai/models` (снятые модели — `is_stopped: true`).
pub async fn get_ai_gateway_models(
    configuration: &configuration::Configuration
) -> Result<models::AiGatewayModelList, Error<GetAiGatewayModelsError>> {
    let uri_str = format!("{}/v1/models", configuration.base_path);
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayModelList`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayModelList`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAiGatewayModelsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы получить сохранённый ответ — например, дождаться завершения фонового
/// ответа, — отправьте GET-запрос на `/v1/responses/{response_id}`.
/// Неизвестный или удалённый `response_id` возвращает ошибку 400.
pub async fn get_ai_gateway_response(
    configuration: &configuration::Configuration,
    response_id: &str
) -> Result<models::AiGatewayResponse, Error<GetAiGatewayResponseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_response_id = response_id;

    let uri_str = format!(
        "{}/v1/responses/{response_id}",
        configuration.base_path,
        response_id = crate::apis::urlencode(p_path_response_id)
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayResponse`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAiGatewayResponseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}

/// Чтобы загрузить файл, отправьте POST-запрос на `/v1/files`
/// (multipart/form-data).  Файл загружается для конкретной модели, поэтому
/// `model` (или `target_model_names`) обязателен: - `purpose=batch` — входной
/// JSONL-файл Batch API. Все строки должны обращаться к одной модели — той, что
/// указана в `model`, иначе вернётся 400. Размер — до 50 МБ и до 50 000
/// запросов; - `user_data` и другие назначения OpenAI — файлы, на которые можно
/// сослаться по `file_id` в сообщениях.  Загрузка бесплатна. Сохраняйте `id`
/// загруженных файлов: получить список файлов нельзя (`GET /v1/files`
/// возвращает 405).
pub async fn upload_ai_gateway_file(
    configuration: &configuration::Configuration,
    file: std::path::PathBuf,
    purpose: &str,
    model: Option<&str>,
    target_model_names: Option<&str>
) -> Result<models::AiGatewayFile, Error<UploadAiGatewayFileError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_form_file = file;
    let p_form_purpose = purpose;
    let p_form_model = model;
    let p_form_target_model_names = target_model_names;

    let uri_str = format!("{}/v1/files", configuration.base_path);
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
    let file = TokioFile::open(&p_form_file).await?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_name = p_form_file
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let file_part =
        reqwest::multipart::Part::stream(reqwest::Body::wrap_stream(stream)).file_name(file_name);
    multipart_form = multipart_form.part("file", file_part);
    multipart_form = multipart_form.text("purpose", p_form_purpose.to_string());
    if let Some(param_value) = p_form_model {
        multipart_form = multipart_form.text("model", param_value.to_string());
    }
    if let Some(param_value) = p_form_target_model_names {
        multipart_form = multipart_form.text("target_model_names", param_value.to_string());
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
                    "Received `text/plain` content type response that cannot be converted to `models::AiGatewayFile`"
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AiGatewayFile`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UploadAiGatewayFileError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
}
