/*
 * Документация публичного API
 *
 * # Введение API Timeweb Cloud позволяет вам управлять ресурсами в облаке программным способом с использованием обычных HTTP-запросов.  Множество функций, которые доступны в панели управления Timeweb Cloud, также доступны через API, что позволяет вам автоматизировать ваши собственные сценарии.  В этой документации сперва будет описан общий дизайн и принципы работы API, а после этого конкретные конечные точки. Также будут приведены примеры запросов к ним.   ## Запросы Запросы должны выполняться по протоколу `HTTPS`, чтобы гарантировать шифрование транзакций. Поддерживаются следующие методы запроса: |Метод|Применение| |--- |--- | |GET|Извлекает данные о коллекциях и отдельных ресурсах.| |POST|Для коллекций создает новый ресурс этого типа. Также используется для выполнения действий с конкретным ресурсом.| |PUT|Обновляет существующий ресурс.| |PATCH|Некоторые ресурсы поддерживают частичное обновление, то есть обновление только части атрибутов ресурса, в этом случае вместо метода PUT будет использован PATCH.| |DELETE|Удаляет ресурс.|  Методы `POST`, `PUT` и `PATCH` могут включать объект в тело запроса с типом содержимого `application/json`.  ### Параметры в запросах Некоторые коллекции поддерживают пагинацию, поиск или сортировку в запросах. В параметрах запроса требуется передать: - `limit` — обозначает количество записей, которое необходимо вернуть  - `offset` — указывает на смещение, относительно начала списка  - `search` — позволяет указать набор символов для поиска  - `sort` — можно задать правило сортировки коллекции  ## Ответы Запросы вернут один из следующих кодов состояния ответа HTTP:  |Статус|Описание| |--- |--- | |200 OK|Действие с ресурсом было выполнено успешно.| |201 Created|Ресурс был успешно создан. При этом ресурс может быть как уже готовым к использованию, так и находиться в процессе запуска.| |204 No Content|Действие с ресурсом было выполнено успешно, и ответ не содержит дополнительной информации в теле.| |400 Bad Request|Был отправлен неверный запрос, например, в нем отсутствуют обязательные параметры и т. д. Тело ответа будет содержать дополнительную информацию об ошибке.| |401 Unauthorized|Ошибка аутентификации.| |403 Forbidden|Аутентификация прошла успешно, но недостаточно прав для выполнения действия.| |404 Not Found|Запрашиваемый ресурс не найден.| |409 Conflict|Запрос конфликтует с текущим состоянием.| |423 Locked|Ресурс из запроса заблокирован от применения к нему указанного метода.| |429 Too Many Requests|Был достигнут лимит по количеству запросов в единицу времени.| |500 Internal Server Error|При выполнении запроса произошла какая-то внутренняя ошибка. Чтобы решить эту проблему, лучше всего создать тикет в панели управления.|  ### Структура успешного ответа Все конечные точки будут возвращать данные в формате `JSON`. Ответы на `GET`-запросы будут иметь на верхнем уровне следующую структуру атрибутов:  |Название поля|Тип|Описание| |--- |--- |--- | |[entity_name]|object, object[], string[], number[], boolean|Динамическое поле, которое будет меняться в зависимости от запрашиваемого ресурса и будет содержать все атрибуты, необходимые для описания этого ресурса. Например, при запросе списка баз данных будет возвращаться поле `dbs`, а при запросе конкретного облачного сервера `server`. Для некоторых конечных точек в ответе может возвращаться сразу несколько ресурсов.| |meta|object|Опционально. Объект, который содержит вспомогательную информацию о ресурсе. Чаще всего будет встречаться при запросе коллекций и содержать поле `total`, которое будет указывать на количество элементов в коллекции.| |response_id|string|Опционально. В большинстве случаев в ответе будет содержаться ID ответа в формате UUIDv4, который однозначно указывает на ваш запрос внутри нашей системы. Если вам потребуется задать вопрос нашей поддержке, приложите к вопросу этот ID— так мы сможем найти ответ на него намного быстрее. Также вы можете использовать этот ID, чтобы убедиться, что это новый ответ на запрос и результат не был получен из кэша.|  Пример запроса на получение списка SSH-ключей: ```     HTTP/2.0 200 OK     {       \"ssh_keys\":[           {             \"body\":\"ssh-rsa AAAAB3NzaC1sdfghjkOAsBwWhs= example@device.local\",             \"created_at\":\"2021-09-15T19:52:27Z\",             \"expired_at\":null,             \"id\":5297,             \"is_default\":false,             \"name\":\"example@device.local\",             \"used_at\":null,             \"used_by\":[]           }       ],       \"meta\":{           \"total\":1       },       \"response_id\":\"94608d15-8672-4eed-8ab6-28bd6fa3cdf7\"     } ```  ### Структура ответа с ошибкой |Название поля|Тип|Описание| |--- |--- |--- | |status_code|number|Короткий числовой идентификатор ошибки.| |error_code|string|Короткий текстовый идентификатор ошибки, который уточняет числовой идентификатор и удобен для программной обработки. Самый простой пример — это код `not_found` для ошибки 404.| |message|string, string[]|Опционально. В большинстве случаев в ответе будет содержаться человекочитаемое подробное описание ошибки или ошибок, которые помогут понять, что нужно исправить.| |response_id|string|Опционально. В большинстве случае в ответе будет содержаться ID ответа в формате UUIDv4, который однозначно указывает на ваш запрос внутри нашей системы. Если вам потребуется задать вопрос нашей поддержке, приложите к вопросу этот ID — так мы сможем найти ответ на него намного быстрее.|  Пример: ```     HTTP/2.0 403 Forbidden     {       \"status_code\": 403,       \"error_code\":  \"forbidden\",       \"message\":     \"You do not have access for the attempted action\",       \"response_id\": \"94608d15-8672-4eed-8ab6-28bd6fa3cdf7\"     } ```  ## Статусы ресурсов Важно учесть, что при создании большинства ресурсов внутри платформы вам будет сразу возвращен ответ от сервера со статусом `200 OK` или `201 Created` и ID созданного ресурса в теле ответа, но при этом этот ресурс может быть ещё в *состоянии запуска*.  Для того чтобы понять, в каком состоянии сейчас находится ваш ресурс, мы добавили поле `status` в ответ на получение информации о ресурсе.  Список статусов будет отличаться в зависимости от типа ресурса. Увидеть поддерживаемый список статусов вы сможете в описании каждого конкретного ресурса.     ## Ограничение скорости запросов (Rate Limiting) Чтобы обеспечить стабильность для всех пользователей, Timeweb Cloud защищает API от всплесков входящего трафика, анализируя количество запросов c каждого аккаунта к каждой конечной точке.  Если ваше приложение отправляет более 20 запросов в секунду на одну конечную точку, то для этого запроса API может вернуть код состояния HTTP `429 Too Many Requests`.   ## Аутентификация Доступ к API осуществляется с помощью JWT-токена. Токенами можно управлять внутри панели управления Timeweb Cloud в разделе *API и Terraform*.  Токен необходимо передавать в заголовке каждого запроса в формате: ```   Authorization: Bearer $TIMEWEB_CLOUD_TOKEN ```  ## Формат примеров API Примеры в этой документации описаны с помощью `curl`, HTTP-клиента командной строки. На компьютерах `Linux` и `macOS` обычно по умолчанию установлен `curl`, и он доступен для загрузки на всех популярных платформах, включая `Windows`.  Каждый пример разделен на несколько строк символом `\\`, который совместим с `bash`. Типичный пример выглядит так: ```   curl -X PATCH      -H \"Content-Type: application/json\"      -H \"Authorization: Bearer $TIMEWEB_CLOUD_TOKEN\"      -d '{\"name\":\"Cute Corvus\",\"comment\":\"Development Server\"}'      \"https://api.timeweb.cloud/api/v1/dedicated/1051\" ``` - Параметр `-X` задает метод запроса. Для согласованности метод будет указан во всех примерах, даже если он явно не требуется для методов `GET`. - Строки `-H` задают требуемые HTTP-заголовки. - Примеры, для которых требуется объект JSON в теле запроса, передают требуемые данные через параметр `-d`.  Чтобы использовать приведенные примеры, не подставляя каждый раз в них свой токен, вы можете добавить токен один раз в переменные окружения в вашей консоли. Например, на `Linux` это можно сделать с помощью команды:  ``` TIMEWEB_CLOUD_TOKEN=\"token\" ```  После этого токен будет автоматически подставляться в ваши запросы.  Обратите внимание, что все значения в этой документации являются примерами. Не полагайтесь на IDы операционных систем, тарифов и т.д., используемые в примерах. Используйте соответствующую конечную точку для получения значений перед созданием ресурсов.   ## Версионирование API построено согласно принципам [семантического версионирования](https://semver.org/lang/ru). Это значит, что мы гарантируем обратную совместимость всех изменений в пределах одной мажорной версии.  Мажорная версия каждой конечной точки обозначается в пути запроса, например, запрос `/api/v1/servers` указывает, что этот метод имеет версию 1.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: info@timeweb.cloud
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

/// KafkaConfigParameters : Настройки топика Kafka. Все значения возвращаются в
/// виде строк. Не заданные явно параметры возвращаются со значениями по
/// умолчанию.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaConfigParameters {
    /// Количество партиций топика.
    #[serde(rename = "partitions", skip_serializing_if = "Option::is_none")]
    pub partitions: Option<String>,
    /// Политика очистки старых сегментов лога: `delete` — удалять, `compact` —
    /// уплотнять.
    #[serde(rename = "cleanup_policy", skip_serializing_if = "Option::is_none")]
    pub cleanup_policy: Option<CleanupPolicy>,
    /// Тип сжатия сообщений в топике.
    #[serde(rename = "compression_type", skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<CompressionType>,
    /// Время (в мс) хранения меток удаления для уплотняемых топиков.
    #[serde(
        rename = "delete_retention_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_retention_ms: Option<String>,
    /// Задержка (в мс) перед удалением файла из файловой системы.
    #[serde(
        rename = "file_delete_delay_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_delete_delay_ms: Option<String>,
    /// Количество сообщений, после которого данные принудительно сбрасываются
    /// на диск.
    #[serde(rename = "flush_messages", skip_serializing_if = "Option::is_none")]
    pub flush_messages: Option<String>,
    /// Интервал (в мс), после которого данные принудительно сбрасываются на
    /// диск.
    #[serde(rename = "flush_ms", skip_serializing_if = "Option::is_none")]
    pub flush_ms: Option<String>,
    /// Интервал (в байтах), с которым Kafka добавляет запись в индекс смещений.
    #[serde(
        rename = "index_interval_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub index_interval_bytes: Option<String>,
    /// Минимальное время (в мс), в течение которого сообщение остается
    /// неуплотненным.
    #[serde(
        rename = "min_compaction_lag_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_compaction_lag_ms: Option<String>,
    /// Максимальное время (в мс), в течение которого сообщение может оставаться
    /// неуплотненным.
    #[serde(
        rename = "max_compaction_lag_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_compaction_lag_ms: Option<String>,
    /// Максимальный размер (в байтах) пакета сообщений.
    #[serde(rename = "max_message_bytes", skip_serializing_if = "Option::is_none")]
    pub max_message_bytes: Option<String>,
    /// Версия формата сообщений, в котором Kafka добавляет сообщения в лог.
    #[serde(
        rename = "message_format_version",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_format_version: Option<String>,
    /// Максимально допустимая разница (в мс) между временной меткой сообщения и
    /// временем его получения брокером.
    #[serde(
        rename = "message_timestamp_difference_max_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_timestamp_difference_max_ms: Option<String>,
    /// Понижение версии формата сообщений для старых клиентов.
    #[serde(
        rename = "message_downconversion_enable",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_downconversion_enable: Option<MessageDownconversionEnable>,
    /// Источник временной метки сообщения: `CreateTime` — время создания
    /// сообщения клиентом, `LogAppendTime` — время добавления сообщения в лог
    /// брокером.
    #[serde(
        rename = "message_timestamp_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_timestamp_type: Option<MessageTimestampType>,
    /// Доля неуплотненных данных в логе, при которой запускается уплотнение.
    #[serde(
        rename = "min_cleanable_dirty_ratio",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_cleanable_dirty_ratio: Option<String>,
    /// Минимальное количество синхронизированных реплик, необходимое для
    /// подтверждения записи.
    #[serde(
        rename = "min_insync_replicas",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_insync_replicas: Option<String>,
    /// Предварительное выделение места на диске при создании нового сегмента
    /// лога.
    #[serde(rename = "preallocate", skip_serializing_if = "Option::is_none")]
    pub preallocate: Option<Preallocate>,
    /// Максимальный размер (в байтах) партиции топика, после которого старые
    /// сегменты удаляются. `-1` — без ограничения.
    #[serde(rename = "retention_bytes", skip_serializing_if = "Option::is_none")]
    pub retention_bytes: Option<String>,
    /// Время (в мс) хранения сообщений в топике. `-1` — хранить бессрочно.
    #[serde(rename = "retention_ms", skip_serializing_if = "Option::is_none")]
    pub retention_ms: Option<String>,
    /// Максимальный размер (в байтах) одного сегмента лога.
    #[serde(rename = "segment_bytes", skip_serializing_if = "Option::is_none")]
    pub segment_bytes: Option<String>,
    /// Максимальный размер (в байтах) индексного файла сегмента лога.
    #[serde(
        rename = "segment_index_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub segment_index_bytes: Option<String>,
    /// Максимальное случайное отклонение (в мс) от времени ротации сегмента.
    #[serde(rename = "segment_jitter_ms", skip_serializing_if = "Option::is_none")]
    pub segment_jitter_ms: Option<String>,
    /// Период (в мс), после которого Kafka создает новый сегмент лога.
    #[serde(rename = "segment_ms", skip_serializing_if = "Option::is_none")]
    pub segment_ms: Option<String>,
    /// Возможность выбрать лидером партиции реплику, которая не входит в число
    /// синхронизированных.
    #[serde(
        rename = "unclean_leader_election_enable",
        skip_serializing_if = "Option::is_none"
    )]
    pub unclean_leader_election_enable: Option<UncleanLeaderElectionEnable>
}

impl KafkaConfigParameters {
    /// Настройки топика Kafka. Все значения возвращаются в виде строк. Не
    /// заданные явно параметры возвращаются со значениями по умолчанию.
    pub fn new() -> KafkaConfigParameters {
        KafkaConfigParameters {
            partitions: None,
            cleanup_policy: None,
            compression_type: None,
            delete_retention_ms: None,
            file_delete_delay_ms: None,
            flush_messages: None,
            flush_ms: None,
            index_interval_bytes: None,
            min_compaction_lag_ms: None,
            max_compaction_lag_ms: None,
            max_message_bytes: None,
            message_format_version: None,
            message_timestamp_difference_max_ms: None,
            message_downconversion_enable: None,
            message_timestamp_type: None,
            min_cleanable_dirty_ratio: None,
            min_insync_replicas: None,
            preallocate: None,
            retention_bytes: None,
            retention_ms: None,
            segment_bytes: None,
            segment_index_bytes: None,
            segment_jitter_ms: None,
            segment_ms: None,
            unclean_leader_election_enable: None
        }
    }
}
/// Политика очистки старых сегментов лога: `delete` — удалять, `compact` —
/// уплотнять.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CleanupPolicy {
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "compact")]
    Compact
}

impl Default for CleanupPolicy {
    fn default() -> CleanupPolicy {
        Self::Delete
    }
}
/// Тип сжатия сообщений в топике.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CompressionType {
    #[serde(rename = "uncompressed")]
    Uncompressed,
    #[serde(rename = "zstd")]
    Zstd,
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(rename = "snappy")]
    Snappy,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "producer")]
    Producer
}

impl Default for CompressionType {
    fn default() -> CompressionType {
        Self::Uncompressed
    }
}
/// Понижение версии формата сообщений для старых клиентов.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageDownconversionEnable {
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "OFF")]
    Off
}

impl Default for MessageDownconversionEnable {
    fn default() -> MessageDownconversionEnable {
        Self::On
    }
}
/// Источник временной метки сообщения: `CreateTime` — время создания сообщения
/// клиентом, `LogAppendTime` — время добавления сообщения в лог брокером.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageTimestampType {
    #[serde(rename = "CreateTime")]
    CreateTime,
    #[serde(rename = "LogAppendTime")]
    LogAppendTime
}

impl Default for MessageTimestampType {
    fn default() -> MessageTimestampType {
        Self::CreateTime
    }
}
/// Предварительное выделение места на диске при создании нового сегмента лога.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Preallocate {
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "OFF")]
    Off
}

impl Default for Preallocate {
    fn default() -> Preallocate {
        Self::On
    }
}
/// Возможность выбрать лидером партиции реплику, которая не входит в число
/// синхронизированных.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UncleanLeaderElectionEnable {
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "OFF")]
    Off
}

impl Default for UncleanLeaderElectionEnable {
    fn default() -> UncleanLeaderElectionEnable {
        Self::On
    }
}
