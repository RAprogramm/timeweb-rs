//! The unions whose members are all strings.

use serde_json::Value;

use crate::spec::json::{as_str, walk_objects};

/// Collapses the unions whose members are all strings.
///
/// A property the spec describes as "an IPv4 or an IPv6 address" becomes an
/// empty struct model, and the string the API sends fails to deserialize with
/// "invalid type: string, expected struct `FirewallRuleCidr`".
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn collapse(spec: &mut Value) {
    walk_objects(spec, &mut |map| {
        for key in ["oneOf", "anyOf"] {
            let Some(members) = map.get_mut(key).and_then(Value::as_array_mut) else {
                continue;
            };
            if members.is_empty()
                || !members
                    .iter()
                    .all(|member| member.get("type").and_then(as_str) == Some("string"))
            {
                continue;
            }
            let description = map.get("description").cloned();
            map.clear();
            map.insert("type".to_string(), Value::String("string".to_string()));
            if let Some(description) = description {
                map.insert("description".to_string(), description);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::collapse;

    #[test]
    fn string_unions_collapse_into_a_string() {
        let mut spec = json!({
            "properties": {
                "cidr": {
                    "anyOf": [
                        { "type": "string", "format": "ipv4" },
                        { "type": "string", "format": "ipv6" }
                    ]
                }
            }
        });
        collapse(&mut spec);
        assert_eq!(spec["properties"]["cidr"], json!({ "type": "string" }));
    }

    #[test]
    fn mixed_unions_are_left_alone() {
        let mut spec = json!({
            "properties": {
                "message": {
                    "anyOf": [{ "type": "string" }, { "type": "array", "items": { "type": "string" } }]
                }
            }
        });
        collapse(&mut spec);
        assert!(spec["properties"]["message"].get("anyOf").is_some());
    }
}
