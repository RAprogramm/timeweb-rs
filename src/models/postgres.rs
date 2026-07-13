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

/// Postgres : Параметры PostgreSQL (`postgres` | `postgres14` | `postgres15` |
/// `postgres16` | `postgres17` | `postgres18`)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Postgres {
    /// Максимальное количество одновременных подключений к серверу (`mysql5` |
    /// `mysql` | `mysql8_4` | `postgres` | `postgres14` | `postgres15` |
    /// `postgres16` | `postgres17` | `postgres18`).
    #[serde(rename = "max_connections", skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<String>,
    /// Доля изменения строк таблицы перед запуском автоматического анализа
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "autovacuum_analyze_scale_factor",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_analyze_scale_factor: Option<String>,
    /// Максимальное количество процессов autovacuum, которые могут работать
    /// одновременно (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(
        rename = "autovacuum_max_workers",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_max_workers: Option<String>,
    /// Интервал между запусками процессов autovacuum (`postgres` | `postgres14`
    /// | `postgres15` | `postgres16` | `postgres17` | `postgres18`).
    #[serde(rename = "autovacuum_naptime", skip_serializing_if = "Option::is_none")]
    pub autovacuum_naptime: Option<String>,
    /// Доля вставленных строк перед запуском vacuum для таблиц с большим
    /// количеством вставок (`postgres` | `postgres14` | `postgres15` |
    /// `postgres16` | `postgres17` | `postgres18`).
    #[serde(
        rename = "autovacuum_vacuum_insert_scale_factor",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_vacuum_insert_scale_factor: Option<String>,
    /// Доля измененных или удаленных строк перед запуском autovacuum
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "autovacuum_vacuum_scale_factor",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_vacuum_scale_factor: Option<String>,
    /// Объем памяти, используемый одним процессом autovacuum (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(
        rename = "autovacuum_work_mem",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_work_mem: Option<String>,
    /// Интервал между циклами фонового процесса записи страниц (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "bgwriter_delay", skip_serializing_if = "Option::is_none")]
    pub bgwriter_delay: Option<String>,
    /// Максимальное количество страниц, записываемых background writer за один
    /// цикл (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(
        rename = "bgwriter_lru_maxpages",
        skip_serializing_if = "Option::is_none"
    )]
    pub bgwriter_lru_maxpages: Option<String>,
    /// Время ожидания блокировки перед проверкой взаимной блокировки
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "deadlock_timeout", skip_serializing_if = "Option::is_none")]
    pub deadlock_timeout: Option<String>,
    /// Максимальный размер списка ожидающих вставок индекса GIN (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(
        rename = "gin_pending_list_limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub gin_pending_list_limit: Option<String>,
    /// Время ожидания неактивной транзакционной сессии перед завершением
    /// соединения (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(
        rename = "idle_in_transaction_session_timeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub idle_in_transaction_session_timeout: Option<String>,
    /// Максимальное количество таблиц в JOIN, которые планировщик может
    /// переупорядочить (`postgres` | `postgres14` | `postgres15` | `postgres16`
    /// | `postgres17` | `postgres18`).
    #[serde(
        rename = "join_collapse_limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub join_collapse_limit: Option<String>,
    /// Максимальное время ожидания блокировки перед отменой запроса (`postgres`
    /// | `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "lock_timeout", skip_serializing_if = "Option::is_none")]
    pub lock_timeout: Option<String>,
    /// Максимальное количество подготовленных транзакций, которые могут
    /// существовать одновременно (`postgres` | `postgres14` | `postgres15` |
    /// `postgres16` | `postgres17` | `postgres18`).
    #[serde(
        rename = "max_prepared_transactions",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_prepared_transactions: Option<String>,
    /// Размер общей памяти, используемой PostgreSQL для буферного кэша
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "shared_buffers", skip_serializing_if = "Option::is_none")]
    pub shared_buffers: Option<String>,
    /// Минимальное время выполнения запроса, после которого он записывается в
    /// журнал (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(
        rename = "log_min_duration_statement",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_min_duration_statement: Option<String>,
    /// Размер памяти, используемой для буферизации WAL-записей (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "wal_buffers", skip_serializing_if = "Option::is_none")]
    pub wal_buffers: Option<String>,
    /// Максимальный объем памяти для временных таблиц каждой сессии (`postgres`
    /// | `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "temp_buffers", skip_serializing_if = "Option::is_none")]
    pub temp_buffers: Option<String>,
    /// Объем памяти, используемый одной операцией сортировки или хеширования
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "work_mem", skip_serializing_if = "Option::is_none")]
    pub work_mem: Option<String>,
    /// Уровень изоляции транзакций по умолчанию (`postgres` | `postgres14` |
    /// `postgres15` | `postgres16` | `postgres17` | `postgres18`).
    #[serde(
        rename = "default_transaction_isolation",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_transaction_isolation: Option<String>,
    /// Оценка объема дискового кэша, доступного планировщику запросов
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "effective_cache_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub effective_cache_size: Option<String>,
    /// Максимальный размер WAL перед запуском контрольной точки (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "max_wal_size", skip_serializing_if = "Option::is_none")]
    pub max_wal_size: Option<String>,
    /// Минимальный размер WAL, который сохраняется между контрольными точками
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "min_wal_size", skip_serializing_if = "Option::is_none")]
    pub min_wal_size: Option<String>,
    /// Уровень детализации записи WAL для восстановления и репликации
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "wal_level", skip_serializing_if = "Option::is_none")]
    pub wal_level: Option<String>,
    /// Максимальное количество слотов репликации, которые могут быть созданы
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "max_replication_slots",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_replication_slots: Option<String>,
    /// Максимальное количество процессов отправки WAL для репликации
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "max_wal_senders", skip_serializing_if = "Option::is_none")]
    pub max_wal_senders: Option<String>,
    /// Максимальное количество фоновых процессов PostgreSQL (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(
        rename = "max_worker_processes",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_worker_processes: Option<String>,
    /// Максимальное количество процессов логической репликации (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(
        rename = "max_logical_replication_workers",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_logical_replication_workers: Option<String>,
    /// Максимальное количество параллельных процессов для операций обслуживания
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "max_parallel_maintenance_workers",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parallel_maintenance_workers: Option<String>,
    /// Максимальное количество параллельных рабочих процессов для запросов
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "max_parallel_workers",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parallel_workers: Option<String>,
    /// Максимальное количество параллельных рабочих процессов на один
    /// Gather-узел (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(
        rename = "max_parallel_workers_per_gather",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parallel_workers_per_gather: Option<String>,
    /// Разрешение использования NULL в массивах PostgreSQL (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "array_nulls", skip_serializing_if = "Option::is_none")]
    pub array_nulls: Option<String>,
    /// Количество страниц, после записи которых выполняется принудительная
    /// очистка данных на диск серверным процессом (`postgres` | `postgres14` |
    /// `postgres15` | `postgres16` | `postgres17` | `postgres18`).
    #[serde(
        rename = "backend_flush_after",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_flush_after: Option<String>,
    /// Управление использованием обратного слеша в строковых литералах
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "backslash_quote", skip_serializing_if = "Option::is_none")]
    pub backslash_quote: Option<String>,
    /// Количество страниц, после которого background writer выполняет очистку
    /// данных на диск (`postgres` | `postgres14` | `postgres15` | `postgres16`
    /// | `postgres17` | `postgres18`).
    #[serde(
        rename = "bgwriter_flush_after",
        skip_serializing_if = "Option::is_none"
    )]
    pub bgwriter_flush_after: Option<String>,
    /// Множитель количества страниц, которые background writer пытается
    /// очистить (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(
        rename = "bgwriter_lru_multiplier",
        skip_serializing_if = "Option::is_none"
    )]
    pub bgwriter_lru_multiplier: Option<String>,
    /// Определяет режим транзакций только для чтения по умолчанию (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(
        rename = "default_transaction_read_only",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_transaction_read_only: Option<String>,
    /// Разрешение использования Hash Aggregate планировщиком запросов
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "enable_hashagg", skip_serializing_if = "Option::is_none")]
    pub enable_hashagg: Option<String>,
    /// Разрешение использования Hash Join планировщиком запросов (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "enable_hashjoin", skip_serializing_if = "Option::is_none")]
    pub enable_hashjoin: Option<String>,
    /// Разрешение использования инкрементальной сортировки планировщиком
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "enable_incremental_sort",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_incremental_sort: Option<String>,
    /// Разрешение использования обычного индексного сканирования (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "enable_indexscan", skip_serializing_if = "Option::is_none")]
    pub enable_indexscan: Option<String>,
    /// Разрешение использования index-only scan (`postgres` | `postgres14` |
    /// `postgres15` | `postgres16` | `postgres17` | `postgres18`).
    #[serde(
        rename = "enable_indexonlyscan",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_indexonlyscan: Option<String>,
    /// Разрешение использования материализации промежуточных результатов
    /// запросов (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(rename = "enable_material", skip_serializing_if = "Option::is_none")]
    pub enable_material: Option<String>,
    /// Разрешение использования Memoize узлов планировщиком запросов
    /// (`postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "enable_memoize", skip_serializing_if = "Option::is_none")]
    pub enable_memoize: Option<String>,
    /// Разрешение использования Merge Join планировщиком запросов (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "enable_mergejoin", skip_serializing_if = "Option::is_none")]
    pub enable_mergejoin: Option<String>,
    /// Разрешение использования параллельного Append для запросов (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(
        rename = "enable_parallel_append",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_parallel_append: Option<String>,
    /// Разрешение использования параллельных Hash операций (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(
        rename = "enable_parallel_hash",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_parallel_hash: Option<String>,
    /// Разрешение удаления ненужных разделов таблицы при планировании запроса
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "enable_partition_pruning",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_partition_pruning: Option<String>,
    /// Разрешение выполнения соединений между секционированными таблицами с
    /// учетом секций (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(
        rename = "enable_partitionwise_join",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_partitionwise_join: Option<String>,
    /// Разрешение выполнения агрегатных операций отдельно для секций таблиц
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "enable_partitionwise_aggregate",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_partitionwise_aggregate: Option<String>,
    /// Разрешение использования последовательного сканирования таблиц
    /// планировщиком запросов (`postgres` | `postgres14` | `postgres15` |
    /// `postgres16` | `postgres17` | `postgres18`).
    #[serde(rename = "enable_seqscan", skip_serializing_if = "Option::is_none")]
    pub enable_seqscan: Option<String>,
    /// Разрешение использования операций сортировки планировщиком запросов
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "enable_sort", skip_serializing_if = "Option::is_none")]
    pub enable_sort: Option<String>,
    /// Разрешение использования TID Scan для поиска строк по физическим
    /// идентификаторам (`postgres` | `postgres14` | `postgres15` | `postgres16`
    /// | `postgres17` | `postgres18`).
    #[serde(rename = "enable_tidscan", skip_serializing_if = "Option::is_none")]
    pub enable_tidscan: Option<String>,
    /// Завершение сессии при возникновении ошибки SQL (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "exit_on_error", skip_serializing_if = "Option::is_none")]
    pub exit_on_error: Option<String>,
    /// Максимальное количество элементов FROM, которые планировщик может
    /// объединять при оптимизации запросов (`postgres` | `postgres14` |
    /// `postgres15` | `postgres16` | `postgres17` | `postgres18`).
    #[serde(
        rename = "from_collapse_limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_collapse_limit: Option<String>,
    /// Включение JIT-компиляции для ускорения выполнения запросов (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "jit", skip_serializing_if = "Option::is_none")]
    pub jit: Option<String>,
    /// Режим использования кэша планов подготовленных запросов (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "plan_cache_mode", skip_serializing_if = "Option::is_none")]
    pub plan_cache_mode: Option<String>,
    /// Всегда заключать идентификаторы в кавычки при генерации SQL (`postgres`
    /// | `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(
        rename = "quote_all_identifiers",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_all_identifiers: Option<String>,
    /// Использование стандартного поведения строковых литералов SQL (`postgres`
    /// | `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(
        rename = "standard_conforming_strings",
        skip_serializing_if = "Option::is_none"
    )]
    pub standard_conforming_strings: Option<String>,
    /// Максимальное время выполнения SQL-запроса перед автоматической отменой
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "statement_timeout", skip_serializing_if = "Option::is_none")]
    pub statement_timeout: Option<String>,
    /// Часовой пояс сервера PostgreSQL по умолчанию (`postgres` | `postgres14`
    /// | `postgres15` | `postgres16` | `postgres17` | `postgres18`).
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Преобразование выражений вида `NULL = NULL` в проверку IS NULL
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "transform_null_equals",
        skip_serializing_if = "Option::is_none"
    )]
    pub transform_null_equals: Option<String>,
    /// Количество объектов, которые может блокировать одна транзакция
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "max_locks_per_transaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_locks_per_transaction: Option<String>,
    /// Лимит стоимости операций autovacuum перед приостановкой работы
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(
        rename = "autovacuum_vacuum_cost_limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_vacuum_cost_limit: Option<String>,
    /// Максимальный интервал времени между автоматическими контрольными точками
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "checkpoint_timeout", skip_serializing_if = "Option::is_none")]
    pub checkpoint_timeout: Option<String>,
    /// Доля интервала checkpoint, за которую PostgreSQL распределяет запись
    /// данных (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(
        rename = "checkpoint_completion_target",
        skip_serializing_if = "Option::is_none"
    )]
    pub checkpoint_completion_target: Option<String>,
    /// Включение сжатия WAL-записей для уменьшения объема журнала (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "wal_compression", skip_serializing_if = "Option::is_none")]
    pub wal_compression: Option<String>,
    /// Оценочная стоимость случайного чтения страницы для планировщика запросов
    /// (`postgres` | `postgres14` | `postgres15` | `postgres16` | `postgres17`
    /// | `postgres18`).
    #[serde(rename = "random_page_cost", skip_serializing_if = "Option::is_none")]
    pub random_page_cost: Option<String>,
    /// Количество параллельных операций ввода-вывода, которые планировщик может
    /// учитывать (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(
        rename = "effective_io_concurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub effective_io_concurrency: Option<String>,
    /// Включение записи в журнал информации об ожидании блокировок дольше
    /// deadlock_timeout (`postgres` | `postgres14` | `postgres15` |
    /// `postgres16` | `postgres17` | `postgres18`).
    #[serde(rename = "log_lock_waits", skip_serializing_if = "Option::is_none")]
    pub log_lock_waits: Option<String>,
    /// Минимальный размер временных файлов, при котором они записываются в
    /// журнал (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(rename = "log_temp_files", skip_serializing_if = "Option::is_none")]
    pub log_temp_files: Option<String>,
    /// Включение сбора статистики времени операций ввода-вывода (`postgres` |
    /// `postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(rename = "track_io_timing", skip_serializing_if = "Option::is_none")]
    pub track_io_timing: Option<String>,
    /// Максимальный объем памяти для операций обслуживания, таких как VACUUM и
    /// CREATE INDEX (`postgres` | `postgres14` | `postgres15` | `postgres16` |
    /// `postgres17` | `postgres18`).
    #[serde(
        rename = "maintenance_work_mem",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_work_mem: Option<String>,
    /// Время ожидания неактивной сессии перед автоматическим завершением
    /// соединения (`postgres14` | `postgres15` | `postgres16` | `postgres17` |
    /// `postgres18`).
    #[serde(
        rename = "idle_session_timeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub idle_session_timeout: Option<String>,
    /// Метод выполнения операций ввода-вывода PostgreSQL (`postgres18`).
    #[serde(rename = "io_method", skip_serializing_if = "Option::is_none")]
    pub io_method: Option<String>,
    /// Количество фоновых процессов для выполнения операций ввода-вывода
    /// (`postgres18`).
    #[serde(rename = "io_workers", skip_serializing_if = "Option::is_none")]
    pub io_workers: Option<String>
}

