//! The `oneOf` and `anyOf` schemas of the specification.
//!
//! A union is the part of the specification the Rust generator handles worst:
//! it turns every union into an untagged enum and names the variants after
//! the member types, which produces invalid Rust for some member shapes. The
//! passes of this module rewrite the unions the generator cannot render.

mod any_of;
mod broken_arrays;
mod free_form_arrays;
mod string_unions;

use serde_json::Value;

/// Rewrites the unions the Rust generator renders as invalid Rust.
pub(super) fn relax_broken_array_unions(spec: &mut Value) {
    broken_arrays::relax(spec);
}

/// Rewrites the `anyOf` unions the generator renders as empty structs.
pub(super) fn relax_any_of(spec: &mut Value) {
    any_of::relax(spec);
}

/// Collapses the unions whose members are all strings.
pub(super) fn collapse_string_unions(spec: &mut Value) {
    string_unions::collapse(spec);
}

/// Types the free-form list members of a union as lists of strings.
pub(super) fn relax_free_form_array_unions(spec: &mut Value) {
    free_form_arrays::relax(spec);
}
