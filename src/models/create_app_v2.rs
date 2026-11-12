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

/// CreateAppV2 : Параметры создания приложения. Для Git-репозитория обязательны
/// provider_id, repository_id, branch_name, commit_sha, build_cmd,
/// is_auto_deploy и framework. Для frontend также обязательны index_dir и
/// preset_id. Для backend из Git обязателен run_cmd. Для backend из готового
/// Docker-образа передайте docker_image и docker_image_tag; Git-параметры не
/// требуются. Для любого backend передайте либо preset_id, либо configuration.
/// Эти поля нельзя передавать одновременно.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAppV2 {
    /// ID провайдера. Обязателен при создании из Git-репозитория. Не требуется
    /// для готового Docker-образа.
    #[serde(rename = "provider_id", skip_serializing_if = "Option::is_none")]
    pub provider_id:           Option<String>,
    /// Тип приложения: frontend — статический сайт, backend — серверное
    /// приложение или Docker-образ.
    #[serde(rename = "type")]
    pub r#type:                Type,
    /// ID репозитория. Обязателен при создании из Git-репозитория. Не требуется
    /// для готового Docker-образа.
    #[serde(rename = "repository_id", skip_serializing_if = "Option::is_none")]
    pub repository_id:         Option<String>,
    /// Команда сборки приложения из Git-репозитория. Обязательна для Git; можно
    /// передать пустую строку, если сборка не нужна.
    #[serde(rename = "build_cmd", skip_serializing_if = "Option::is_none")]
    pub build_cmd:             Option<String>,
    /// Переменные окружения приложения. Ключи и значения — строки. Длина ключа
    /// — не более 255 символов, значения — не более 10240. По умолчанию пустой
    /// объект.
    #[serde(rename = "envs", skip_serializing_if = "Option::is_none")]
    pub envs:                  Option<std::collections::HashMap<String, String>>,
    /// Ветка Git-репозитория, из которой необходимо собрать приложение.
    /// Обязательна при создании из Git.
    #[serde(rename = "branch_name", skip_serializing_if = "Option::is_none")]
    pub branch_name:           Option<String>,
    /// Автоматически запускать деплой при изменении Git-репозитория. Поле
    /// обязательно при создании из Git. Для провайдера git и готового
    /// Docker-образа автодеплой выключен.
    #[serde(rename = "is_auto_deploy", skip_serializing_if = "Option::is_none")]
    pub is_auto_deploy:        Option<bool>,
    /// Хэш коммита для первого деплоя. Обязателен при создании из Git.
    #[serde(rename = "commit_sha", skip_serializing_if = "Option::is_none")]
    pub commit_sha:            Option<String>,
    /// Имя приложения, от 1 до 255 символов. Допустимы латинские и русские
    /// буквы, цифры, пробелы и символы - . ( ) , & # [ ] _.
    #[serde(rename = "name")]
    pub name:                  String,
    /// Комментарий к приложению. Обязательное поле; можно передать пустую
    /// строку. Допустимы те же символы, что и в имени приложения.
    #[serde(rename = "comment")]
    pub comment:               String,
    /// ID тарифа приложения. Обязателен для frontend. Для backend передайте
    /// либо preset_id, либо configuration. Нельзя передавать вместе с
    /// configuration. Доступные тарифы: GET /api/v1/presets/apps.
    #[serde(rename = "preset_id", skip_serializing_if = "Option::is_none")]
    pub preset_id:             Option<i32>,
    /// Версия окружения для сборки и запуска, например версия Node.js. Можно
    /// передать null.
    #[serde(
        rename = "env_version",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub env_version:           Option<Option<String>>,
    /// Фреймворк приложения, строка с учётом регистра. Обязателен при создании
    /// из Git. Для frontend: `angular`, `astro`, `ember`, `next.js`, `nuxt`,
    /// `preact`, `react`, `remix`, `solid`, `solid-start`, `svelte`,
    /// `sveltekit`, `tanstack-start`, `vue`, `static-nobuild`, `another`. Для
    /// backend: `django`, `express`, `phoenix`, `Spring`, `laravel`, `beego`,
    /// `fastapi`, `ASP.NET Core`, `hapi`, `celery`, `flask`, `gin`, `docker`,
    /// `docker-compose`, `fastify`, `nest`, `symfony`, `yii`, `next.js`,
    /// `nuxt`, `astro`, `tanstack-start`, `solid-start`, `ktor`, `axum`,
    /// `actix`, `tornado`, `aiohttp`, `fiber`, `echo`, `Micronaut`, `Quarkus`,
    /// `remix`, `sveltekit`. Для готового Docker-образа не требуется.
    #[serde(rename = "framework", skip_serializing_if = "Option::is_none")]
    pub framework:             Option<Framework>,
    /// Путь к каталогу с собранными статическими файлами, например /dist.
    /// Обязателен для frontend. Не используется для backend.
    #[serde(rename = "index_dir", skip_serializing_if = "Option::is_none")]
    pub index_dir:             Option<String>,
    /// Команда запуска backend-приложения из Git. Обязательна для backend из
    /// Git. Не требуется для готового Docker-образа и frontend.
    #[serde(rename = "run_cmd", skip_serializing_if = "Option::is_none")]
    pub run_cmd:               Option<String>,
    /// Системные пакеты для установки в окружение приложения. Допустимы
    /// строчные латинские буквы, цифры и символы . + - =. По умолчанию пустой
    /// массив.
    #[serde(
        rename = "system_dependencies",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_dependencies:   Option<Vec<String>>,
    /// ID проекта, в который необходимо добавить приложение. Проект должен
    /// принадлежать аккаунту.
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id:            Option<i32>,
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration:         Option<Box<models::CreateAppV2Configuration>>,
    /// Язык или способ развёртывания. Обязателен в v2. Для frontend:
    /// `javascript`. Для backend: `node`, `elixir`, `go`, `php`, `python`,
    /// `ruby`, `docker`, `docker-compose`, `dotnet`, `java`, `kotlin`, `rust`,
    /// `bun`, `deno`, `custom-image`. Для SSR-приложений из Git используйте
    /// node, для готового Docker-образа — custom-image.
    #[serde(rename = "language")]
    pub language:              Language,
    /// Каталог приложения в Git-репозитории, относительно которого выполняется
    /// сборка. Непустой путь начинается с /, не содержит .. или двойной
    /// начальный /. Допустимы латинские буквы, цифры, . - _ /. Можно передать
    /// пустую строку.
    #[serde(rename = "build_entry_path", skip_serializing_if = "Option::is_none")]
    pub build_entry_path:      Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network:               Option<Box<models::CreateAppV2Network>>,
    /// HTTP-путь для проверки работоспособности backend-приложения.
    #[serde(rename = "health_check", skip_serializing_if = "Option::is_none")]
    pub health_check:          Option<String>,
    /// ID глобальных переменных окружения аккаунта, которые необходимо связать
    /// с приложением.
    #[serde(rename = "global_env_ids", skip_serializing_if = "Option::is_none")]
    pub global_env_ids:        Option<Vec<f64>>,
    /// Имя готового Docker-образа для backend, без тега. Передавайте вместе с
    /// docker_image_tag. При указании обоих полей Git-параметры не требуются.
    #[serde(rename = "docker_image", skip_serializing_if = "Option::is_none")]
    pub docker_image:          Option<String>,
    /// Тег готового Docker-образа. Передавайте вместе с docker_image.
    #[serde(rename = "docker_image_tag", skip_serializing_if = "Option::is_none")]
    pub docker_image_tag:      Option<String>,
    /// Добавить доверенные корневые сертификаты Минцифры в окружение
    /// backend-приложения. Применяется в российском регионе.
    #[serde(
        rename = "is_russian_trusted_ca",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_russian_trusted_ca: Option<bool>,
    /// Добавить доверенные корневые сертификаты ТЦИ в окружение
    /// backend-приложения. Применяется в российском регионе.
    #[serde(rename = "is_tci_trusted_ca", skip_serializing_if = "Option::is_none")]
    pub is_tci_trusted_ca:     Option<bool>
}