impl Postgres {
    /// Параметры PostgreSQL (`postgres` | `postgres14` | `postgres15` |
    /// `postgres16` | `postgres17` | `postgres18`)
    pub fn new() -> Postgres {
        Postgres {
            max_connections: None,
            autovacuum_analyze_scale_factor: None,
            autovacuum_max_workers: None,
            autovacuum_naptime: None,
            autovacuum_vacuum_insert_scale_factor: None,
            autovacuum_vacuum_scale_factor: None,
            autovacuum_work_mem: None,
            bgwriter_delay: None,
            bgwriter_lru_maxpages: None,
            deadlock_timeout: None,
            gin_pending_list_limit: None,
            idle_in_transaction_session_timeout: None,
            join_collapse_limit: None,
            lock_timeout: None,
            max_prepared_transactions: None,
            shared_buffers: None,
            log_min_duration_statement: None,
            wal_buffers: None,
            temp_buffers: None,
            work_mem: None,
            default_transaction_isolation: None,
            effective_cache_size: None,
            max_wal_size: None,
            min_wal_size: None,
            wal_level: None,
            max_replication_slots: None,
            max_wal_senders: None,
            max_worker_processes: None,
            max_logical_replication_workers: None,
            max_parallel_maintenance_workers: None,
            max_parallel_workers: None,
            max_parallel_workers_per_gather: None,
            array_nulls: None,
            backend_flush_after: None,
            backslash_quote: None,
            bgwriter_flush_after: None,
            bgwriter_lru_multiplier: None,
            default_transaction_read_only: None,
            enable_hashagg: None,
            enable_hashjoin: None,
            enable_incremental_sort: None,
            enable_indexscan: None,
            enable_indexonlyscan: None,
            enable_material: None,
            enable_memoize: None,
            enable_mergejoin: None,
            enable_parallel_append: None,
            enable_parallel_hash: None,
            enable_partition_pruning: None,
            enable_partitionwise_join: None,
            enable_partitionwise_aggregate: None,
            enable_seqscan: None,
            enable_sort: None,
            enable_tidscan: None,
            exit_on_error: None,
            from_collapse_limit: None,
            jit: None,
            plan_cache_mode: None,
            quote_all_identifiers: None,
            standard_conforming_strings: None,
            statement_timeout: None,
            timezone: None,
            transform_null_equals: None,
            max_locks_per_transaction: None,
            autovacuum_vacuum_cost_limit: None,
            checkpoint_timeout: None,
            checkpoint_completion_target: None,
            wal_compression: None,
            random_page_cost: None,
            effective_io_concurrency: None,
            log_lock_waits: None,
            log_temp_files: None,
            track_io_timing: None,
            maintenance_work_mem: None,
            idle_session_timeout: None,
            io_method: None,
            io_workers: None
        }
    }
}
