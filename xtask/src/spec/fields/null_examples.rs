//! The properties the specification documents as `null`.

use serde_json::Value;

use crate::spec::json::{as_object_mut, walk_objects};

/// Marks the properties the spec documents as `null` as nullable.
///
/// A property whose example is `null` is a property the API sends as `null`.
/// Without the nullable marker the generator types it as a plain value and the
/// response fails to deserialize with "invalid type: null, expected ...".
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn relax(spec: &mut Value) {
    walk_objects(spec, &mut |map| {
        let Some(properties) = map.get_mut("properties").and_then(as_object_mut) else {
            return;
        };
        for property in properties.values_mut() {
            let Some(property_map) = as_object_mut(property) else {
                continue;
            };
            let documented_as_null = property_map
                .get("example")
                .is_some_and(serde_json::Value::is_null);
            if documented_as_null {
                property_map.insert("nullable".to_string(), Value::Bool(true));
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::relax;

    #[test]
    fn null_examples_become_nullable_properties() {
        let mut spec = json!({
            "properties": {
                "id": { "type": "string", "example": null },
                "name": { "type": "string", "example": "value" }
            }
        });
        relax(&mut spec);
        assert_eq!(spec["properties"]["id"]["nullable"], json!(true));
        assert!(spec["properties"]["name"].get("nullable").is_none());
    }
}
