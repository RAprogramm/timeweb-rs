//! Post-processing of the generated models.
//!
//! `openapi-generator` produces code that follows the specification to the
//! letter. The live API is more forgiving than the specification, and the
//! difference breaks deserialization of real responses. This module repairs
//! the generated code so the responses the API documents actually parse:
//!
//! 1. **Optional fields.** A nullable property the generator emits as
//!    `Option<T>` with `deserialize_with = "Option::deserialize"` is still
//!    *mandatory* for serde: a response that omits the field fails with
//!    "missing field `x`". Adding `default` makes an absent field deserialize
//!    to `None` while a present `null` keeps mapping to `None`.
//! 2. **Timestamps.** The API sends some timestamps without a UTC offset (e.g.
//!    `2026-07-04T05:43:00`), which the strict `chrono::DateTime<FixedOffset>`
//!    deserializer rejects with "premature end of input". The fields use the
//!    tolerant deserializers of the crate instead, which accept both the offset
//!    and the naive shape.
//! 3. **Panics.** The generated `parse_deep_object` aborts the process with
//!    `unimplemented!()` when it is handed a value that is not an object; the
//!    patched version returns an empty list instead.
//!
//! The task is idempotent: a model that already carries the patched
//! attributes is left untouched.

use std::fs;

use masterror::AppResult;
use regex::Regex;

use crate::paths;

/// The deserializer that reads a timestamp with or without a UTC offset.
const DATETIME_DESERIALIZER: &str = "crate::datetime::deserialize_datetime";

/// The deserializer that reads an optional timestamp.
const OPTIONAL_DATETIME_DESERIALIZER: &str = "crate::datetime::deserialize_optional_datetime";

/// The attribute the generator emits for a nullable property.
const NULLABLE_DESERIALIZER: &str = "Option::deserialize";

/// The panic the generator emits for a value that is not an object.
const UNIMPLEMENTED_PANIC: &str =
    "unimplemented!(\"Only objects are supported with style=deepObject\")";

/// The replacement of the generated panic.
const EMPTY_LIST: &str = "Vec::new()";

/// The declaration of the generated error envelope.
const RESPONSE_CONTENT: &str = "pub struct ResponseContent<T> {";

/// The response headers the error envelope carries.
const RESPONSE_CONTENT_HEADERS: &str = "    pub headers: reqwest::header::HeaderMap,";

