//! The response envelopes of the specification.

mod account_status;
mod project_resources;
mod response_id;

use serde_json::Value;

/// Marks `response_id` nullable so a `null` from the API deserializes.
pub(super) fn nullable_response_id(spec: &mut Value) {
    response_id::relax(spec);
}

/// Adds the `status` fields the live API returns but the spec omits.
pub(super) fn add_undocumented_account_status_fields(spec: &mut Value) {
    account_status::add_fields(spec);
}

/// Exposes the project resources and the app-to-project linkage the API
/// returns.
pub(super) fn add_undocumented_project_resource_fields(spec: &mut Value) {
    project_resources::add_fields(spec);
}
