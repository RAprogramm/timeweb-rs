//! The identifier fields the specification types as numbers.

use serde_json::Value;

use crate::spec::json::{as_object_mut, as_str, walk_objects};

/// Types the identifier fields as integers instead of floats.
///
/// The upstream spec types numeric ids (`id` and `*_id`) as `number`, so the
/// generated Rust model uses `f64` and JSON output renders them as floats
/// (e.g. `1873345.0`). Genuine decimals (`balance`, `*_cost`, ...) are left
/// untouched because their names do not match.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn relax(spec: &mut Value) {
    walk_objects(spec, &mut |map| {
        let Some(properties) = map.get_mut("properties").and_then(as_object_mut) else {
            return;
        };
        for (name, schema) in properties.iter_mut() {
            let is_id = name == "id" || name.ends_with("_id");
            if !is_id {
                continue;
            }
            let Some(schema) = as_object_mut(schema) else {
                continue;
            };
            if schema.get("type").and_then(as_str) != Some("number") {
                continue;
            }
            schema.insert("type".to_string(), Value::String("integer".to_string()));
            schema.insert("format".to_string(), Value::String("int64".to_string()));
        }
    });
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::relax;

    #[test]
    fn identifier_numbers_become_integers() {
        let mut spec = json!({
            "properties": {
                "id": { "type": "number" },
                "project_id": { "type": "number" },
                "balance": { "type": "number" }
            }
        });
        relax(&mut spec);
        assert_eq!(spec["properties"]["id"]["type"], json!("integer"));
        assert_eq!(spec["properties"]["id"]["format"], json!("int64"));
        assert_eq!(spec["properties"]["project_id"]["type"], json!("integer"));
        assert_eq!(spec["properties"]["balance"]["type"], json!("number"));
    }
}
