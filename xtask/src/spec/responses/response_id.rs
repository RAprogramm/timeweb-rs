//! The `response_id` field of the response envelopes.

use serde_json::Value;

use crate::spec::json::{as_object_mut, schema_mut, walk_objects};

/// Marks `response_id` nullable so a `null` from the API deserializes.
///
/// The upstream spec types `response_id` as a required `uuid` and the
/// response wrappers list it in `required`, so the generator emits
/// `response_id: uuid::Uuid`. The live API does not honour this — some
/// endpoints (e.g. `GET /account/finances`) return `response_id: null`,
/// which then fails deserialization with "invalid type: null, expected a
/// formatted UUID string". Marking the shared schema (and any inline
/// occurrence) nullable yields `Option<uuid::Uuid>`, so `null` maps to `None`
/// instead of erroring.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn relax(spec: &mut Value) {
    if let Some(shared) = schema_mut(spec, "response_id") {
        shared.insert("nullable".to_string(), Value::Bool(true));
    }
    walk_objects(spec, &mut |map| {
        let Some(properties) = map.get_mut("properties").and_then(as_object_mut) else {
            return;
        };
        let Some(inline) = properties.get_mut("response_id").and_then(as_object_mut) else {
            return;
        };
        if inline.contains_key("$ref") {
            return;
        }
        inline.insert("nullable".to_string(), Value::Bool(true));
    });
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::relax;

    #[test]
    fn response_id_becomes_nullable() {
        let mut spec = json!({
            "components": { "schemas": { "response_id": { "type": "string" } } },
            "properties": { "response_id": { "type": "string" } }
        });
        relax(&mut spec);
        assert_eq!(
            spec["components"]["schemas"]["response_id"]["nullable"],
            json!(true)
        );
        assert_eq!(spec["properties"]["response_id"]["nullable"], json!(true));
    }
}
