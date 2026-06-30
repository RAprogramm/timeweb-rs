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

/// DedicatedServer : Выделенный сервер
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedServer {
    /// ID для каждого экземпляра выделенного сервера. Автоматически
    /// генерируется при создании.
    #[serde(rename = "id")]
    pub id:                    i64,
    /// Описание параметров процессора выделенного сервера.
    #[serde(rename = "cpu_description")]
    pub cpu_description:       String,
    /// Описание параметров жёсткого диска выделенного сервера.
    #[serde(rename = "hdd_description")]
    pub hdd_description:       String,
    /// Описание ОЗУ выделенного сервера.
    #[serde(rename = "ram_description")]
    pub ram_description:       String,
    /// Значение времени, указанное в комбинированном формате даты и времени
    /// ISO8601, которое представляет, когда был создан выделенный сервер.
    #[serde(rename = "created_at")]
    pub created_at:            chrono::DateTime<chrono::FixedOffset>,
    /// IP-адрес сетевого интерфейса IPv4.
    #[serde(rename = "ip", deserialize_with = "Option::deserialize")]
    pub ip:                    Option<String>,
    /// IP-адрес сетевого интерфейса IPMI.
    #[serde(rename = "ipmi_ip", deserialize_with = "Option::deserialize")]
    pub ipmi_ip:               Option<String>,
    /// Логин, используемый для входа в IPMI-консоль.
    #[serde(rename = "ipmi_login", deserialize_with = "Option::deserialize")]
    pub ipmi_login:            Option<String>,
    /// Пароль, используемый для входа в IPMI-консоль.
    #[serde(rename = "ipmi_password", deserialize_with = "Option::deserialize")]
    pub ipmi_password:         Option<String>,
    /// IP-адрес сетевого интерфейса IPv6.
    #[serde(rename = "ipv6", deserialize_with = "Option::deserialize")]
    pub ipv6:                  Option<String>,
    /// Внутренний дополнительный ID сервера.
    #[serde(rename = "node_id", deserialize_with = "Option::deserialize")]
    pub node_id:               Option<i64>,
    /// Удобочитаемое имя, установленное для выделенного сервера.
    #[serde(rename = "name")]
    pub name:                  String,
    /// Комментарий к выделенному серверу.
    #[serde(rename = "comment")]
    pub comment:               String,
    /// Пароль для подключения к VNC-консоли выделенного сервера.
    #[serde(rename = "vnc_pass", deserialize_with = "Option::deserialize")]
    pub vnc_pass:              Option<String>,
    /// Строка состояния, указывающая состояние выделенного сервера. Может быть
    /// «installing», «installed», «on» или «off».
    #[serde(rename = "status")]
    pub status:                Status,
    /// ID операционной системы, установленной на выделенный сервер.
    #[serde(rename = "os_id", deserialize_with = "Option::deserialize")]
    pub os_id:                 Option<i64>,
    /// ID панели управления, установленной на выделенный сервер.
    #[serde(rename = "cp_id", deserialize_with = "Option::deserialize")]
    pub cp_id:                 Option<i64>,
    /// ID интернет-канала, установленного на выделенный сервер.
    #[serde(rename = "bandwidth_id", deserialize_with = "Option::deserialize")]
    pub bandwidth_id:          Option<i64>,
    /// Массив уникальных ID сетевых дисков, подключенных к выделенному серверу.
    #[serde(rename = "network_drive_id", deserialize_with = "Option::deserialize")]
    pub network_drive_id:      Option<Vec<f64>>,
    /// Массив уникальных ID дополнительных IP-адресов, подключенных к
    /// выделенному серверу.
    #[serde(
        rename = "additional_ip_addr_id",
        deserialize_with = "Option::deserialize"
    )]
    pub additional_ip_addr_id: Option<Vec<f64>>,
    /// ID списка дополнительных услуг выделенного сервера.
    #[serde(rename = "plan_id", deserialize_with = "Option::deserialize")]
    pub plan_id:               Option<i64>,
    /// Стоимость выделенного сервера.
    #[serde(rename = "price")]
    pub price:                 f64,
    /// Локация сервера.
    #[serde(rename = "location")]
    pub location:              String,
    /// Количество готовых к автоматической выдаче серверов. Если значение равно
    /// 0, сервер будет установлен через инженеров.
    #[serde(rename = "autoinstall_ready")]
    pub autoinstall_ready:     f64,
    /// Пароль root сервера или пароль Администратора для серверов Windows.
    #[serde(rename = "password", deserialize_with = "Option::deserialize")]
    pub password:              Option<String>,
    /// Ссылка на аватар сервера.
    #[serde(rename = "avatar_link", deserialize_with = "Option::deserialize")]
    pub avatar_link:           Option<String>,
    /// Это логическое значение, которое показывает, готов ли выделенный сервер
    /// к моментальной выдаче.
    #[serde(rename = "is_pre_installed")]
    pub is_pre_installed:      bool,
    /// ID тарифа сервера.
    #[serde(rename = "preset_id")]
    pub preset_id:             i32,
    /// ID проекта
    #[serde(rename = "project_id")]
    pub project_id:            i32
}

impl DedicatedServer {
    /// Выделенный сервер
    pub fn new(
        id: i64,
        cpu_description: String,
        hdd_description: String,
        ram_description: String,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        ip: Option<String>,
        ipmi_ip: Option<String>,
        ipmi_login: Option<String>,
        ipmi_password: Option<String>,
        ipv6: Option<String>,
        node_id: Option<i64>,
        name: String,
        comment: String,
        vnc_pass: Option<String>,
        status: Status,
        os_id: Option<i64>,
        cp_id: Option<i64>,
        bandwidth_id: Option<i64>,
        network_drive_id: Option<Vec<f64>>,
        additional_ip_addr_id: Option<Vec<f64>>,
        plan_id: Option<i64>,
        price: f64,
        location: String,
        autoinstall_ready: f64,
        password: Option<String>,
        avatar_link: Option<String>,
        is_pre_installed: bool,
        preset_id: i32,
        project_id: i32
    ) -> DedicatedServer {
        DedicatedServer {
            id,
            cpu_description,
            hdd_description,
            ram_description,
            created_at,
            ip,
            ipmi_ip,
            ipmi_login,
            ipmi_password,
            ipv6,
            node_id,
            name,
            comment,
            vnc_pass,
            status,
            os_id,
            cp_id,
            bandwidth_id,
            network_drive_id,
            additional_ip_addr_id,
            plan_id,
            price,
            location,
            autoinstall_ready,
            password,
            avatar_link,
            is_pre_installed,
            preset_id,
            project_id
        }
    }
}
/// Строка состояния, указывающая состояние выделенного сервера. Может быть
/// «installing», «installed», «on» или «off».
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "installing")]
    Installing,
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off
}

impl Default for Status {
    fn default() -> Status {
        Self::Installing
    }
}
