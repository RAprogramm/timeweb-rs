//! The project resources the live API returns but the spec omits.

use serde_json::{Map, Value};

use crate::spec::json::{as_object_mut, schema_mut};

/// The route that lists the resources of a project.
const RESOURCES_ROUTE: &str = "/api/v1/projects/{project_id}/resources";

/// Exposes the project resources and the app-to-project linkage the API
/// returns.
///
/// The live `GET /projects/{project_id}/resources` returns far more
/// collections than the spec documents (verified against production):
/// `apps`, `agents`, `domains`, `knowledgebases`, `mailboxes`, `registries`,
/// `cdn`, `monitorings` and `routers` alongside the six documented ones. Only
/// `apps` is added here — its element shape was verified to match the
/// documented `app` schema exactly; the other collections stay untouched until
/// their shapes can be checked against live data.
///
/// Live `app` objects also carry a `project_id` the spec omits — the only way
/// to attribute an app to a project from the apps list.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn add_fields(spec: &mut Value) {
    if let Some(app) = schema_mut(spec, "app") {
        let properties = app
            .entry("properties".to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        if let Some(properties) = as_object_mut(properties) {
            if !properties.contains_key("project_id") {
                let mut schema = Map::new();
                schema.insert("type".to_string(), Value::String("integer".to_string()));
                schema.insert("nullable".to_string(), Value::Bool(true));
                properties.insert("project_id".to_string(), Value::Object(schema));
            }
        }
    }

    let Some(paths) = spec.get_mut("paths").and_then(Value::as_object_mut) else {
        return;
    };
    let Some(project) = paths.get_mut(RESOURCES_ROUTE).and_then(as_object_mut) else {
        return;
    };
    let Some(operation) = project.get_mut("get").and_then(as_object_mut) else {
        return;
    };
    let Some(responses) = operation
        .get_mut("responses")
        .and_then(as_object_mut)
        .and_then(|responses| responses.get_mut("200"))
        .and_then(as_object_mut)
    else {
        return;
    };
    let Some(content) = responses
        .get_mut("content")
        .and_then(as_object_mut)
        .and_then(|content| content.get_mut("application/json"))
        .and_then(as_object_mut)
    else {
        return;
    };
    let Some(schema) = content.get_mut("schema").and_then(as_object_mut) else {
        return;
    };
    let Some(parts) = schema.get_mut("allOf").and_then(Value::as_array_mut) else {
        return;
    };
    for part in parts.iter_mut() {
        let Some(properties) = part.get_mut("properties").and_then(as_object_mut) else {
            continue;
        };
        if !properties.contains_key("servers") || properties.contains_key("apps") {
            continue;
        }
        let mut item = Map::new();
        item.insert(
            "$ref".to_string(),
            Value::String("#/components/schemas/app".to_string())
        );
        let mut array = Map::new();
        array.insert("type".to_string(), Value::String("array".to_string()));
        array.insert("items".to_string(), Value::Object(item));
        array.insert("nullable".to_string(), Value::Bool(true));
        properties.insert("apps".to_string(), Value::Object(array));
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::add_fields;

    #[test]
    fn the_app_project_id_is_added() {
        let mut spec = json!({
            "components": {
                "schemas": { "app": { "type": "object", "properties": { "id": { "type": "integer" } } } }
            }
        });
        add_fields(&mut spec);
        assert_eq!(
            spec["components"]["schemas"]["app"]["properties"]["project_id"]["nullable"],
            json!(true)
        );
    }

    #[test]
    fn the_project_apps_collection_is_added() {
        let mut spec = json!({
            "paths": {
                "/api/v1/projects/{project_id}/resources": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "allOf": [
                                                { "properties": { "servers": { "type": "array" } } }
                                            ]
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
        add_fields(&mut spec);
        let properties = &spec["paths"]["/api/v1/projects/{project_id}/resources"]["get"]["responses"]
            ["200"]["content"]["application/json"]["schema"]["allOf"][0]["properties"];
        assert!(properties.get("apps").is_some());
    }
}
