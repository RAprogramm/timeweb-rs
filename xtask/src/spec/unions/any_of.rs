//! The `anyOf` unions the Rust generator renders as empty structs.

use serde_json::Value;

use crate::spec::json::{as_str, walk_objects};

/// Rewrites the `anyOf` unions the generator renders as empty structs.
///
/// The generator turns an `anyOf` of a string and a list into an empty
/// struct model, so a string body fails to deserialize with "invalid type:
/// string, expected struct BaseErrorMessage". The same union written as a
/// `oneOf` becomes an untagged enum with a variant per member, which parses
/// both shapes.
///
/// The members of such a union are mutually exclusive — a value is either a
/// string or a list, never both — so the rewrite does not change the meaning
/// of the schema.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn relax(spec: &mut Value) {
    walk_objects(spec, &mut |map| {
        let Some(members) = map.get_mut("anyOf").and_then(Value::as_array_mut) else {
            return;
        };
        let typed = members.iter().all(|member| member.get("type").is_some());
        let all_objects = members
            .iter()
            .all(|member| member.get("type").and_then(as_str) == Some("object"));
        if !typed || all_objects {
            return;
        }
        let members = std::mem::take(members);
        map.remove("anyOf");
        map.insert("oneOf".to_string(), Value::Array(members));
    });
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::relax;

    #[test]
    fn scalar_unions_become_one_of() {
        let mut spec = json!({
            "properties": {
                "message": {
                    "anyOf": [{ "type": "string" }, { "type": "array", "items": { "type": "string" } }]
                }
            }
        });
        relax(&mut spec);
        assert!(spec["properties"]["message"].get("anyOf").is_none());
        assert!(spec["properties"]["message"].get("oneOf").is_some());
    }

    #[test]
    fn object_unions_are_left_alone() {
        let mut spec = json!({
            "properties": {
                "params": {
                    "anyOf": [{ "type": "object" }, { "type": "object" }]
                }
            }
        });
        relax(&mut spec);
        assert!(spec["properties"]["params"].get("anyOf").is_some());
    }
}
