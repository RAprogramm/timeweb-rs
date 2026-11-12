//! The path parameters of the routes.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::spec::json::{as_object, as_object_mut, as_str, child_mut};

/// The HTTP methods the specification describes operations for.
const METHODS: [&str; 5] = ["get", "post", "put", "patch", "delete"];

/// Returns the placeholder names embedded in a route template.
fn placeholders(route: &str) -> Vec<&str> {
    let mut names = Vec::new();
    let mut rest = route;
    while let Some(start) = rest.find('{') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('}') else {
            break;
        };
        names.push(&after[..end]);
        rest = &after[end + 1..];
    }
    names
}

/// Resolves a parameter reference against the shared parameter components.
fn resolve<'a>(param: &'a Value, components: Option<&'a Map<String, Value>>) -> &'a Value {
    let Some(reference) = as_str(param).or_else(|| param.get("$ref").and_then(as_str)) else {
        return param;
    };
    let Some(name) = reference.strip_prefix("#/components/parameters/") else {
        return param;
    };
    components.and_then(|map| map.get(name)).unwrap_or(param)
}

/// Renames the path parameters that do not match their route placeholder.
fn fix_parameter_list(
    parameters: &mut [Value],
    names: &[&str],
    components: Option<&Map<String, Value>>
) {
    for parameter in parameters.iter_mut() {
        let resolved = resolve(parameter, components).clone();
        if resolved.get("in").and_then(as_str) != Some("path") {
            continue;
        }
        let Some(name) = resolved.get("name").and_then(as_str).map(str::to_string) else {
            continue;
        };
        if names.contains(&name.as_str()) {
            continue;
        }
        let Some(map) = as_object_mut(parameter) else {
            continue;
        };
        let mut inlined = resolved;
        let normalized = name.replace('-', "_");
        let Some(inlined_map) = as_object_mut(&mut inlined) else {
            continue;
        };
        if names.contains(&normalized.as_str()) {
            inlined_map.insert("name".to_string(), Value::String(normalized));
        } else {
            inlined_map.insert("in".to_string(), Value::String("query".to_string()));
        }
        *map = inlined_map.clone();
    }
}

/// Makes every route's path parameters consistent with its template.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn fix_path_parameters(spec: &mut Value) {
    let components = spec
        .get("components")
        .and_then(|value| value.get("parameters"))
        .and_then(as_object)
        .cloned();
    let mut shared_by_name: BTreeMap<String, Value> = BTreeMap::new();
    if let Some(components) = &components {
        for component in components.values() {
            if component.get("in").and_then(as_str) == Some("path") {
                if let Some(name) = component.get("name").and_then(as_str) {
                    shared_by_name.insert(name.replace('-', "_"), component.clone());
                }
            }
        }
    }

    let Some(paths) = child_mut(spec, "paths").and_then(as_object_mut) else {
        return;
    };
    for (route, item) in paths.iter_mut() {
        let names = placeholders(route);
        let Some(item) = as_object_mut(item) else {
            continue;
        };
        let had_shared = item.contains_key("parameters");
        let mut shared = item
            .remove("parameters")
            .and_then(|value| value.as_array().cloned())
            .unwrap_or_default();
        fix_parameter_list(&mut shared, &names, components.as_ref());
        for (method, operation) in item.iter_mut() {
            if !METHODS.contains(&method.as_str()) {
                continue;
            }
            let Some(operation) = as_object_mut(operation) else {
                continue;
            };
            let mut parameters = operation
                .remove("parameters")
                .and_then(|value| value.as_array().cloned())
                .unwrap_or_default();
            fix_parameter_list(&mut parameters, &names, components.as_ref());

            let mut declared: Vec<String> = Vec::new();
            for parameter in shared.iter().chain(parameters.iter()) {
                let resolved = resolve(parameter, components.as_ref());
                if resolved.get("in").and_then(as_str) == Some("path") {
                    if let Some(name) = resolved.get("name").and_then(as_str) {
                        declared.push(name.to_string());
                    }
                }
            }
            for name in &names {
                if declared.iter().any(|declared| declared == name) {
                    continue;
                }
                let key = name.replace('-', "_");
                let schema = shared_by_name
                    .get(&key)
                    .and_then(|component| component.get("schema"))
                    .cloned()
                    .unwrap_or_else(|| {
                        let mut schema = Map::new();
                        schema.insert("type".to_string(), Value::String("string".to_string()));
                        Value::Object(schema)
                    });
                let mut parameter = Map::new();
                parameter.insert("name".to_string(), Value::String((*name).to_string()));
                parameter.insert("in".to_string(), Value::String("path".to_string()));
                parameter.insert("required".to_string(), Value::Bool(true));
                parameter.insert("schema".to_string(), schema);
                parameters.push(Value::Object(parameter));
            }
            operation.insert("parameters".to_string(), Value::Array(parameters));
        }
        if had_shared {
            item.insert("parameters".to_string(), Value::Array(shared));
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::fix_path_parameters;

    #[test]
    fn missing_path_parameter_is_declared() {
        let mut spec = json!({
            "paths": {
                "/api/v1/servers/{server_id}": {
                    "get": { "operationId": "getServer" }
                }
            }
        });
        fix_path_parameters(&mut spec);
        let parameters = spec["paths"]["/api/v1/servers/{server_id}"]["get"]["parameters"]
            .as_array()
            .expect("parameters are added");
        assert_eq!(parameters.len(), 1);
        assert_eq!(parameters[0]["name"], json!("server_id"));
        assert_eq!(parameters[0]["in"], json!("path"));
    }

    #[test]
    fn orphan_path_parameter_becomes_a_query_parameter() {
        let mut spec = json!({
            "paths": {
                "/api/v1/servers": {
                    "get": {
                        "operationId": "getServers",
                        "parameters": [
                            { "name": "server_id", "in": "path", "schema": { "type": "string" } }
                        ]
                    }
                }
            }
        });
        fix_path_parameters(&mut spec);
        let parameters = spec["paths"]["/api/v1/servers"]["get"]["parameters"]
            .as_array()
            .expect("parameters stay");
        assert_eq!(parameters[0]["in"], json!("query"));
    }

    #[test]
    fn a_declared_parameter_is_left_alone() {
        let mut spec = json!({
            "paths": {
                "/api/v1/servers/{server_id}": {
                    "get": {
                        "operationId": "getServer",
                        "parameters": [
                            { "name": "server_id", "in": "path", "schema": { "type": "string" } }
                        ]
                    }
                }
            }
        });
        fix_path_parameters(&mut spec);
        let parameters = spec["paths"]["/api/v1/servers/{server_id}"]["get"]["parameters"]
            .as_array()
            .expect("parameters stay");
        assert_eq!(parameters.len(), 1);
        assert_eq!(parameters[0]["in"], json!("path"));
    }
}
