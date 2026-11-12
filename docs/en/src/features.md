# Feature flags

The crate ships one Cargo feature per API area, named after the area with
underscores replaced by dashes:

| Area | Feature | Area | Feature |
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

The `full` feature turns every area on, and the default features are
`native-tls` and `full`.

## Compile a single area

```toml
timeweb-rs = { version = "0.9", default-features = false, features = ["native-tls", "servers"] }
```

Only the models the `servers` area uses are compiled — the other thousand
models are left out. A model shared by two areas is compiled when either of
them is enabled.

## Check the names

The list above is generated from the specification. The authoritative list
lives in the manifest, and the CI verifies that the manifest and the
generated code stay in sync on every pull request.
