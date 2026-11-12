//! The nullable fields of the schemas.

use serde_json::Value;

use crate::spec::json::{as_object_mut, schema_mut};

/// Marks the VPC string fields nullable that the live API returns as `null`.
///
/// The `vpc` schema types `description` and `public_ip` as required strings,
/// but `GET /api/v2/vpcs` returns `null` for them, failing with "invalid type:
/// null, expected a string".
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn relax_vpc(spec: &mut Value) {
    let Some(vpc) = schema_mut(spec, "vpc") else {
        return;
    };
    let Some(properties) = vpc.get_mut("properties").and_then(as_object_mut) else {
        return;
    };
    for name in ["description", "public_ip"] {
        let Some(field) = properties.get_mut(name).and_then(as_object_mut) else {
            continue;
        };
        if field.contains_key("$ref") {
            continue;
        }
        field.insert("nullable".to_string(), Value::Bool(true));
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::relax_vpc;

    #[test]
    fn the_vpc_fields_become_nullable() {
        let mut spec = json!({
            "components": {
                "schemas": {
                    "vpc": {
                        "properties": {
                            "description": { "type": "string" },
                            "public_ip": { "type": "string" }
                        }
                    }
                }
            }
        });
        relax_vpc(&mut spec);
        let properties = &spec["components"]["schemas"]["vpc"]["properties"];
        assert_eq!(properties["description"]["nullable"], json!(true));
        assert_eq!(properties["public_ip"]["nullable"], json!(true));
    }
}
