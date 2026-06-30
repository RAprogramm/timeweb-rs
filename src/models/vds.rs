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

/// Vds : Сервер
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vds {
    /// ID для каждого экземпляра сервера. Автоматически генерируется при
    /// создании.
    #[serde(rename = "id")]
    pub id:                f64,
    /// Удобочитаемое имя, установленное для сервера.
    #[serde(rename = "name")]
    pub name:              String,
    /// Комментарий к серверу.
    #[serde(rename = "comment")]
    pub comment:           String,
    /// Дата создания сервера в формате ISO8061.
    #[serde(rename = "created_at")]
    pub created_at:        String,
    #[serde(rename = "os")]
    pub os:                Box<models::VdsOs>,
    #[serde(rename = "software", deserialize_with = "Option::deserialize")]
    pub software:          Option<Box<models::VdsSoftware>>,
    /// ID тарифа сервера.
    #[serde(rename = "preset_id", deserialize_with = "Option::deserialize")]
    pub preset_id:         Option<f64>,
    /// Локация сервера.
    #[serde(rename = "location")]
    pub location:          String,
    /// ID конфигуратора сервера.
    #[serde(rename = "configurator_id", deserialize_with = "Option::deserialize")]
    pub configurator_id:   Option<f64>,
    /// Режим загрузки ОС сервера.
    #[serde(rename = "boot_mode")]
    pub boot_mode:         BootMode,
    /// Статус сервера.
    #[serde(rename = "status")]
    pub status:            Status,
    /// Значение времени, указанное в комбинированном формате даты и времени
    /// ISO8601, которое представляет, когда был запущен сервер.
    #[serde(rename = "start_at", deserialize_with = "Option::deserialize")]
    pub start_at:          Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Это логическое значение, которое показывает, включена ли защита от DDoS
    /// у данного сервера.
    #[serde(rename = "is_ddos_guard")]
    pub is_ddos_guard:     bool,
    /// Это логическое значение, которое показывает, доступно ли подключение по
    /// SSH для поддержки.
    #[serde(rename = "is_master_ssh")]
    pub is_master_ssh:     bool,
    /// Это логическое значение, которое показывает, является ли CPU выделенным.
    #[serde(rename = "is_dedicated_cpu")]
    pub is_dedicated_cpu:  bool,
    /// Количество видеокарт сервера.
    #[serde(rename = "gpu")]
    pub gpu:               f64,
    /// Количество ядер процессора сервера.
    #[serde(rename = "cpu")]
    pub cpu:               f64,
    /// Частота ядер процессора сервера.
    #[serde(rename = "cpu_frequency")]
    pub cpu_frequency:     String,
    /// Размер (в Мб) ОЗУ сервера.
    #[serde(rename = "ram")]
    pub ram:               f64,
    /// Список дисков сервера.
    #[serde(rename = "disks")]
    pub disks:             Vec<models::VdsDisksInner>,
    /// ID аватара сервера.
    #[serde(rename = "avatar_id", deserialize_with = "Option::deserialize")]
    pub avatar_id:         Option<String>,
    /// Ссылка на аватар сервера.
    #[serde(rename = "avatar_link", deserialize_with = "Option::deserialize")]
    pub avatar_link:       Option<String>,
    /// Пароль от VNC.
    #[serde(rename = "vnc_pass")]
    pub vnc_pass:          String,
    /// Пароль root сервера или пароль Администратора для серверов Windows.
    #[serde(rename = "root_pass", deserialize_with = "Option::deserialize")]
    pub root_pass:         Option<String>,
    #[serde(rename = "image", deserialize_with = "Option::deserialize")]
    pub image:             Option<Box<models::VdsImage>>,
    /// Список сетей сервера.
    #[serde(rename = "networks")]
    pub networks:          Vec<models::VdsNetworksInner>,
    /// Cloud-init скрипт.
    #[serde(rename = "cloud_init", deserialize_with = "Option::deserialize")]
    pub cloud_init:        Option<String>,
    /// Это логическое значение, которое показывает, включен ли QEMU-agent на
    /// сервере.
    #[serde(rename = "is_qemu_agent")]
    pub is_qemu_agent:     bool,
    #[serde(rename = "availability_zone")]
    pub availability_zone: models::AvailabilityZone
}

impl Vds {
    /// Сервер
    pub fn new(
        id: f64,
        name: String,
        comment: String,
        created_at: String,
        os: models::VdsOs,
        software: Option<models::VdsSoftware>,
        preset_id: Option<f64>,
        location: String,
        configurator_id: Option<f64>,
        boot_mode: BootMode,
        status: Status,
        start_at: Option<chrono::DateTime<chrono::FixedOffset>>,
        is_ddos_guard: bool,
        is_master_ssh: bool,
        is_dedicated_cpu: bool,
        gpu: f64,
        cpu: f64,
        cpu_frequency: String,
        ram: f64,
        disks: Vec<models::VdsDisksInner>,
        avatar_id: Option<String>,
        avatar_link: Option<String>,
        vnc_pass: String,
        root_pass: Option<String>,
        image: Option<models::VdsImage>,
        networks: Vec<models::VdsNetworksInner>,
        cloud_init: Option<String>,
        is_qemu_agent: bool,
        availability_zone: models::AvailabilityZone
    ) -> Vds {
        Vds {
            id,
            name,
            comment,
            created_at,
            os: Box::new(os),
            software: if let Some(x) = software {
                Some(Box::new(x))
            } else {
                None
            },
            preset_id,
            location,
            configurator_id,
            boot_mode,
            status,
            start_at,
            is_ddos_guard,
            is_master_ssh,
            is_dedicated_cpu,
            gpu,
            cpu,
            cpu_frequency,
            ram,
            disks,
            avatar_id,
            avatar_link,
            vnc_pass,
            root_pass,
            image: if let Some(x) = image {
                Some(Box::new(x))
            } else {
                None
            },
            networks,
            cloud_init,
            is_qemu_agent,
            availability_zone
        }
    }
}
/// Режим загрузки ОС сервера.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BootMode {
    #[serde(rename = "std")]
    Std,
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "cd")]
    Cd
}

impl Default for BootMode {
    fn default() -> BootMode {
        Self::Std
    }
}
/// Статус сервера.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "installing")]
    Installing,
    #[serde(rename = "software_install")]
    SoftwareInstall,
    #[serde(rename = "reinstalling")]
    Reinstalling,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "turning_on")]
    TurningOn,
    #[serde(rename = "turning_off")]
    TurningOff,
    #[serde(rename = "hard_turning_off")]
    HardTurningOff,
    #[serde(rename = "rebooting")]
    Rebooting,
    #[serde(rename = "hard_rebooting")]
    HardRebooting,
    #[serde(rename = "removing")]
    Removing,
    #[serde(rename = "removed")]
    Removed,
    #[serde(rename = "cloning")]
    Cloning,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "configuring")]
    Configuring,
    #[serde(rename = "no_paid")]
    NoPaid,
    #[serde(rename = "permanent_blocked")]
    PermanentBlocked
}

impl Default for Status {
    fn default() -> Status {
        Self::Installing
    }
}
