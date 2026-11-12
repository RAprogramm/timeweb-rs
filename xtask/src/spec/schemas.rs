//! The component schemas of the specification.

mod collisions;

use serde_json::Value;

/// Renames the component schemas whose names collide case-insensitively.
pub(super) fn rename_colliding_schemas(spec: &mut Value) {
    collisions::rename(spec);
}
