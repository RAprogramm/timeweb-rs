//! The unions whose array members the Rust generator cannot name.

use serde_json::Value;

use crate::spec::json::{as_object, walk_objects};

/// Returns whether a union member is a list of binary files.
fn is_binary_array_member(member: &Value) -> bool {
    let Some(map) = as_object(member) else {
        return false;
    };
    if map.get("type").and_then(Value::as_str) != Some("array") {
        return false;
    }
    let Some(items) = map.get("items").and_then(as_object) else {
        return false;
    };
    items.get("type").and_then(Value::as_str) == Some("string")
        && items.get("format").and_then(Value::as_str) == Some("binary")
}

/// Returns whether a union member is a list of free-form objects.
fn is_free_form_array_member(member: &Value) -> bool {
    let Some(map) = as_object(member) else {
        return false;
    };
    if map.get("type").and_then(Value::as_str) != Some("array") {
        return false;
    }
    let Some(items) = map.get("items").and_then(as_object) else {
        return false;
    };
    items.get("type").and_then(Value::as_str) == Some("object") && !items.contains_key("$ref")
}

/// Rewrites the unions the Rust generator renders as invalid Rust.
///
/// The upstream spec describes a few fields as "a string, or a list of
/// free-form objects" (the AI Gateway validation errors, chat message parts,
/// response inputs) and one as "a file, or a list of files" (the image edit
/// request). `openapi-generator` turns such a union into an untagged enum and
/// names the array variant after the item type: a list of strings yields the
/// valid `ArrayVecString(Vec<String>)`, but a list of free-form objects yields
/// the invalid identifier `ArrayVecstd::collections::HashMap(...)` and a list
/// of files yields `ArrayVecstd::path::PathBuf(...)` — the generated crate
/// does not compile.
///
/// The offending unions are replaced with a single schema the generator maps
/// to a workable Rust type: free-form JSON for the object case (it accepts a
/// string, a list or an object) and a file path for the binary case.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn relax(spec: &mut Value) {
    walk_objects(spec, &mut |map| {
        for key in ["oneOf", "anyOf"] {
            let Some(members) = map.get_mut(key).and_then(Value::as_array_mut) else {
                continue;
            };
            let binary = members.iter().any(is_binary_array_member);
            let free_form = members.iter().any(is_free_form_array_member);
            if !binary && !free_form {
                continue;
            }
            let description = map.get("description").cloned();
            map.clear();
            if let Some(description) = description {
                map.insert("description".to_string(), description);
            }
            if binary {
                map.insert("type".to_string(), Value::String("string".to_string()));
                map.insert("format".to_string(), Value::String("binary".to_string()));
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::relax;

    #[test]
    fn free_form_array_unions_are_replaced() {
        let mut spec = json!({
            "properties": {
                "detail": {
                    "oneOf": [
                        { "type": "string" },
                        { "type": "array", "items": { "type": "object", "additionalProperties": true } }
                    ]
                }
            }
        });
        relax(&mut spec);
        assert!(spec["properties"]["detail"].get("oneOf").is_none());
        assert!(spec["properties"]["detail"].get("type").is_none());
    }

    #[test]
    fn binary_array_unions_become_a_file_path() {
        let mut spec = json!({
            "properties": {
                "image": {
                    "oneOf": [
                        { "type": "string", "format": "binary" },
                        { "type": "array", "items": { "type": "string", "format": "binary" } }
                    ]
                }
            }
        });
        relax(&mut spec);
        assert_eq!(spec["properties"]["image"]["type"], json!("string"));
        assert_eq!(spec["properties"]["image"]["format"], json!("binary"));
    }

    #[test]
    fn string_arrays_are_left_alone() {
        let mut spec = json!({
            "properties": {
                "stop": {
                    "oneOf": [
                        { "type": "string" },
                        { "type": "array", "items": { "type": "string" } }
                    ]
                }
            }
        });
        relax(&mut spec);
        assert!(spec["properties"]["stop"].get("oneOf").is_some());
    }
}
