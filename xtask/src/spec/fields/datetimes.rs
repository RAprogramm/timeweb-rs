//! The timestamps the specification types as RFC 3339.

use serde_json::Value;

use crate::spec::json::{as_object_mut, schema_mut};

/// Types the deploy timestamps as plain strings.
///
/// The spec declares `deploy.started_at`/`ended_at` as `format: date-time`
/// with RFC 3339 examples, but the live API returns naive timestamps without
/// a UTC offset (e.g. `2026-07-04T05:43:00`). The generated
/// `chrono::DateTime<FixedOffset>` fields then fail deserialization with
/// "premature end of input". Dropping the format yields `String` fields that
/// survive both shapes; ISO-ordered strings still sort chronologically.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn relax_deploy(spec: &mut Value) {
    let Some(deploy) = schema_mut(spec, "deploy") else {
        return;
    };
    let Some(properties) = deploy.get_mut("properties").and_then(as_object_mut) else {
        return;
    };
    for name in ["started_at", "ended_at"] {
        if let Some(property) = properties.get_mut(name).and_then(as_object_mut) {
            property.remove("format");
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::relax_deploy;

    #[test]
    fn deploy_timestamps_lose_their_format() {
        let mut spec = json!({
            "components": {
                "schemas": {
                    "deploy": {
                        "properties": {
                            "started_at": { "type": "string", "format": "date-time" },
                            "ended_at": { "type": "string", "format": "date-time" }
                        }
                    }
                }
            }
        });
        relax_deploy(&mut spec);
        let properties = &spec["components"]["schemas"]["deploy"]["properties"];
        assert!(properties["started_at"].get("format").is_none());
        assert!(properties["ended_at"].get("format").is_none());
    }
}
