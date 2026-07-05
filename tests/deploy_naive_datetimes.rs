// Regression test for naive deploy timestamps.
//
// Timeweb's live API returns `started_at`/`ended_at` on
// `GET /api/v1/apps/{app_id}/deploys` without a UTC offset (e.g.
// `2026-07-04T05:43:00`), even though the spec declares `format: date-time`
// with RFC 3339 examples. The generated `Deploy` model must type these
// fields as plain strings (see
// `openapi/normalize_spec.py::naive_deploy_datetimes`) so both the naive
// live shape and the documented RFC 3339 shape deserialize.

use timeweb_rs::models::Deploy;

#[test]
fn deploy_tolerates_naive_timestamps() {
    let body = r#"{
        "app_id": "2e1c50e8-d947-4945-9988-813fa2fd810c",
        "commit_sha": "d802ac241e84d5740fae66d5f950a8cd2c96e775",
        "id": "45755624-d25e-4472-a59c-2c0b74ba5242",
        "started_at": "2026-07-04T05:43:00",
        "ended_at": "2026-07-04T05:46:32",
        "status": "success",
        "commit_msg": "fix: something"
    }"#;

    let deploy: Deploy = serde_json::from_str(body).expect("naive timestamps must deserialize");
    assert_eq!(deploy.started_at, "2026-07-04T05:43:00");
    assert_eq!(deploy.ended_at.as_deref(), Some("2026-07-04T05:46:32"));
}

#[test]
fn deploy_tolerates_rfc3339_timestamps_and_null_ended_at() {
    let body = r#"{
        "app_id": "2e1c50e8-d947-4945-9988-813fa2fd810c",
        "commit_sha": "d802ac241e84d5740fae66d5f950a8cd2c96e775",
        "id": "45755624-d25e-4472-a59c-2c0b74ba5242",
        "started_at": "2024-07-24T10:38:44.000Z",
        "ended_at": null,
        "status": "stopped",
        "commit_msg": "feat: something else"
    }"#;

    let deploy: Deploy = serde_json::from_str(body).expect("documented shape must deserialize");
    assert_eq!(deploy.started_at, "2024-07-24T10:38:44.000Z");
    assert_eq!(deploy.ended_at, None);
}