/// Splits a `serde` attribute into its items.
fn split_attributes(attributes: &str) -> Vec<String> {
    attributes
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

/// The value of an attribute item.
fn attribute_value(item: &str, key: &str) -> Option<String> {
    let prefix = format!("{key} = ");
    item.strip_prefix(&prefix)
        .map(|value| value.trim_matches('"').to_string())
}

/// Reorders the attribute items and applies the requested changes.
///
/// # Arguments
///
/// * `items` — the attribute items of a field.
/// * `datetime` — the deserializer the field needs, if any.
/// * `default` — whether the field needs the `default` attribute.
///
/// # Returns
///
/// The reordered items, or `None` when nothing has to change.
fn rewrite_attributes(
    items: &[String],
    datetime: Option<&str>,
    default: bool
) -> Option<Vec<String>> {
    let mut rename = None;
    let mut current_default = None;
    let mut deserialize_with = None;
    let mut with = None;
    let mut skip_serializing = None;
    let mut rest = Vec::new();
    for item in items {
        if let Some(value) = item.strip_prefix("rename = ") {
            rename = Some(format!("rename = {value}"));
        } else if item.starts_with("default") {
            current_default = Some("default".to_string());
        } else if let Some(value) = item.strip_prefix("deserialize_with = ") {
            deserialize_with = Some(value.trim_matches('"').to_string());
        } else if let Some(value) = item.strip_prefix("with = ") {
            with = Some(format!("with = {value}"));
        } else if let Some(value) = item.strip_prefix("skip_serializing_if = ") {
            skip_serializing = Some(format!("skip_serializing_if = {value}"));
        } else {
            rest.push(item.clone());
        }
    }

    let needs_default = default && current_default.is_none();
    let needs_datetime =
        datetime.is_some_and(|deserializer| deserialize_with.as_deref() != Some(deserializer));
    if !needs_default && !needs_datetime {
        return None;
    }

    let mut rewritten = Vec::new();
    if let Some(rename) = rename {
        rewritten.push(rename);
    }
    if needs_default {
        rewritten.push("default".to_string());
    }
    if let Some(deserializer) = datetime {
        rewritten.push(format!("deserialize_with = \"{deserializer}\""));
    } else if let Some(deserialize_with) = deserialize_with {
        rewritten.push(format!("deserialize_with = \"{deserialize_with}\""));
    }
    if let Some(with) = with {
        rewritten.push(with);
    }
    if let Some(skip_serializing) = skip_serializing {
        rewritten.push(skip_serializing);
    }
    rewritten.extend(rest);
    Some(rewritten)
}

/// Patches the models of a single file.
///
/// # Arguments
///
/// * `text` — the source of the model file.
///
/// # Returns
///
/// The patched source and whether anything changed.
fn patch_model_source(text: &str) -> (String, bool) {
    let field = Regex::new(
        r"(?s)#\[serde\((?P<attrs>[^)]*)\)\]\s*pub (?P<name>\w+):\s*(?P<type>[^,\n]+),"
    )
    .expect("the field pattern is valid");
    let mut changed = false;
    let patched = field
        .replace_all(text, |captures: &regex::Captures<'_>| {
            let attributes = &captures["attrs"];
            let field_type = &captures["type"];
            let items = split_attributes(attributes);
            let nullable = items.iter().any(|item| {
                attribute_value(item, "deserialize_with").as_deref() == Some(NULLABLE_DESERIALIZER)
            });

            let datetime = if field_type.contains("chrono::DateTime") {
                if field_type.trim_start().starts_with("Option<") {
                    Some(OPTIONAL_DATETIME_DESERIALIZER)
                } else {
                    Some(DATETIME_DESERIALIZER)
                }
            } else {
                None
            };
            let default = nullable || datetime.is_some();

            let Some(rewritten) = rewrite_attributes(&items, datetime, default) else {
                return captures
                    .get(0)
                    .map_or_else(String::new, |found| found.as_str().to_string());
            };
            changed = true;
            format!(
                "#[serde({})]\n    pub {}: {},",
                rewritten.join(", "),
                &captures["name"],
                field_type
            )
        })
        .to_string();
    (patched, changed)
}

/// Patches the shared helpers of the generated API module.
///
/// # Arguments
///
/// * `text` — the source of `src/apis/mod.rs`.
///
/// # Returns
///
/// The patched source and whether anything changed.
fn patch_apis_source(text: &str) -> (String, bool) {
    let mut patched = text.replace(UNIMPLEMENTED_PANIC, EMPTY_LIST);
    let panic_replaced = patched != text;

    let changed = if patched.contains(RESPONSE_CONTENT) && !patched.contains("pub headers:") {
        let declaration = format!("{RESPONSE_CONTENT}\n{RESPONSE_CONTENT_HEADERS}");
        patched = patched.replace(RESPONSE_CONTENT, &declaration);
        true
    } else {
        panic_replaced
    };
    (patched, changed)
}

/// Patches the error envelope of a generated API module.
///
/// The generated operations drop the response headers when they report an
/// error, so the client cannot honor the `Retry-After` header the API sends
/// with its rate-limit responses. The patch keeps the headers next to the
/// body.
///
/// # Arguments
///
/// * `text` — the source of an API module.
///
/// # Returns
///
/// The patched source and whether anything changed.
fn patch_error_envelope(text: &str) -> (String, bool) {
    let content = Regex::new(r"(?s)\} else \{\n(\s*)let content = resp\.text\(\)\.await\?;")
        .expect("the response pattern is valid");
    let mut patched = text.to_string();
    if !patched.contains("let headers = resp.headers().clone();") {
        patched = content
            .replace_all(
                &patched,
                "} else {\n${1}let headers = resp.headers().clone();\n${1}let content = resp.text().await?;",
            )
            .to_string();
    }
    let envelope = Regex::new(r"(?s)(ResponseContent \{\n\s*status,\n\s*content,\n)(\s*entity)")
        .expect("the envelope pattern is valid");
    if !patched.contains("headers,") {
        patched = envelope
            .replace_all(&patched, "${1}            headers,\n${2}")
            .to_string();
    }
    let changed = patched != text;
    (patched, changed)
}

