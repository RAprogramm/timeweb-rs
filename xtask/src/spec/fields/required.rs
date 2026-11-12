//! The `required` lists that demand more than the API sends.

use serde_json::Value;

use crate::spec::json::{as_object, as_object_mut, as_str, schema_mut};

/// The properties every listed schema reliably returns.
const MINIMAL_REQUIRED: [(&str, &[&str]); 1] = [("app", &["id", "name", "status"])];

/// The required properties the live API omits, by schema.
const OMITTED_REQUIRED: [(&str, &str); 6] = [
    ("image", "deleted_at"),
    ("UploadSuccessfulResponse", "upload_successful"),
    ("resource", "uri"),
    ("firewall-group", "policy"),
    ("FirewallGroupInAPI", "policy"),
    ("NodeOut", "node_ip")
];

/// Trims the `required` lists that demand fields the live API omits.
///
/// Some list-item schemas mark many fields `required` that the live API does
/// not actually send. The `app` schema, for example, requires `framework`,
/// `branch_name` and `server_id` — none of which appear in a
/// `GET /api/v1/apps` item (the API sends `branch`, not `branch_name`, and no
/// `framework`). One missing required field fails deserialization of the whole
/// collection, so the dashboard shows zero apps despite the account having
/// several.
///
/// The listed schemas keep only the minimal identifying fields the API
/// reliably returns; the rest become `Option` and tolerate absence.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn relax_overstrict(spec: &mut Value) {
    for (name, minimal) in MINIMAL_REQUIRED {
        let Some(schema) = schema_mut(spec, name) else {
            continue;
        };
        if !schema.get("required").is_some_and(Value::is_array) {
            continue;
        }
        let properties = schema.get("properties").and_then(as_object);
        let required = minimal
            .iter()
            .filter(|field| properties.is_some_and(|properties| properties.contains_key(**field)))
            .map(|field| Value::String((*field).to_string()))
            .collect();
        schema.insert("required".to_string(), Value::Array(required));
    }
}

/// Drops the properties the live API omits from the required lists.
///
/// The `image` schema requires `deleted_at`, the image-upload response
/// requires `upload_successful` and the `resource` schema requires `uri`, but
/// the responses the API documents omit them. A missing required property
/// fails the deserialization of the whole response.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn relax_omitted(spec: &mut Value) {
    for (schema, property) in OMITTED_REQUIRED {
        let Some(schema) = schema_mut(spec, schema) else {
            continue;
        };
        let Some(required) = schema.get_mut("required").and_then(Value::as_array_mut) else {
            continue;
        };
        required.retain(|entry| as_str(entry) != Some(property));
    }
}

/// Drops `requests` from the apps frontend-preset required list.
///
/// `apps-presets.frontend_presets.items` marks `requests` as required, but
/// `GET /api/v1/presets/apps` never sends it (the backend-preset schema does
/// not require it either). The missing field fails deserialization of the
/// whole presets collection, so `apps list-presets` errors out.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn relax_apps_preset(spec: &mut Value) {
    let Some(presets) = schema_mut(spec, "apps-presets") else {
        return;
    };
    let Some(items) = presets
        .get_mut("properties")
        .and_then(as_object_mut)
        .and_then(|properties| properties.get_mut("frontend_presets"))
        .and_then(as_object_mut)
        .and_then(|presets| presets.get_mut("items"))
        .and_then(as_object_mut)
    else {
        return;
    };
    let Some(required) = items.get_mut("required").and_then(Value::as_array_mut) else {
        return;
    };
    required.retain(|entry| as_str(entry) != Some("requests"));
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{relax_apps_preset, relax_omitted, relax_overstrict};

    #[test]
    fn overstrict_required_lists_are_trimmed() {
        let mut spec = json!({
            "components": {
                "schemas": {
                    "app": {
                        "type": "object",
                        "required": ["id", "name", "status", "framework"],
                        "properties": {
                            "id": { "type": "integer" },
                            "name": { "type": "string" },
                            "status": { "type": "string" },
                            "framework": { "type": "string" }
                        }
                    }
                }
            }
        });
        relax_overstrict(&mut spec);
        assert_eq!(
            spec["components"]["schemas"]["app"]["required"],
            json!(["id", "name", "status"])
        );
    }

    #[test]
    fn the_frontend_preset_requests_field_is_dropped() {
        let mut spec = json!({
            "components": {
                "schemas": {
                    "apps-presets": {
                        "properties": {
                            "frontend_presets": {
                                "items": { "required": ["requests", "backend"] }
                            }
                        }
                    }
                }
            }
        });
        relax_apps_preset(&mut spec);
        assert_eq!(
            spec["components"]["schemas"]["apps-presets"]["properties"]["frontend_presets"]["items"]
                ["required"],
            json!(["backend"])
        );
    }

    #[test]
    fn the_omitted_properties_are_dropped() {
        let mut spec = json!({
            "components": {
                "schemas": {
                    "image": {
                        "required": ["id", "deleted_at"],
                        "properties": {
                            "id": { "type": "string" },
                            "deleted_at": { "type": "string" }
                        }
                    }
                }
            }
        });
        relax_omitted(&mut spec);
        assert_eq!(
            spec["components"]["schemas"]["image"]["required"],
            json!(["id"])
        );
    }
}
