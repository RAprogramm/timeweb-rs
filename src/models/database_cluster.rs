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

/// DatabaseCluster : Кластер базы данных
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseCluster {
    /// ID для каждого экземпляра базы данных. Автоматически генерируется при
    /// создании.
    #[serde(rename = "id")]
    pub id: i64,
    /// Значение времени, указанное в комбинированном формате даты и времени
    /// ISO8601, которое представляет, когда была создана база данных.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Локация сервера.
    #[serde(rename = "location", deserialize_with = "Option::deserialize")]
    pub location: Option<String>,
    /// Название кластера базы данных.
    #[serde(rename = "name")]
    pub name: String,
    /// Описание кластера базы данных.
    #[serde(rename = "description")]
    pub description: String,
    /// Список сетей кластера базы данных.
    #[serde(rename = "networks")]
    pub networks: Vec<models::DatabaseClusterNetworksInner>,
    /// Использование публичного IPv6-адреса.
    #[serde(rename = "is_enabled_public_ipv6")]
    pub is_enabled_public_ipv6: bool,
    /// Тип базы данных. Список возможных значений шире, чем список типов,
    /// доступных при создании нового кластера.
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<String>,
    /// Тип хеширования кластера базы данных (mysql5 | mysql | postgres).
    #[serde(rename = "hash_type", deserialize_with = "Option::deserialize")]
    pub hash_type: Option<HashType>,
    /// Ссылка на аватар для базы данных.
    #[serde(rename = "avatar_link", deserialize_with = "Option::deserialize")]
    pub avatar_link: Option<String>,
    /// Порт
    #[serde(rename = "port", deserialize_with = "Option::deserialize")]
    pub port: Option<i32>,
    /// Текущий статус кластера базы данных. Значение `read_only` означает, что
    /// запись в кластер заблокирована из-за переполнения диска — чтобы снять
    /// блокировку, освободите место или увеличьте размер диска.
    #[serde(rename = "status")]
    pub status: Status,
    /// ID тарифа. Равен `null` у кластеров, созданных через конфигуратор — в
    /// этом случае заполнен `configurator_id`.
    #[serde(rename = "preset_id", deserialize_with = "Option::deserialize")]
    pub preset_id: Option<i32>,
    /// ID конфигуратора. Равен `null` у кластеров, созданных по тарифу.
    #[serde(rename = "configurator_id", deserialize_with = "Option::deserialize")]
    pub configurator_id: Option<i32>,
    /// Количество ядер процессора.
    #[serde(rename = "cpu", deserialize_with = "Option::deserialize")]
    pub cpu: Option<i32>,
    /// Частота процессора.
    #[serde(rename = "cpu_frequency", deserialize_with = "Option::deserialize")]
    pub cpu_frequency: Option<String>,
    /// Используются ли выделенные ядра процессора.
    #[serde(rename = "is_dedicated_cpu")]
    pub is_dedicated_cpu: bool,
    /// Объем оперативной памяти (в Мб).
    #[serde(rename = "ram", deserialize_with = "Option::deserialize")]
    pub ram: Option<i32>,
    #[serde(rename = "disk", deserialize_with = "Option::deserialize")]
    pub disk: Option<Box<models::DatabaseClusterDisk>>,
    /// Подключен ли к кластеру дополнительный диск.
    #[serde(rename = "has_additional_disk")]
    pub has_additional_disk: bool,
    #[serde(rename = "disk_autoscaling", deserialize_with = "Option::deserialize")]
    pub disk_autoscaling: Option<Box<models::DatabaseClusterDiskAutoscaling>>,
    #[serde(rename = "config_parameters", deserialize_with = "Option::deserialize")]
    pub config_parameters: Option<Box<models::DatabaseClusterConfigParameters>>,
    /// Доступность публичного IP-адреса
    #[serde(rename = "is_enabled_public_network")]
    pub is_enabled_public_network: bool,
    /// Включено ли защищенное подключение к кластеру базы данных.
    #[serde(rename = "is_secure_connection_enabled")]
    pub is_secure_connection_enabled: bool,
    /// Включены ли автоматические резервные копии кластера базы данных.
    #[serde(rename = "is_autobackups_enabled")]
    pub is_autobackups_enabled: bool,
    /// Включено ли расписание резервного копирования кластера базы данных.
    #[serde(rename = "is_backup_schedule_enabled")]
    pub is_backup_schedule_enabled: bool,
    /// Зона доступности кластера базы данных.
    #[serde(rename = "availability_zone", deserialize_with = "Option::deserialize")]
    pub availability_zone: Option<models::AvailabilityZone>,
    /// ID проекта, в котором находится кластер базы данных.
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i32>,
    /// Список реплик кластера базы данных.
    #[serde(rename = "replica_list")]
    pub replica_list: Vec<models::DatabaseClusterReplicaListInner>,
    /// Список доменов кластера базы данных. Если публичная сеть отключена
    /// (`is_enabled_public_network: false`), список всегда пустой.
    #[serde(rename = "domains")]
    pub domains: Vec<models::DatabaseClusterDomainsInner>,
    /// Список дочерних сервисов кластера базы данных.
    #[serde(rename = "child_services")]
    pub child_services: Vec<models::DatabaseClusterChildServicesInner>,
    /// Список родительских сервисов кластера базы данных.
    #[serde(rename = "parent_services")]
    pub parent_services: Vec<models::DatabaseClusterParentServicesInner>,
    #[serde(rename = "maintenance_slot")]
    pub maintenance_slot: Box<models::DatabaseClusterMaintenanceSlot>
}