/// Patches every generated model and the shared API module.
///
/// # Errors
///
/// Returns an error when the generated sources cannot be read or written.
pub fn patch_all() -> AppResult<()> {
    let mut patched_models = 0;
    let mut entries = fs::read_dir(paths::models_dir())?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(std::fs::DirEntry::path);
    for entry in entries {
        let path = entry.path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        let (patched, changed) = patch_model_source(&text);
        if changed {
            fs::write(&path, patched)?;
            patched_models += 1;
        }
    }

    let apis = paths::apis_module();
    let text = fs::read_to_string(&apis)?;
    let (patched, changed) = patch_apis_source(&text);
    if changed {
        fs::write(&apis, patched)?;
    }

    let mut patched_apis = 0;
    let mut entries = fs::read_dir(paths::apis_dir())?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(std::fs::DirEntry::path);
    for entry in entries {
        let path = entry.path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        let (patched, changed) = patch_error_envelope(&text);
        if changed {
            fs::write(&path, patched)?;
            patched_apis += 1;
        }
    }

    println!("patched {patched_models} model files and {patched_apis} API modules");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{patch_apis_source, patch_error_envelope, patch_model_source, split_attributes};

    #[test]
    fn nullable_fields_gain_a_default() {
        let source = r#"
    #[serde(rename = "response_id", deserialize_with = "Option::deserialize")]
    pub response_id: Option<uuid::Uuid>,
"#;
        let (patched, changed) = patch_model_source(source);
        assert!(changed);
        assert!(patched.contains("default"));
        assert!(patched.contains("deserialize_with = \"Option::deserialize\""));
    }

    #[test]
    fn required_timestamps_use_the_tolerant_deserializer() {
        let source = r#"
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
"#;
        let (patched, changed) = patch_model_source(source);
        assert!(changed);
        assert!(patched.contains("crate::datetime::deserialize_datetime"));
    }

    #[test]
    fn optional_timestamps_use_the_tolerant_optional_deserializer() {
        let source = r#"
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<chrono::DateTime<chrono::FixedOffset>>,
"#;
        let (patched, changed) = patch_model_source(source);
        assert!(changed);
        assert!(patched.contains("crate::datetime::deserialize_optional_datetime"));
        assert!(patched.contains("default"));
    }

    #[test]
    fn patching_is_idempotent() {
        let source = r#"
    #[serde(rename = "response_id", deserialize_with = "Option::deserialize")]
    pub response_id: Option<uuid::Uuid>,
"#;
        let (once, changed) = patch_model_source(source);
        assert!(changed);
        let (twice, changed) = patch_model_source(&once);
        assert!(!changed);
        assert_eq!(once, twice);
    }

    #[test]
    fn the_generated_panic_becomes_an_empty_list() {
        let source = "    unimplemented!(\"Only objects are supported with style=deepObject\")\n";
        let (patched, changed) = patch_apis_source(source);
        assert!(changed);
        assert_eq!(patched, "    Vec::new()\n");
    }

    #[test]
    fn the_error_envelope_keeps_the_response_headers() {
        let source = r"
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<X> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
";
        let (patched, changed) = patch_error_envelope(source);
        assert!(changed);
        assert!(patched.contains("let headers = resp.headers().clone();"));
        assert!(patched.contains("headers,"));
        assert_eq!(patched.matches("resp.headers().clone()").count(), 1);
    }

    #[test]
    fn patching_the_error_envelope_is_idempotent() {
        let source = r"
    } else {
        let content = resp.text().await?;
        let entity: Option<X> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity
        }))
    }
";
        let (once, changed) = patch_error_envelope(source);
        assert!(changed);
        let (twice, changed) = patch_error_envelope(&once);
        assert!(!changed);
        assert_eq!(once, twice);
    }

    #[test]
    fn attributes_split_on_the_commas() {
        let items = split_attributes("rename = \"x\", deserialize_with = \"Option::deserialize\"");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], "rename = \"x\"");
    }
}