impl CreateAppV2 {
    /// Параметры создания приложения. Для Git-репозитория обязательны
    /// provider_id, repository_id, branch_name, commit_sha, build_cmd,
    /// is_auto_deploy и framework. Для frontend также обязательны index_dir и
    /// preset_id. Для backend из Git обязателен run_cmd. Для backend из
    /// готового Docker-образа передайте docker_image и docker_image_tag;
    /// Git-параметры не требуются. Для любого backend передайте либо preset_id,
    /// либо configuration. Эти поля нельзя передавать одновременно.
    pub fn new(r#type: Type, name: String, comment: String, language: Language) -> CreateAppV2 {
        CreateAppV2 {
            provider_id: None,
            r#type,
            repository_id: None,
            build_cmd: None,
            envs: None,
            branch_name: None,
            is_auto_deploy: None,
            commit_sha: None,
            name,
            comment,
            preset_id: None,
            env_version: None,
            framework: None,
            index_dir: None,
            run_cmd: None,
            system_dependencies: None,
            project_id: None,
            configuration: None,
            language,
            build_entry_path: None,
            network: None,
            health_check: None,
            global_env_ids: None,
            docker_image: None,
            docker_image_tag: None,
            is_russian_trusted_ca: None,
            is_tci_trusted_ca: None
        }
    }
}
/// Тип приложения: frontend — статический сайт, backend — серверное приложение
/// или Docker-образ.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "frontend")]
    Frontend,
    #[serde(rename = "backend")]
    Backend
}

