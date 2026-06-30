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

/// Balancer : Балансировщик
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Balancer {
    /// ID для каждого экземпляра балансировщика. Автоматически генерируется при
    /// создании.
    #[serde(rename = "id")]
    pub id:                  f64,
    /// ID пользователя.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id:          Option<String>,
    /// Алгоритм переключений балансировщика.
    #[serde(rename = "algo")]
    pub algo:                Algo,
    /// Значение времени, указанное в комбинированном формате даты и времени
    /// ISO8601, которое представляет, когда был создан балансировщик.
    #[serde(rename = "created_at")]
    pub created_at:          chrono::DateTime<chrono::FixedOffset>,
    /// Порог количества ошибок.
    #[serde(rename = "fall")]
    pub fall:                f64,
    /// Интервал проверки.
    #[serde(rename = "inter")]
    pub inter:               f64,
    /// IP-адрес сетевого интерфейса IPv4.
    #[serde(rename = "ip", deserialize_with = "Option::deserialize")]
    pub ip:                  Option<String>,
    /// Локальный IP-адрес сетевого интерфейса IPv4.
    #[serde(rename = "local_ip", deserialize_with = "Option::deserialize")]
    pub local_ip:            Option<String>,
    /// Это логическое значение, которое показывает, выдает ли балансировщик
    /// сигнал о проверке жизнеспособности.
    #[serde(rename = "is_keepalive")]
    pub is_keepalive:        bool,
    /// Удобочитаемое имя, установленное для балансировщика.
    #[serde(rename = "name")]
    pub name:                String,
    /// Адрес балансировщика.
    #[serde(rename = "path")]
    pub path:                String,
    /// Порт балансировщика.
    #[serde(rename = "port")]
    pub port:                f64,
    /// Протокол.
    #[serde(rename = "proto")]
    pub proto:               Proto,
    /// Порог количества успешных ответов.
    #[serde(rename = "rise")]
    pub rise:                f64,
    /// Максимальное количество соединений.
    #[serde(rename = "maxconn")]
    pub maxconn:             f64,
    /// Таймаут подключения.
    #[serde(rename = "connect_timeout")]
    pub connect_timeout:     f64,
    /// Таймаут клиента.
    #[serde(rename = "client_timeout")]
    pub client_timeout:      f64,
    /// Таймаут сервера.
    #[serde(rename = "server_timeout")]
    pub server_timeout:      f64,
    /// Таймаут HTTP запроса.
    #[serde(rename = "httprequest_timeout")]
    pub httprequest_timeout: f64,
    /// ID тарифа.
    #[serde(rename = "preset_id")]
    pub preset_id:           f64,
    /// Это логическое значение, которое показывает, требуется ли
    /// перенаправление на SSL.
    #[serde(rename = "is_ssl")]
    pub is_ssl:              bool,
    /// Статус балансировщика.
    #[serde(rename = "status")]
    pub status:              Status,
    /// Это логическое значение, которое показывает, сохраняется ли сессия.
    #[serde(rename = "is_sticky")]
    pub is_sticky:           bool,
    /// Таймаут ответа балансировщика.
    #[serde(rename = "timeout")]
    pub timeout:             f64,
    /// Ссылка на аватар балансировщика.
    #[serde(rename = "avatar_link", deserialize_with = "Option::deserialize")]
    pub avatar_link:         Option<String>,
    /// Это логическое значение, которое показывает, выступает ли балансировщик
    /// в качестве прокси.
    #[serde(rename = "is_use_proxy")]
    pub is_use_proxy:        bool,
    #[serde(rename = "rules")]
    pub rules:               Vec<models::Rule>,
    /// Список IP-адресов, привязанных к балансировщику
    #[serde(rename = "ips")]
    pub ips:                 Vec<String>,
    /// Географическое расположение балансировщика
    #[serde(rename = "location")]
    pub location:            String,
    #[serde(rename = "availability_zone")]
    pub availability_zone:   models::AvailabilityZone,
    /// ID проекта
    #[serde(rename = "project_id")]
    pub project_id:          i32,
    /// Список сетей сервера.
    #[serde(rename = "networks")]
    pub networks:            Vec<models::BalancerNetworksInner>
}

impl Balancer {
    /// Балансировщик
    pub fn new(
        id: f64,
        algo: Algo,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        fall: f64,
        inter: f64,
        ip: Option<String>,
        local_ip: Option<String>,
        is_keepalive: bool,
        name: String,
        path: String,
        port: f64,
        proto: Proto,
        rise: f64,
        maxconn: f64,
        connect_timeout: f64,
        client_timeout: f64,
        server_timeout: f64,
        httprequest_timeout: f64,
        preset_id: f64,
        is_ssl: bool,
        status: Status,
        is_sticky: bool,
        timeout: f64,
        avatar_link: Option<String>,
        is_use_proxy: bool,
        rules: Vec<models::Rule>,
        ips: Vec<String>,
        location: String,
        availability_zone: models::AvailabilityZone,
        project_id: i32,
        networks: Vec<models::BalancerNetworksInner>
    ) -> Balancer {
        Balancer {
            id,
            account_id: None,
            algo,
            created_at,
            fall,
            inter,
            ip,
            local_ip,
            is_keepalive,
            name,
            path,
            port,
            proto,
            rise,
            maxconn,
            connect_timeout,
            client_timeout,
            server_timeout,
            httprequest_timeout,
            preset_id,
            is_ssl,
            status,
            is_sticky,
            timeout,
            avatar_link,
            is_use_proxy,
            rules,
            ips,
            location,
            availability_zone,
            project_id,
            networks
        }
    }
}
/// Алгоритм переключений балансировщика.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Algo {
    #[serde(rename = "roundrobin")]
    Roundrobin,
    #[serde(rename = "leastconn")]
    Leastconn
}

impl Default for Algo {
    fn default() -> Algo {
        Self::Roundrobin
    }
}
/// Протокол.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Proto {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "http2")]
    Http2,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "tcp")]
    Tcp
}

impl Default for Proto {
    fn default() -> Proto {
        Self::Http
    }
}
/// Статус балансировщика.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "stoped")]
    Stoped,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "no_paid")]
    NoPaid
}

impl Default for Status {
    fn default() -> Status {
        Self::Started
    }
}
