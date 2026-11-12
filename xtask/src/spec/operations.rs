//! The operations of the specification.

mod image_upload;

use serde_json::Value;

/// Adds the missing binary request body to the image-upload operation.
pub(super) fn add_image_upload_body(spec: &mut Value) {
    image_upload::add_body(spec);
}
