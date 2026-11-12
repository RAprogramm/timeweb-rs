# Флаги Cargo

У каждой области API свой флаг Cargo: имя флага — это имя области, где
подчёркивания заменены на дефисы.

| Область | Флаг | Область | Флаг |
| --- | --- | --- | --- |
| `account_api` | `account` | `knowledge_bases_api` | `knowledge-bases` |
| `ai_agent_integrations_api` | `ai-agent-integrations` | `kubernetes_api` | `kubernetes` |
| `ai_agent_skills_api` | `ai-agent-skills` | `locations_api` | `locations` |
| `ai_agents_api` | `ai-agents` | `mail_api` | `mail` |
| `ai_gateway_api` | `ai-gateway` | `mcp_api` | `mcp` |
| `ai_gateway_inference_api` | `ai-gateway-inference` | `network_drives_api` | `network-drives` |
| `ai_models_api` | `ai-models` | `payments_api` | `payments` |
| `ai_solutions_api` | `ai-solutions` | `projects_api` | `projects` |
| `apps_api` | `apps` | `routers_api` | `routers` |
| `balancers_api` | `balancers` | `s3_api` | `s3` |
| `cdn_api` | `cdn` | `servers_api` | `servers` |
| `container_registry_api` | `container-registry` | `ssh_api` | `ssh` |
| `databases_api` | `databases` | `vpc_api` | `vpc` |
| `dedicated_servers_api` | `dedicated-servers` | | |
| `domains_api` | `domains` | | |
| `firewall_api` | `firewall` | | |
| `floating_ip_api` | `floating-ip` | | |
| `images_api` | `images` | | |

Флаг `full` включает все области сразу. Флаги по умолчанию — `native-tls` и
`full`.

## Одна область

```toml
timeweb-rs = { version = "0.9", default-features = false, features = ["native-tls", "servers"] }
```

В сборку попадут только модели, которые использует область `servers`, — остальные
тысячи моделей останутся за кадром. Модель, общая для двух областей,
компилируется, если включена любая из них.

## Актуальность списка

Список выше собран из спецификации. Авторитетный список — в манифесте, а CI
проверяет, что манифест и сгенерированный код не разошлись, на каждом pull
request.
