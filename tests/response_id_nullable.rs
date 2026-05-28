// Regression test for the `response_id: null` deserialization bug.
//
// Timeweb's live API returns `response_id: null` on some responses (e.g.
// `GET /api/v1/account/finances`), even though the spec types it as a
// required `uuid`. The generated models must tolerate the null — they
// type `response_id` as `Option<uuid::Uuid>` (see
// `openapi/normalize_spec.py::nullable_response_id`). This test pins that
// behaviour so a future regeneration can't silently reintroduce a
// non-optional `response_id`.

use timeweb_rs::models::GetFinances200Response;

#[test]
fn finances_response_tolerates_null_response_id() {
    let body = r#"{
        "finances": {
            "balance": 1234.5,
            "currency": "RUB",
            "discount_end_date_at": null,
            "discount_percent": 0.0,
            "hourly_cost": 0.0,
            "hourly_fee": 0.0,
            "monthly_cost": 100.0,
            "monthly_fee": 0.0,
            "total_paid": 0.0,
            "hours_left": null,
            "autopay_card_info": null
        },
        "response_id": null
    }"#;

    let parsed: GetFinances200Response =
        serde_json::from_str(body).expect("null response_id must deserialize to None, not error");

    assert!(parsed.response_id.is_none());
    assert!((parsed.finances.balance - 1234.5).abs() < f64::EPSILON);
}

#[test]
fn finances_response_still_accepts_a_uuid_response_id() {
    let body = r#"{
        "finances": {
            "balance": 0.0,
            "currency": "RUB",
            "discount_end_date_at": null,
            "discount_percent": 0.0,
            "hourly_cost": 0.0,
            "hourly_fee": 0.0,
            "monthly_cost": 0.0,
            "monthly_fee": 0.0,
            "total_paid": 0.0,
            "hours_left": null,
            "autopay_card_info": null
        },
        "response_id": "15095f25-aac3-4d60-a788-96cb5136f186"
    }"#;

    let parsed: GetFinances200Response =
        serde_json::from_str(body).expect("a present uuid response_id must still deserialize");

    assert_eq!(
        parsed.response_id.map(|id| id.to_string()).as_deref(),
        Some("15095f25-aac3-4d60-a788-96cb5136f186")
    );
}
