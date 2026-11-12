//! Normalization of the upstream Timeweb Cloud `OpenAPI` specification.
//!
//! The vendor publishes a specification the Rust generator cannot turn into a
//! compiling client without adjustments. This module applies them, grouped by
//! the concern they address:
//!
//! * [`paths`] — the path parameters of a route.
//! * [`tags`] — the localization of the operation tags.
//! * [`unions`] — the `oneOf`/`anyOf` schemas.
//! * [`fields`] — the individual properties of a schema.
//! * [`responses`] — the response envelopes.
//! * [`operations`] — the operations themselves.
//!
//! The passes are idempotent: running them twice yields the same document.

mod fields;
mod json;
mod operations;
mod paths;
mod responses;
mod schemas;
mod tags;
mod unions;

use std::{fs, path::Path};

use masterror::{AppError, AppResult};
use serde_json::Value;

/// Reads a specification, normalizes it and writes the result.
///
/// # Arguments
///
/// * `input` — the specification to read.
/// * `output` — the file to write the normalized specification to.
///
/// # Errors
///
/// Returns an error when the input cannot be read, is not valid JSON or
/// cannot be written back.
pub fn normalize_file(input: &Path, output: &Path) -> AppResult<()> {
    let text = fs::read_to_string(input)?;
    let mut spec: Value = serde_json::from_str(&text)?;
    normalize(&mut spec);
    let normalized = serde_json::to_string_pretty(&spec)?;
    fs::write(output, normalized)?;
    Ok(())
}

/// Applies every normalization pass to the specification.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
///
/// # Example
///
/// ```rust
/// let mut spec = serde_json::json!({ "paths": {} });
/// spec::normalize(&mut spec);
/// assert!(spec.get("paths").is_some());
/// ```
pub fn normalize(spec: &mut Value) {
    schemas::rename_colliding_schemas(spec);
    paths::fix_path_parameters(spec);
    tags::localize_tags(spec);
    unions::relax_broken_array_unions(spec);
    unions::relax_any_of(spec);
    unions::collapse_string_unions(spec);
    unions::relax_free_form_array_unions(spec);
    fields::nullable_null_examples(spec);
    responses::nullable_response_id(spec);
    responses::add_undocumented_account_status_fields(spec);
    responses::add_undocumented_project_resource_fields(spec);
    fields::rename_mismatched_properties(spec);
    fields::relax_overstrict_required(spec);
    fields::relax_omitted_required(spec);
    fields::relax_apps_preset_required(spec);
    fields::relax_open_enums(spec);
    fields::nullable_vpc_optional_fields(spec);
    fields::integer_id_fields(spec);
    operations::add_image_upload_body(spec);
    fields::naive_deploy_datetimes(spec);
    fields::add_observed_enum_values(spec);
}

/// Builds the error the tasks report when the generated code does not match
/// the specification.
///
/// # Arguments
///
/// * `message` — the description of the failure.
///
/// # Returns
///
/// The error to return from a task.
pub fn spec_error(message: impl Into<String>) -> AppError {
    AppError::internal(message.into())
}
