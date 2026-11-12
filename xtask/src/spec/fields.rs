//! The individual properties of the schemas.

mod datetimes;
mod enums;
mod identifiers;
mod null_examples;
mod nullable;
mod renames;
mod required;

use serde_json::Value;

/// Marks the properties the spec documents as `null` as nullable.
pub(super) fn nullable_null_examples(spec: &mut Value) {
    null_examples::relax(spec);
}

/// Renames the response properties whose spec name differs from the API's.
pub(super) fn rename_mismatched_properties(spec: &mut Value) {
    renames::rename(spec);
}

/// Trims the `required` lists that demand fields the live API omits.
pub(super) fn relax_overstrict_required(spec: &mut Value) {
    required::relax_overstrict(spec);
}

/// Drops the properties the live API omits from the required lists.
pub(super) fn relax_omitted_required(spec: &mut Value) {
    required::relax_omitted(spec);
}

/// Drops `requests` from the apps frontend-preset required list.
pub(super) fn relax_apps_preset_required(spec: &mut Value) {
    required::relax_apps_preset(spec);
}

/// Drops the closed enums on the fields the API extends with new values.
pub(super) fn relax_open_enums(spec: &mut Value) {
    enums::relax_open(spec);
}

/// Appends the enum values the live API emits but the published spec lacks.
pub(super) fn add_observed_enum_values(spec: &mut Value) {
    enums::add_observed(spec);
}

/// Marks the VPC string fields nullable that the live API returns as `null`.
pub(super) fn nullable_vpc_optional_fields(spec: &mut Value) {
    nullable::relax_vpc(spec);
}

/// Types the identifier fields as integers instead of floats.
pub(super) fn integer_id_fields(spec: &mut Value) {
    identifiers::relax(spec);
}

/// Types the deploy timestamps as plain strings.
pub(super) fn naive_deploy_datetimes(spec: &mut Value) {
    datetimes::relax_deploy(spec);
}
