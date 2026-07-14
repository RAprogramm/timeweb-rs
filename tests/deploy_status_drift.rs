// Regression test for deploy statuses the live API emits ahead of its
// published spec.
//
// In July 2026 `GET /api/v1/apps/{app_id}/deploys` started returning
// `"status": "building"`, which the published bundle
// (https://timeweb.cloud/api-docs-data/bundle.json) still does not list —
// deserialization failed with `unknown variant 'building'` and the whole
// deploys listing came down with it. The value is injected at generation
// time (see `openapi/normalize_spec.py::add_observed_enum_values`), and this
// test pins the generated variant so a spec re-sync cannot silently drop it.

#![cfg(feature = "apps")]

use timeweb_rs::models::DeployStatus;

#[test]
fn building_status_deserializes() {
    let status: DeployStatus =
        serde_json::from_str(r#""building""#).expect("live-API status `building` must parse");
    assert_eq!(status, DeployStatus::Building);
}

#[test]
fn building_status_round_trips() {
    assert_eq!(DeployStatus::Building.to_string(), "building");
    assert_eq!(
        serde_json::to_string(&DeployStatus::Building).expect("serialize"),
        r#""building""#
    );
}

#[test]
fn documented_statuses_still_parse() {
    for (raw, expected) in [
        ("created", DeployStatus::Created),
        ("building_code", DeployStatus::BuildingCode),
        ("success", DeployStatus::Success),
        ("failure", DeployStatus::Failure)
    ] {
        let status: DeployStatus =
            serde_json::from_str(&format!(r#""{raw}""#)).expect("documented status must parse");
        assert_eq!(status, expected);
    }
}
