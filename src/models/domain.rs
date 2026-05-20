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

/// Domain : Домен
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    /// Допустимые периоды продления домена.
    #[serde(rename = "allowed_buy_periods")]
    pub allowed_buy_periods: Vec<models::DomainAllowedBuyPeriodsInner>,
    /// Количество дней, оставшихся до конца срока регистрации домена.
    #[serde(rename = "days_left")]
    pub days_left: f64,
    /// Статус домена.
    #[serde(rename = "domain_status")]
    pub domain_status: DomainStatus,
    /// Дата окончания срока регистрации домена, для доменов без срока окончания
    /// регистрации будет приходить 0000-00-00.
    #[serde(rename = "expiration")]
    pub expiration: String,
    /// Полное имя домена.
    #[serde(rename = "fqdn")]
    pub fqdn: String,
    /// ID домена.
    #[serde(rename = "id")]
    pub id: f64,
    /// Ссылка на аватар домена.
    #[serde(rename = "avatar_link", deserialize_with = "Option::deserialize")]
    pub avatar_link: Option<String>,
    /// Это логическое значение, которое показывает, включено ли автопродление
    /// домена.
    #[serde(
        rename = "is_autoprolong_enabled",
        deserialize_with = "Option::deserialize"
    )]
    pub is_autoprolong_enabled: Option<bool>,
    /// Это логическое значение, которое показывает, является ли домен
    /// премиальным.
    #[serde(rename = "is_premium")]
    pub is_premium: bool,
    /// Это логическое значение, которое показывает, можно ли сейчас продлить
    /// домен.
    #[serde(rename = "is_prolong_allowed")]
    pub is_prolong_allowed: bool,
    /// Это логическое значение, которое показывает, является ли домен
    /// техническим.
    #[serde(rename = "is_technical")]
    pub is_technical: bool,
    /// Это логическое значение, которое показывает, включено ли скрытие данных
    /// администратора домена для whois. Если приходит null, значит для данной
    /// зоны эта услуга не доступна.
    #[serde(
        rename = "is_whois_privacy_enabled",
        deserialize_with = "Option::deserialize"
    )]
    pub is_whois_privacy_enabled: Option<bool>,
    /// Привязанный к домену IP-адрес.
    #[serde(rename = "linked_ip", deserialize_with = "Option::deserialize")]
    pub linked_ip: Option<String>,
    /// До какого числа оплачен домен.
    #[serde(rename = "paid_till", deserialize_with = "Option::deserialize")]
    pub paid_till: Option<String>,
    /// ID администратора, на которого зарегистрирован домен.
    #[serde(rename = "person_id", deserialize_with = "Option::deserialize")]
    pub person_id: Option<f64>,
    /// Стоимость премиального домена.
    #[serde(
        rename = "premium_prolong_cost",
        deserialize_with = "Option::deserialize"
    )]
    pub premium_prolong_cost: Option<f64>,
    /// ID регистратора домена.
    #[serde(rename = "provider", deserialize_with = "Option::deserialize")]
    pub provider: Option<String>,
    /// Статус заявки на продление/регистрацию/трансфер домена.
    #[serde(rename = "request_status", deserialize_with = "Option::deserialize")]
    pub request_status: Option<RequestStatus>,
    /// Список поддоменов.
    #[serde(rename = "subdomains")]
    pub subdomains: Vec<models::Subdomain>,
    /// ID доменной зоны.
    #[serde(rename = "tld_id", deserialize_with = "Option::deserialize")]
    pub tld_id: Option<f64>
}

impl Domain {
    /// Домен
    pub fn new(
        allowed_buy_periods: Vec<models::DomainAllowedBuyPeriodsInner>,
        days_left: f64,
        domain_status: DomainStatus,
        expiration: String,
        fqdn: String,
        id: f64,
        avatar_link: Option<String>,
        is_autoprolong_enabled: Option<bool>,
        is_premium: bool,
        is_prolong_allowed: bool,
        is_technical: bool,
        is_whois_privacy_enabled: Option<bool>,
        linked_ip: Option<String>,
        paid_till: Option<String>,
        person_id: Option<f64>,
        premium_prolong_cost: Option<f64>,
        provider: Option<String>,
        request_status: Option<RequestStatus>,
        subdomains: Vec<models::Subdomain>,
        tld_id: Option<f64>
    ) -> Domain {
        Domain {
            allowed_buy_periods,
            days_left,
            domain_status,
            expiration,
            fqdn,
            id,
            avatar_link,
            is_autoprolong_enabled,
            is_premium,
            is_prolong_allowed,
            is_technical,
            is_whois_privacy_enabled,
            linked_ip,
            paid_till,
            person_id,
            premium_prolong_cost,
            provider,
            request_status,
            subdomains,
            tld_id
        }
    }
}
/// Статус домена.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DomainStatus {
    #[serde(rename = "awaiting_payment")]
    AwaitingPayment,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "final_expired")]
    FinalExpired,
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "no_paid")]
    NoPaid,
    #[serde(rename = "ns_based")]
    NsBased,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "soon_expire")]
    SoonExpire,
    #[serde(rename = "today_expired")]
    TodayExpired
}

impl Default for DomainStatus {
    fn default() -> DomainStatus {
        Self::AwaitingPayment
    }
}
/// Статус заявки на продление/регистрацию/трансфер домена.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequestStatus {
    #[serde(rename = "prolongation_fail")]
    ProlongationFail,
    #[serde(rename = "prolongation_request")]
    ProlongationRequest,
    #[serde(rename = "registration_fail")]
    RegistrationFail,
    #[serde(rename = "registration_request")]
    RegistrationRequest,
    #[serde(rename = "transfer_fail")]
    TransferFail,
    #[serde(rename = "transfer_request")]
    TransferRequest,
    #[serde(rename = "awaiting_person")]
    AwaitingPerson
}

impl Default for RequestStatus {
    fn default() -> RequestStatus {
        Self::ProlongationFail
    }
}
