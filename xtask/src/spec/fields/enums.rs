//! The enums the API extends beyond the published specification.

use serde_json::Value;

use crate::spec::json::{as_str, schema_mut, walk_objects};

/// The enum values the live API emits but the published spec lacks.
const OBSERVED_ENUM_VALUES: [(&str, &[&str]); 1] = [("deploy-status", &["building"])];

/// The enum values the live API emits but the published spec lacks, by schema.
const OBSERVED_PROPERTY_ENUMS: [(&str, &str, &[&str]); 2] = [
    ("ClusterOut", "network_driver", &["canal"]),
    ("ClusterIn", "network_driver", &["canal"])
];

/// The resource kinds the floating-ip type enumerates.
const RESOURCE_KINDS: [&str; 4] = ["server", "balancer", "database", "network"];

/// Drops the closed enums on the fields the API extends with new values.
///
/// Several string enums in the spec are effectively open-ended and lag the
/// live API, so a value the spec has not caught up to fails deserialization
/// and the whole collection comes back empty:
///
/// * availability zones (`ru-1`/`pl-1`/`nl-1`/`de-1`/...);
/// * database engines (`mysql`/`postgres17`/`postgres18`/...);
/// * floating-ip `resource_type` (`server`/`balancer`/`database`/`network` —
///   the API also returns `dbaas`);
/// * network ip `type` (`ipv4`/`ipv6` — the API returns the malformed `ipv_4`
///   on database-cluster/balancer/vds/server-ip networks).
///
/// The enum is stripped (keeping `type: string`) wherever one of these
/// appears, so any present or future value deserializes as a plain string.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn relax_open(spec: &mut Value) {
    walk_objects(spec, &mut |map| {
        let is_open = map
            .get("enum")
            .and_then(Value::as_array)
            .is_some_and(|values| {
                let mut members: Vec<&str> = values.iter().filter_map(as_str).collect();
                members.sort_unstable();
                let mut kinds: Vec<&str> = RESOURCE_KINDS.to_vec();
                kinds.sort_unstable();
                members.contains(&"ru-1")
                    || members.contains(&"postgres14")
                    || members.contains(&"ipv4")
                    || members == kinds
            });
        if !is_open {
            return;
        }
        map.remove("enum");
        map.entry("type".to_string())
            .or_insert_with(|| Value::String("string".to_string()));
    });
}

/// Appends the enum values the live API emits but the published spec lacks.
///
/// Unlike the open-ended enums stripped by [`relax_open`], status enums stay
/// typed on purpose — consumers match on them. But Timeweb's runtime drifts
/// ahead of its published spec: `deploy-status` gained `building` in July 2026
/// while the bundle still lists only `building_code`, so every deploys listing
/// failed with "unknown variant `building`". Each entry in
/// [`OBSERVED_ENUM_VALUES`] is a value seen in real responses, keyed by the
/// `components/schemas` name; appending here keeps the generated variant
/// across spec re-syncs until the published spec catches up.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn add_observed(spec: &mut Value) {
    for (name, extra) in OBSERVED_ENUM_VALUES {
        let Some(schema) = schema_mut(spec, name) else {
            continue;
        };
        let Some(values) = schema.get_mut("enum").and_then(Value::as_array_mut) else {
            continue;
        };
        for value in extra {
            if !values
                .iter()
                .any(|existing| as_str(existing) == Some(*value))
            {
                values.push(Value::String((*value).to_string()));
            }
        }
    }
    for (name, property, extra) in OBSERVED_PROPERTY_ENUMS {
        let Some(schema) = schema_mut(spec, name) else {
            continue;
        };
        let Some(values) = schema
            .get_mut("properties")
            .and_then(Value::as_object_mut)
            .and_then(|properties| properties.get_mut(property))
            .and_then(Value::as_object_mut)
            .and_then(|property| property.get_mut("enum"))
            .and_then(Value::as_array_mut)
        else {
            continue;
        };
        for value in extra {
            if !values
                .iter()
                .any(|existing| as_str(existing) == Some(*value))
            {
                values.push(Value::String((*value).to_string()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{add_observed, relax_open};

    #[test]
    fn open_enums_are_stripped() {
        let mut spec = json!({
            "properties": {
                "location": { "type": "string", "enum": ["ru-1", "pl-1"] },
                "status": { "type": "string", "enum": ["active", "stopped"] }
            }
        });
        relax_open(&mut spec);
        assert!(spec["properties"]["location"].get("enum").is_none());
        assert_eq!(spec["properties"]["location"]["type"], json!("string"));
        assert!(spec["properties"]["status"].get("enum").is_some());
    }

    #[test]
    fn observed_values_are_appended() {
        let mut spec = json!({
            "components": {
                "schemas": {
                    "deploy-status": { "type": "string", "enum": ["created", "success"] }
                }
            }
        });
        add_observed(&mut spec);
        assert_eq!(
            spec["components"]["schemas"]["deploy-status"]["enum"],
            json!(["created", "success", "building"])
        );
    }
}
