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

/// Mysql : Параметры MySQL (`mysql5` | `mysql` | `mysql8_4`)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mysql {
    /// Размер буфера, используемого при соединениях таблиц без индексов
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "join_buffer_size", skip_serializing_if = "Option::is_none")]
    pub join_buffer_size: Option<String>,
    /// Максимальное количество одновременных подключений к серверу (`mysql5` |
    /// `mysql` | `mysql8_4` | `postgres` | `postgres14` | `postgres15` |
    /// `postgres16` | `postgres17` | `postgres18`).
    #[serde(rename = "max_connections", skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<String>,
    /// Размер буфера сортировки для операций ORDER BY и GROUP BY (`mysql5` |
    /// `mysql` | `mysql8_4`).
    #[serde(rename = "sort_buffer_size", skip_serializing_if = "Option::is_none")]
    pub sort_buffer_size: Option<String>,
    /// Количество потоков, которые сервер сохраняет для повторного
    /// использования (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "thread_cache_size", skip_serializing_if = "Option::is_none")]
    pub thread_cache_size: Option<String>,
    /// Размер буферного пула InnoDB для хранения данных и индексов в памяти
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "innodb_buffer_pool_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_buffer_pool_size: Option<String>,
    /// Интервал между значениями столбцов с атрибутом `AUTO_INCREMENT`
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "auto_increment_increment",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_increment_increment: Option<String>,
    /// Начальное значение для столбцов с атрибутом `AUTO_INCREMENT` (`mysql5` |
    /// `mysql` | `mysql8_4`).
    #[serde(
        rename = "auto_increment_offset",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_increment_offset: Option<String>,
    /// Количество операций ввода-вывода в секунду `IOPS`, используемых InnoDB
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "innodb_io_capacity", skip_serializing_if = "Option::is_none")]
    pub innodb_io_capacity: Option<String>,
    /// Количество потоков, используемых для фоновой очистки undo-записей InnoDB
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "innodb_purge_threads",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_purge_threads: Option<String>,
    /// Количество потоков ввода-вывода для операций чтения InnoDB (`mysql5` |
    /// `mysql` | `mysql8_4`).
    #[serde(
        rename = "innodb_read_io_threads",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_read_io_threads: Option<String>,
    /// Ограничение количества одновременно выполняющихся потоков InnoDB
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "innodb_thread_concurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_thread_concurrency: Option<String>,
    /// Количество потоков ввода-вывода для операций записи InnoDB (`mysql5` |
    /// `mysql` | `mysql8_4`).
    #[serde(
        rename = "innodb_write_io_threads",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_write_io_threads: Option<String>,
    /// Размер файла журнала транзакций InnoDB redo log (`mysql5` | `mysql` |
    /// `mysql8_4`).
    #[serde(
        rename = "innodb_log_file_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_log_file_size: Option<String>,
    /// Максимальный размер пакета данных, который может передаваться между
    /// клиентом и сервером (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "max_allowed_packet", skip_serializing_if = "Option::is_none")]
    pub max_allowed_packet: Option<String>,
    /// Максимальный размер таблиц типа MEMORY (`mysql5` | `mysql` |
    /// `mysql8_4`).
    #[serde(
        rename = "max_heap_table_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_heap_table_size: Option<String>,
    /// Режим работы SQL сервера, определяющий поведение обработки запросов
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "sql_mode", skip_serializing_if = "Option::is_none")]
    pub sql_mode: Option<String>,
    /// Тип кэша запросов (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "query_cache_type", skip_serializing_if = "Option::is_none")]
    pub query_cache_type: Option<String>,
    /// Объем памяти, выделяемый для кэширования результатов запросов (`mysql5`
    /// | `mysql` | `mysql8_4`).
    #[serde(rename = "query_cache_size", skip_serializing_if = "Option::is_none")]
    pub query_cache_size: Option<String>,
    /// Режим записи журнала InnoDB при фиксации транзакций (`mysql5` | `mysql`
    /// | `mysql8_4`).
    #[serde(
        rename = "innodb_flush_log_at_trx_commit",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_flush_log_at_trx_commit: Option<String>,
    /// Уровень изоляции транзакций по умолчанию (`mysql5` | `mysql` |
    /// `mysql8_4`).
    #[serde(
        rename = "transaction_isolation",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_isolation: Option<String>,
    /// Время выполнения запроса, после которого он считается долгим и может
    /// попасть в slow query log (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "long_query_time", skip_serializing_if = "Option::is_none")]
    pub long_query_time: Option<String>,
    /// Максимальный размер временных таблиц в памяти (`mysql5` | `mysql` |
    /// `mysql8_4`).
    #[serde(rename = "tmp_table_size", skip_serializing_if = "Option::is_none")]
    pub tmp_table_size: Option<String>,
    /// Количество открытых таблиц, которые сервер может хранить в кэше
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "table_open_cache", skip_serializing_if = "Option::is_none")]
    pub table_open_cache: Option<String>,
    /// Количество экземпляров кэша открытых таблиц для снижения конкуренции
    /// между потоками (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "table_open_cache_instances",
        skip_serializing_if = "Option::is_none"
    )]
    pub table_open_cache_instances: Option<String>,
    /// Метод выполнения операций записи и синхронизации файлов InnoDB (`mysql5`
    /// | `mysql` | `mysql8_4`).
    #[serde(
        rename = "innodb_flush_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_flush_method: Option<String>,
    /// Включение строгой проверки операций InnoDB (`mysql5` | `mysql` |
    /// `mysql8_4`).
    #[serde(rename = "innodb_strict_mode", skip_serializing_if = "Option::is_none")]
    pub innodb_strict_mode: Option<String>,
    /// Включение журнала медленных запросов (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "slow_query_log", skip_serializing_if = "Option::is_none")]
    pub slow_query_log: Option<String>,
    /// Размер кэша бинарного журнала для транзакций (`mysql5` | `mysql` |
    /// `mysql8_4`).
    #[serde(rename = "binlog_cache_size", skip_serializing_if = "Option::is_none")]
    pub binlog_cache_size: Option<String>,
    /// Задержка синхронизации групповой фиксации бинарного журнала в
    /// микросекундах (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "binlog_group_commit_sync_delay",
        skip_serializing_if = "Option::is_none"
    )]
    pub binlog_group_commit_sync_delay: Option<String>,
    /// Количество информации, записываемой в бинарный журнал при row-based
    /// репликации (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "binlog_row_image", skip_serializing_if = "Option::is_none")]
    pub binlog_row_image: Option<String>,
    /// Включение записи SQL-запросов в бинарный журнал при row-based репликации
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "binlog_rows_query_log_events",
        skip_serializing_if = "Option::is_none"
    )]
    pub binlog_rows_query_log_events: Option<String>,
    /// Кодировка по умолчанию для сервера MySQL (`mysql5` | `mysql` |
    /// `mysql8_4`).
    #[serde(
        rename = "character_set_server",
        skip_serializing_if = "Option::is_none"
    )]
    pub character_set_server: Option<String>,
    /// Определяет автоматическое поведение TIMESTAMP без явных значений по
    /// умолчанию (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "explicit_defaults_for_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub explicit_defaults_for_timestamp: Option<String>,
    /// Максимальная длина результата функции GROUP_CONCAT (`mysql5` | `mysql` |
    /// `mysql8_4`).
    #[serde(
        rename = "group_concat_max_len",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_concat_max_len: Option<String>,
    /// Включение или отключение адаптивного хэш-индекса InnoDB для ускорения
    /// поиска по индексам (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "innodb_adaptive_hash_index",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_adaptive_hash_index: Option<String>,
    /// Время ожидания блокировки InnoDB перед завершением транзакции с ошибкой
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "innodb_lock_wait_timeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_lock_wait_timeout: Option<String>,
    /// Включение распределения памяти InnoDB между NUMA-узлами (`mysql5` |
    /// `mysql` | `mysql8_4`).
    #[serde(
        rename = "innodb_numa_interleave",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_numa_interleave: Option<String>,
    /// Время ожидания данных от клиента при чтении сетевого соединения
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "net_read_timeout", skip_serializing_if = "Option::is_none")]
    pub net_read_timeout: Option<String>,
    /// Время ожидания записи данных клиенту через сетевое соединение (`mysql5`
    /// | `mysql` | `mysql8_4`).
    #[serde(rename = "net_write_timeout", skip_serializing_if = "Option::is_none")]
    pub net_write_timeout: Option<String>,
    /// Максимальное время выполнения регулярных выражений (`mysql` |
    /// `mysql8_4`).
    #[serde(rename = "regexp_time_limit", skip_serializing_if = "Option::is_none")]
    pub regexp_time_limit: Option<String>,
    /// Количество операций записи бинарного журнала перед принудительной
    /// синхронизацией на диск (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "sync_binlog", skip_serializing_if = "Option::is_none")]
    pub sync_binlog: Option<String>,
    /// Количество определений таблиц, хранящихся в кэше (`mysql5` | `mysql` |
    /// `mysql8_4`).
    #[serde(
        rename = "table_definition_cache",
        skip_serializing_if = "Option::is_none"
    )]
    pub table_definition_cache: Option<String>,
    /// Разрешение создания хранимых функций без проверки бинарной регистрации
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "log_bin_trust_function_creators",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_bin_trust_function_creators: Option<String>,
    /// Отключение DNS-разрешения имен клиентов при подключении к серверу
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "skip_name_resolve", skip_serializing_if = "Option::is_none")]
    pub skip_name_resolve: Option<String>,
    /// Общий размер redo log InnoDB для хранения журнала восстановления
    /// (`mysql8_4`).
    #[serde(
        rename = "innodb_redo_log_capacity",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_redo_log_capacity: Option<String>,
    /// Время ожидания неактивного клиентского соединения перед закрытием
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(rename = "wait_timeout", skip_serializing_if = "Option::is_none")]
    pub wait_timeout: Option<String>,
    /// Время ожидания неактивного интерактивного соединения перед закрытием
    /// (`mysql5` | `mysql` | `mysql8_4`).
    #[serde(
        rename = "interactive_timeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub interactive_timeout: Option<String>,
    /// Часовой пояс сервера MySQL по умолчанию (`mysql5` | `mysql` |
    /// `mysql8_4`).
    #[serde(rename = "default-time-zone", skip_serializing_if = "Option::is_none")]
    pub default_time_zone: Option<String>,
    /// Режим строгой проверки операций в Percona XtraDB Cluster (`mysql5` |
    /// `mysql` | `mysql8_4`).
    #[serde(rename = "pxc_strict_mode", skip_serializing_if = "Option::is_none")]
    pub pxc_strict_mode: Option<String>
}

