//! The localization of the operation tags.

use serde_json::Value;

use crate::spec::json::{as_object_mut, as_str, child_mut};

/// The HTTP methods the specification describes operations for.
const METHODS: [&str; 5] = ["get", "post", "put", "patch", "delete"];

/// Replaces the Russian tag names with the English names the spec carries.
///
/// Every tag in the upstream spec is written in Russian, which the generator
/// cannot turn into Rust module names — all operations collapse into a single
/// `default_api` module. Each tag carries its English name in
/// `x-name-i18n.eng` (operations additionally in `x-tags-i18n`), so the
/// generator emits one module per API area (`servers_api`, `databases_api`,
/// ...).
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn localize_tags(spec: &mut Value) {
    let mut mapping: Vec<(String, String)> = Vec::new();
    if let Some(tags) = child_mut(spec, "tags").and_then(Value::as_array_mut) {
        for tag in tags.iter_mut() {
            let Some(map) = as_object_mut(tag) else {
                continue;
            };
            let english = map
                .get("x-name-i18n")
                .and_then(|value| value.get("eng"))
                .and_then(as_str)
                .map(str::to_string);
            let Some(english) = english else {
                continue;
            };
            if let Some(name) = map.get("name").and_then(as_str) {
                mapping.push((name.to_string(), english.clone()));
            }
            map.insert("name".to_string(), Value::String(english));
        }
    }

    let Some(paths) = child_mut(spec, "paths").and_then(as_object_mut) else {
        return;
    };
    for item in paths.values_mut() {
        let Some(operations) = as_object_mut(item) else {
            continue;
        };
        for (method, operation) in operations.iter_mut() {
            if !METHODS.contains(&method.as_str()) {
                continue;
            }
            let Some(operation) = as_object_mut(operation) else {
                continue;
            };
            let english = operation
                .get("x-tags-i18n")
                .and_then(|value| value.get("eng"))
                .and_then(Value::as_array)
                .map(|tags| {
                    tags.iter()
                        .filter_map(as_str)
                        .map(str::to_string)
                        .collect::<Vec<String>>()
                });
            match english {
                Some(tags) if !tags.is_empty() => {
                    operation.insert(
                        "tags".to_string(),
                        Value::Array(tags.into_iter().map(Value::String).collect())
                    );
                }
                _ => {
                    let Some(current) = operation.get_mut("tags").and_then(Value::as_array_mut)
                    else {
                        continue;
                    };
                    for tag in current.iter_mut() {
                        let Some(name) = as_str(tag).map(str::to_string) else {
                            continue;
                        };
                        let english = mapping
                            .iter()
                            .find(|(russian, _)| russian == &name)
                            .map(|(_, english)| english.clone());
                        if let Some(english) = english {
                            *tag = Value::String(english);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::localize_tags;

    #[test]
    fn russian_tags_become_english_ones() {
        let mut spec = json!({
            "tags": [{ "name": "Серверы", "x-name-i18n": { "eng": "servers" } }],
            "paths": {
                "/api/v1/servers": {
                    "get": { "operationId": "getServers", "tags": ["Серверы"] }
                }
            }
        });
        localize_tags(&mut spec);
        assert_eq!(spec["tags"][0]["name"], json!("servers"));
        assert_eq!(
            spec["paths"]["/api/v1/servers"]["get"]["tags"],
            json!(["servers"])
        );
    }

    #[test]
    fn the_english_tags_of_an_operation_win() {
        let mut spec = json!({
            "tags": [{ "name": "Серверы", "x-name-i18n": { "eng": "servers" } }],
            "paths": {
                "/api/v1/servers": {
                    "get": {
                        "operationId": "getServers",
                        "tags": ["Серверы"],
                        "x-tags-i18n": { "eng": ["cloud-servers"] }
                    }
                }
            }
        });
        localize_tags(&mut spec);
        assert_eq!(
            spec["paths"]["/api/v1/servers"]["get"]["tags"],
            json!(["cloud-servers"])
        );
    }
}
