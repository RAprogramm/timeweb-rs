//! The undocumented fields of the account status response.

use serde_json::{Map, Value};

use crate::spec::json::{as_object_mut, schema_mut};

/// The fields the live API returns but the upstream `status` schema omits.
const ADDITIONS: [(&str, &str); 4] = [
    ("login", "string"),
    ("registered_at", "string"),
    ("is_password_set", "boolean"),
    ("two_factor_method", "string")
];

/// Adds the `status` fields the live API returns but the spec omits.
///
/// `GET /account/status` returns `login` (the account identifier shown to the
/// user), `registered_at`, `two_factor_method` and `is_password_set`, but the
/// upstream `status` schema documents none of them — it stops at
/// `ym_client_id`. Without `login` a client is left with only the hoster's
/// `company_info` (the same "ООО ТАЙМВЭБ.КЛАУД" for every account), which is
/// useless for identifying the account.
///
/// The fields are added defensively (not marked required), so the generator
/// emits `Option` types and a missing field never breaks deserialization.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn add_fields(spec: &mut Value) {
    let Some(status) = schema_mut(spec, "status") else {
        return;
    };
    let properties = status
        .entry("properties".to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    let Some(properties) = as_object_mut(properties) else {
        return;
    };
    for (name, kind) in ADDITIONS {
        if properties.contains_key(name) {
            continue;
        }
        let mut schema = Map::new();
        schema.insert("type".to_string(), Value::String(kind.to_string()));
        if name == "two_factor_method" {
            schema.insert("nullable".to_string(), Value::Bool(true));
        }
        properties.insert(name.to_string(), Value::Object(schema));
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::add_fields;

    #[test]
    fn the_undocumented_fields_are_added() {
        let mut spec = json!({
            "components": {
                "schemas": {
                    "status": { "type": "object", "properties": { "ym_client_id": { "type": "string" } } }
                }
            }
        });
        add_fields(&mut spec);
        let properties = &spec["components"]["schemas"]["status"]["properties"];
        assert!(properties.get("login").is_some());
        assert!(properties.get("registered_at").is_some());
        assert!(properties.get("is_password_set").is_some());
        assert_eq!(properties["two_factor_method"]["nullable"], json!(true));
    }
}
