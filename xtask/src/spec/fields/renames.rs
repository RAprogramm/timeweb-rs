//! The response properties whose name in the spec differs from the API one.

use serde_json::Value;

use crate::spec::json::{as_object_mut, as_str, walk_objects};

/// The renamed response properties, from the spec spelling to the API one.
const RENAMED_PROPERTIES: [(&str, &str); 2] = [
    ("ssh-keys", "ssh_keys"),
    ("knowledgebases", "knowledge_bases")
];

/// Renames the response properties whose spec name differs from the API's.
///
/// Some list responses name their collection property differently from what
/// the live API sends, so the generated client deserializes an empty list (or
/// fails on a missing field):
///
/// * `GET /api/v1/ssh-keys` — spec `ssh-keys` vs API `ssh_keys`;
/// * `GET /api/v1/cloud-ai/knowledge-bases` — spec `knowledgebases` vs API
///   `knowledge_bases`.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn rename(spec: &mut Value) {
    walk_objects(spec, &mut |map| {
        {
            let Some(properties) = map.get_mut("properties").and_then(as_object_mut) else {
                return;
            };
            for (old, new) in RENAMED_PROPERTIES {
                let Some(property) = properties.remove(old) else {
                    continue;
                };
                properties.insert(new.to_string(), property);
            }
        }
        let Some(required) = map.get_mut("required").and_then(Value::as_array_mut) else {
            return;
        };
        for entry in required.iter_mut() {
            let Some(name) = as_str(entry).map(str::to_string) else {
                continue;
            };
            let Some((_, new)) = RENAMED_PROPERTIES.iter().find(|(old, _)| *old == name) else {
                continue;
            };
            *entry = Value::String((*new).to_string());
        }
    });
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::rename;

    #[test]
    fn mismatched_property_names_are_renamed() {
        let mut spec = json!({
            "properties": { "ssh-keys": { "type": "array" } },
            "required": ["ssh-keys"]
        });
        rename(&mut spec);
        assert!(spec["properties"].get("ssh-keys").is_none());
        assert!(spec["properties"].get("ssh_keys").is_some());
        assert_eq!(spec["required"], json!(["ssh_keys"]));
    }
}
