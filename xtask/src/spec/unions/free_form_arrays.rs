//! The unions whose list members have no item type.

use serde_json::{Map, Value};

use crate::spec::json::{as_object_mut, walk_objects};

/// Types the free-form list members of a union as lists of strings.
///
/// The error envelope describes its `message` field as "a string, or a list
/// of messages", but the list member has no item type, so the generator emits
/// an empty struct model and a string body fails to deserialize.
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
            for member in members.iter_mut() {
                let Some(member_map) = as_object_mut(member) else {
                    continue;
                };
                if member_map.get("type").and_then(Value::as_str) != Some("array") {
                    continue;
                }
                let free_form = member_map
                    .get("items")
                    .and_then(Value::as_object)
                    .is_some_and(|items| items.is_empty());
                if !free_form {
                    continue;
                }
                let mut items = Map::new();
                items.insert("type".to_string(), Value::String("string".to_string()));
                member_map.insert("items".to_string(), Value::Object(items));
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::relax;

    #[test]
    fn free_form_array_members_become_string_lists() {
        let mut spec = json!({
            "properties": {
                "message": {
                    "anyOf": [{ "type": "string" }, { "type": "array", "items": {} }]
                }
            }
        });
        relax(&mut spec);
        assert_eq!(
            spec["properties"]["message"]["anyOf"][1]["items"],
            json!({ "type": "string" })
        );
    }

    #[test]
    fn typed_array_members_are_left_alone() {
        let mut spec = json!({
            "properties": {
                "message": {
                    "anyOf": [{ "type": "string" }, { "type": "array", "items": { "type": "string" } }]
                }
            }
        });
        relax(&mut spec);
        assert_eq!(
            spec["properties"]["message"]["anyOf"][1]["items"],
            json!({ "type": "string" })
        );
    }
}
