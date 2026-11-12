//! The binary request body of the image-upload operation.

use serde_json::{Map, Value};

use crate::spec::json::as_object_mut;

/// The route that uploads an image.
const UPLOAD_ROUTE: &str = "/api/v1/images/{image_id}";

/// Adds the missing binary request body to the image-upload operation.
///
/// The upstream spec declares `POST /api/v1/images/{image_id}` (operationId
/// `uploadImage`) with only a `Content-Disposition` header parameter and no
/// `requestBody` — so the generated client cannot send the image file at all.
/// The endpoint accepts the raw file as the request body; inject an
/// `application/octet-stream` binary body so the generated function gains a
/// body parameter.
///
/// # Arguments
///
/// * `spec` — the specification to normalize in place.
pub(super) fn add_body(spec: &mut Value) {
    let Some(paths) = spec.get_mut("paths").and_then(Value::as_object_mut) else {
        return;
    };
    let Some(operation) = paths
        .get_mut(UPLOAD_ROUTE)
        .and_then(as_object_mut)
        .and_then(|path| path.get_mut("post"))
        .and_then(as_object_mut)
    else {
        return;
    };
    if operation.contains_key("requestBody") {
        return;
    }
    let mut schema = Map::new();
    schema.insert("type".to_string(), Value::String("string".to_string()));
    schema.insert("format".to_string(), Value::String("binary".to_string()));
    let mut media = Map::new();
    media.insert("schema".to_string(), Value::Object(schema));
    let mut content = Map::new();
    content.insert("application/octet-stream".to_string(), Value::Object(media));
    let mut body = Map::new();
    body.insert("required".to_string(), Value::Bool(true));
    body.insert("content".to_string(), Value::Object(content));
    operation.insert("requestBody".to_string(), Value::Object(body));
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::add_body;

    #[test]
    fn the_binary_body_is_added() {
        let mut spec = json!({
            "paths": {
                "/api/v1/images/{image_id}": { "post": { "operationId": "uploadImage" } }
            }
        });
        add_body(&mut spec);
        let body = &spec["paths"]["/api/v1/images/{image_id}"]["post"]["requestBody"];
        assert_eq!(body["required"], json!(true));
        assert_eq!(
            body["content"]["application/octet-stream"]["schema"]["format"],
            json!("binary")
        );
    }

    #[test]
    fn an_existing_body_is_kept() {
        let mut spec = json!({
            "paths": {
                "/api/v1/images/{image_id}": {
                    "post": { "operationId": "uploadImage", "requestBody": { "required": false } }
                }
            }
        });
        add_body(&mut spec);
        assert_eq!(
            spec["paths"]["/api/v1/images/{image_id}"]["post"]["requestBody"]["required"],
            json!(false)
        );
    }
}
