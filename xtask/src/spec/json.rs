//! Helpers for navigating the JSON document of a specification.

use serde_json::{Map, Value};

/// Visits every object of the specification, depth first.
///
/// # Arguments
///
/// * `value` — the value to walk.
/// * `visit` — the action to run on every object.
pub(super) fn walk_objects(value: &mut Value, visit: &mut impl FnMut(&mut Map<String, Value>)) {
    match value {
        Value::Object(map) => {
            visit(map);
            for child in map.values_mut() {
                walk_objects(child, visit);
            }
        }
        Value::Array(items) => {
            for child in items {
                walk_objects(child, visit);
            }
        }
        _ => {}
    }
}

/// Returns the object of a value, if it is one.
pub(super) fn as_object(value: &Value) -> Option<&Map<String, Value>> {
    value.as_object()
}

/// Returns the mutable object of a value, if it is one.
pub(super) fn as_object_mut(value: &mut Value) -> Option<&mut Map<String, Value>> {
    value.as_object_mut()
}

/// Returns the string of a value, if it is one.
pub(super) fn as_str(value: &Value) -> Option<&str> {
    value.as_str()
}

/// Returns the mutable child of a value under a key.
pub(super) fn child_mut<'a>(value: &'a mut Value, key: &str) -> Option<&'a mut Value> {
    value.as_object_mut()?.get_mut(key)
}

/// Returns the mutable schemas of the specification.
pub(super) fn schemas_mut(spec: &mut Value) -> Option<&mut Map<String, Value>> {
    spec.get_mut("components")
        .and_then(|value| value.get_mut("schemas"))
        .and_then(as_object_mut)
}

/// Returns the mutable schema of the specification under a name.
pub(super) fn schema_mut<'a>(
    spec: &'a mut Value,
    name: &str
) -> Option<&'a mut Map<String, Value>> {
    schemas_mut(spec)?.get_mut(name).and_then(as_object_mut)
}