impl Default for Type {
    fn default() -> Type {
        Self::Frontend
    }
}
/// Фреймворк приложения, строка с учётом регистра. Обязателен при создании из
/// Git. Для frontend: `angular`, `astro`, `ember`, `next.js`, `nuxt`, `preact`,
/// `react`, `remix`, `solid`, `solid-start`, `svelte`, `sveltekit`,
/// `tanstack-start`, `vue`, `static-nobuild`, `another`. Для backend: `django`,
/// `express`, `phoenix`, `Spring`, `laravel`, `beego`, `fastapi`, `ASP.NET
/// Core`, `hapi`, `celery`, `flask`, `gin`, `docker`, `docker-compose`,
/// `fastify`, `nest`, `symfony`, `yii`, `next.js`, `nuxt`, `astro`,
/// `tanstack-start`, `solid-start`, `ktor`, `axum`, `actix`, `tornado`,
/// `aiohttp`, `fiber`, `echo`, `Micronaut`, `Quarkus`, `remix`, `sveltekit`.
/// Для готового Docker-образа не требуется.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Framework {
    #[serde(rename = "django")]
    Django,
    #[serde(rename = "express")]
    Express,
    #[serde(rename = "phoenix")]
    Phoenix,
    #[serde(rename = "Spring")]
    Spring,
    #[serde(rename = "laravel")]
    Laravel,
    #[serde(rename = "beego")]
    Beego,
    #[serde(rename = "fastapi")]
    Fastapi,
    #[serde(rename = "ASP.NET Core")]
    AspNetCore,
    #[serde(rename = "hapi")]
    Hapi,
    #[serde(rename = "celery")]
    Celery,
    #[serde(rename = "flask")]
    Flask,
    #[serde(rename = "gin")]
    Gin,
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "docker-compose")]
    DockerCompose,
    #[serde(rename = "fastify")]
    Fastify,
    #[serde(rename = "nest")]
    Nest,
    #[serde(rename = "symfony")]
    Symfony,
    #[serde(rename = "yii")]
    Yii,
    #[serde(rename = "next.js")]
    NextJs,
    #[serde(rename = "nuxt")]
    Nuxt,
    #[serde(rename = "astro")]
    Astro,
    #[serde(rename = "tanstack-start")]
    TanstackStart,
    #[serde(rename = "solid-start")]
    SolidStart,
    #[serde(rename = "ktor")]
    Ktor,
    #[serde(rename = "axum")]
    Axum,
    #[serde(rename = "actix")]
    Actix,
    #[serde(rename = "tornado")]
    Tornado,
    #[serde(rename = "aiohttp")]
    Aiohttp,
    #[serde(rename = "fiber")]
    Fiber,
    #[serde(rename = "echo")]
    Echo,
    #[serde(rename = "Micronaut")]
    Micronaut,
    #[serde(rename = "Quarkus")]
    Quarkus,
    #[serde(rename = "remix")]
    Remix,
    #[serde(rename = "sveltekit")]
    Sveltekit,
    #[serde(rename = "angular")]
    Angular,
    #[serde(rename = "ember")]
    Ember,
    #[serde(rename = "preact")]
    Preact,
    #[serde(rename = "react")]
    React,
    #[serde(rename = "solid")]
    Solid,
    #[serde(rename = "svelte")]
    Svelte,
    #[serde(rename = "vue")]
    Vue,
    #[serde(rename = "static-nobuild")]
    StaticNobuild,
    #[serde(rename = "another")]
    Another
}

impl Default for Framework {
    fn default() -> Framework {
        Self::Django
    }
}
/// Язык или способ развёртывания. Обязателен в v2. Для frontend: `javascript`.
/// Для backend: `node`, `elixir`, `go`, `php`, `python`, `ruby`, `docker`,
/// `docker-compose`, `dotnet`, `java`, `kotlin`, `rust`, `bun`, `deno`,
/// `custom-image`. Для SSR-приложений из Git используйте node, для готового
/// Docker-образа — custom-image.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "javascript")]
    Javascript,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "elixir")]
    Elixir,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "php")]
    Php,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "ruby")]
    Ruby,
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "docker-compose")]
    DockerCompose,
    #[serde(rename = "dotnet")]
    Dotnet,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "kotlin")]
    Kotlin,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "bun")]
    Bun,
    #[serde(rename = "deno")]
    Deno,
    #[serde(rename = "custom-image")]
    CustomImage
}

impl Default for Language {
    fn default() -> Language {
        Self::Javascript
    }
}