impl Mysql {
    /// Параметры MySQL (`mysql5` | `mysql` | `mysql8_4`)
    pub fn new() -> Mysql {
        Mysql {
            join_buffer_size: None,
            max_connections: None,
            sort_buffer_size: None,
            thread_cache_size: None,
            innodb_buffer_pool_size: None,
            auto_increment_increment: None,
            auto_increment_offset: None,
            innodb_io_capacity: None,
            innodb_purge_threads: None,
            innodb_read_io_threads: None,
            innodb_thread_concurrency: None,
            innodb_write_io_threads: None,
            innodb_log_file_size: None,
            max_allowed_packet: None,
            max_heap_table_size: None,
            sql_mode: None,
            query_cache_type: None,
            query_cache_size: None,
            innodb_flush_log_at_trx_commit: None,
            transaction_isolation: None,
            long_query_time: None,
            tmp_table_size: None,
            table_open_cache: None,
            table_open_cache_instances: None,
            innodb_flush_method: None,
            innodb_strict_mode: None,
            slow_query_log: None,
            binlog_cache_size: None,
            binlog_group_commit_sync_delay: None,
            binlog_row_image: None,
            binlog_rows_query_log_events: None,
            character_set_server: None,
            explicit_defaults_for_timestamp: None,
            group_concat_max_len: None,
            innodb_adaptive_hash_index: None,
            innodb_lock_wait_timeout: None,
            innodb_numa_interleave: None,
            net_read_timeout: None,
            net_write_timeout: None,
            regexp_time_limit: None,
            sync_binlog: None,
            table_definition_cache: None,
            log_bin_trust_function_creators: None,
            skip_name_resolve: None,
            innodb_redo_log_capacity: None,
            wait_timeout: None,
            interactive_timeout: None,
            default_time_zone: None,
            pxc_strict_mode: None
        }
    }
}