impl DatabaseCluster {
    /// Кластер базы данных
    pub fn new(
        id: i64,
        created_at: String,
        location: Option<String>,
        name: String,
        description: String,
        networks: Vec<models::DatabaseClusterNetworksInner>,
        is_enabled_public_ipv6: bool,
        r#type: Option<String>,
        hash_type: Option<HashType>,
        avatar_link: Option<String>,
        port: Option<i32>,
        status: Status,
        preset_id: Option<i32>,
        configurator_id: Option<i32>,
        cpu: Option<i32>,
        cpu_frequency: Option<String>,
        is_dedicated_cpu: bool,
        ram: Option<i32>,
        disk: Option<models::DatabaseClusterDisk>,
        has_additional_disk: bool,
        disk_autoscaling: Option<models::DatabaseClusterDiskAutoscaling>,
        config_parameters: Option<models::DatabaseClusterConfigParameters>,
        is_enabled_public_network: bool,
        is_secure_connection_enabled: bool,
        is_autobackups_enabled: bool,
        is_backup_schedule_enabled: bool,
        availability_zone: Option<models::AvailabilityZone>,
        replica_list: Vec<models::DatabaseClusterReplicaListInner>,
        domains: Vec<models::DatabaseClusterDomainsInner>,
        child_services: Vec<models::DatabaseClusterChildServicesInner>,
        parent_services: Vec<models::DatabaseClusterParentServicesInner>,
        maintenance_slot: models::DatabaseClusterMaintenanceSlot
    ) -> DatabaseCluster {
        DatabaseCluster {
            id,
            created_at,
            location,
            name,
            description,
            networks,
            is_enabled_public_ipv6,
            r#type,
            hash_type,
            avatar_link,
            port,
            status,
            preset_id,
            configurator_id,
            cpu,
            cpu_frequency,
            is_dedicated_cpu,
            ram,
            disk: if let Some(x) = disk {
                Some(Box::new(x))
            } else {
                None
            },
            has_additional_disk,
            disk_autoscaling: if let Some(x) = disk_autoscaling {
                Some(Box::new(x))
            } else {
                None
            },
            config_parameters: if let Some(x) = config_parameters {
                Some(Box::new(x))
            } else {
                None
            },
            is_enabled_public_network,
            is_secure_connection_enabled,
            is_autobackups_enabled,
            is_backup_schedule_enabled,
            availability_zone,
            project_id: None,
            replica_list,
            domains,
            child_services,
            parent_services,
            maintenance_slot: Box::new(maintenance_slot)
        }
    }
}
/// Тип хеширования кластера базы данных (mysql5 | mysql | postgres).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HashType {
    #[serde(rename = "caching_sha2")]
    CachingSha2,
    #[serde(rename = "mysql_native")]
    MysqlNative
}

impl Default for HashType {
    fn default() -> HashType {
        Self::CachingSha2
    }
}
/// Текущий статус кластера базы данных. Значение `read_only` означает, что
/// запись в кластер заблокирована из-за переполнения диска — чтобы снять
/// блокировку, освободите место или увеличьте размер диска.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "no_paid")]
    NoPaid,
    #[serde(rename = "lan_transfer")]
    LanTransfer,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "backup_recovery")]
    BackupRecovery,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "rebooting")]
    Rebooting,
    #[serde(rename = "turning_off")]
    TurningOff,
    #[serde(rename = "turning_on")]
    TurningOn,
    #[serde(rename = "read_only")]
    ReadOnly,
    #[serde(rename = "user_transfer")]
    UserTransfer
}

impl Default for Status {
    fn default() -> Status {
        Self::Started
    }
}
