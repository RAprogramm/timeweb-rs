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

/// App : Экземпляр приложения.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct App {
    /// ID для каждого экземпляра приложения. Автоматически генерируется при
    /// создании.
    #[serde(rename = "id")]
    pub id:             f64,
    /// Тип приложения.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type:         Option<Type>,
    /// Удобочитаемое имя, установленное для приложения.
    #[serde(rename = "name")]
    pub name:           String,
    /// Статус приложения.
    #[serde(rename = "status")]
    pub status:         Status,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider:       Option<Box<models::AppProvider>>,
    /// IPv4-адрес приложения.
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip:             Option<String>,
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains:        Option<Vec<models::AppDomainsInner>>,
    #[serde(rename = "framework", skip_serializing_if = "Option::is_none")]
    pub framework:      Option<Box<models::Frameworks>>,
    /// Локация сервера.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location:       Option<String>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository:     Option<Box<models::Repository>>,
    /// Версия окружения.
    #[serde(
        rename = "env_version",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub env_version:    Option<Option<String>>,
    /// Переменные окружения приложения. Объект с ключами и значениями типа
    /// string.
    #[serde(rename = "envs", skip_serializing_if = "Option::is_none")]
    pub envs:           Option<serde_json::Value>,
    /// Название ветки репозитория из которой собрано приложение.
    #[serde(rename = "branch_name", skip_serializing_if = "Option::is_none")]
    pub branch_name:    Option<String>,
    /// Включен ли автоматический деплой.
    #[serde(rename = "is_auto_deploy", skip_serializing_if = "Option::is_none")]
    pub is_auto_deploy: Option<bool>,
    /// Хэш коммита из которого собрано приложеие.
    #[serde(rename = "commit_sha", skip_serializing_if = "Option::is_none")]
    pub commit_sha:     Option<String>,
    /// Комментарий к приложению.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment:        Option<String>,
    /// ID тарифа.
    #[serde(rename = "preset_id", skip_serializing_if = "Option::is_none")]
    pub preset_id:      Option<f64>,
    /// Путь к директории с индексным файлом. Определен для приложений `type:
    /// frontend`. Для приложений `type: backend` всегда null.
    #[serde(
        rename = "index_dir",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub index_dir:      Option<Option<String>>,
    /// Команда сборки приложения.
    #[serde(rename = "build_cmd", skip_serializing_if = "Option::is_none")]
    pub build_cmd:      Option<String>,
    /// Ссылка на аватар приложения.
    #[serde(
        rename = "avatar_link",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avatar_link:    Option<Option<String>>,
    /// Команда для запуска приложения. Определена для приложений `type:
    /// backend`. Для приложений `type: frontend` всегда null.
    #[serde(
        rename = "run_cmd",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub run_cmd:        Option<Option<String>>,
    #[serde(
        rename = "configuration",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration:  Option<Option<Box<models::AppConfiguration>>>,
    #[serde(
        rename = "disk_status",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub disk_status:    Option<Option<Box<models::AppDiskStatus>>>,
    /// Включен ли агент QEMU.
    #[serde(rename = "is_qemu_agent", skip_serializing_if = "Option::is_none")]
    pub is_qemu_agent:  Option<bool>,
    /// Язык программирования приложения.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language:       Option<String>,
    /// Время запуска приложения.
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time:     Option<chrono::DateTime<chrono::FixedOffset>>
}

impl App {
    /// Экземпляр приложения.
    pub fn new(id: f64, name: String, status: Status) -> App {
        App {
            id,
            r#type: None,
            name,
            status,
            provider: None,
            ip: None,
            domains: None,
            framework: None,
            location: None,
            repository: None,
            env_version: None,
            envs: None,
            branch_name: None,
            is_auto_deploy: None,
            commit_sha: None,
            comment: None,
            preset_id: None,
            index_dir: None,
            build_cmd: None,
            avatar_link: None,
            run_cmd: None,
            configuration: None,
            disk_status: None,
            is_qemu_agent: None,
            language: None,
            start_time: None
        }
    }
}
/// Тип приложения.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "backend")]
    Backend,
    #[serde(rename = "frontend")]
    Frontend
}

impl Default for Type {
    fn default() -> Type {
        Self::Backend
    }
}
/// Статус приложения.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "no_paid")]
    NoPaid,
    #[serde(rename = "deploy")]
    Deploy,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "startup_error")]
    StartupError,
    #[serde(rename = "new")]
    New,
    #[serde(rename = "reboot")]
    Reboot
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
