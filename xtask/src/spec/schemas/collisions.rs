//! The component schemas whose names collide case-insensitively.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::spec::json::{as_str, schemas_mut, walk_objects};

/// Renames the component schemas whose names collide case-insensitively.
///
/// The generator writes one file per schema and derives the file name from
/// the schema name, so two schemas that differ only in case end up in the
/// same file: the specification has both `Resource` (the cluster resource
/// usage) and `resource` (an API resource), and only one of them survives the
/// generation — the model of the other one no longer matches the schema, and
/// the responses that use it fail to deserialize.
///
/// The schema with the most references keeps its name; the others get a
/// numeric suffix, and every reference to them is rewritten.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn rename(spec: &mut Value) {
    let Some(schemas) = schemas_mut(spec) else {
        return;
    };
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for name in schemas.keys() {
        groups
            .entry(name.to_lowercase())
            .or_default()
            .push(name.clone());
    }

    let mut renames: Vec<(String, String)> = Vec::new();
    for (_, mut names) in groups {
        if names.len() < 2 {
            continue;
        }
        names.sort();
        let references: BTreeMap<String, usize> = names
            .iter()
            .map(|name| (name.clone(), count_references(spec, name)))
            .collect();
        names.sort_by_key(|name| {
            (
                std::cmp::Reverse(references.get(name).copied().unwrap_or_default()),
                name.clone()
            )
        });
        let (keeper, colliding) = names.split_first().expect("the group is not empty");
        let _ = keeper;
        for (index, name) in colliding.iter().enumerate() {
            let renamed = format!("{name}{}", index + 2);
            renames.push((name.clone(), renamed));
        }
    }
    if renames.is_empty() {
        return;
    }

    for (from, to) in &renames {
        let reference = format!("#/components/schemas/{from}");
        let replacement = format!("#/components/schemas/{to}");
        walk_objects(spec, &mut |map| {
            if let Some(value) = map.get_mut("$ref") {
                if as_str(value) == Some(reference.as_str()) {
                    *value = Value::String(replacement.clone());
                }
            }
        });
    }
    for (from, to) in &renames {
        let Some(schemas) = schemas_mut(spec) else {
            return;
        };
        let Some(schema) = schemas.remove(from) else {
            continue;
        };
        schemas.insert(to.clone(), schema);
    }
}

/// The number of references to a component schema.
fn count_references(spec: &Value, name: &str) -> usize {
    let reference = format!("#/components/schemas/{name}");
    let mut count = 0;
    count_in(spec, &reference, &mut count);
    count
}

/// Counts the references to a schema in a value.
fn count_in(node: &Value, reference: &str, count: &mut usize) {
    match node {
        Value::Object(map) => {
            if map.get("$ref").and_then(as_str) == Some(reference) {
                *count += 1;
            }
            for value in map.values() {
                count_in(value, reference, count);
            }
        }
        Value::Array(items) => {
            for item in items {
                count_in(item, reference, count);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::rename;

    #[test]
    fn colliding_names_are_renamed() {
        let mut spec = json!({
            "components": {
                "schemas": {
                    "Resource": { "type": "object", "properties": { "used": { "type": "number" } } },
                    "resource": { "type": "object", "properties": { "uri": { "type": "string" } } }
                }
            },
            "properties": {
                "usage": { "$ref": "#/components/schemas/Resource" }
            }
        });
        rename(&mut spec);
        let schemas = &spec["components"]["schemas"];
        assert!(schemas.get("Resource").is_some());
        assert!(schemas.get("resource").is_none());
        assert!(schemas.get("resource2").is_some());
        assert_eq!(
            spec["properties"]["usage"]["$ref"],
            json!("#/components/schemas/Resource")
        );
    }

    #[test]
    fn distinct_names_are_left_alone() {
        let mut spec = json!({
            "components": {
                "schemas": {
                    "Resource": { "type": "object" },
                    "Resources": { "type": "object" }
                }
            }
        });
        rename(&mut spec);
        let schemas = &spec["components"]["schemas"];
        assert!(schemas.get("Resource").is_some());
        assert!(schemas.get("Resources").is_some());
        assert!(schemas.get("Resource2").is_none());
    }
}
