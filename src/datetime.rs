//! Timestamps the API sends with and without a UTC offset.
//!
//! The Timeweb Cloud API is not consistent about the timestamps it returns:
//! most endpoints send RFC 3339 values with an offset (`2024-07-24T10:38:44.000Z`),
//! while the Kubernetes area sends naive local-looking values without one
//! (`2023-01-13T10:10:29.742918`). The strict
//! [`chrono::DateTime<chrono::FixedOffset>`] deserializer rejects the second
//! shape with "premature end of input", which fails the deserialization of
//! the whole response.
//!
//! The deserializers here accept both shapes: a value with an offset keeps
//! it, and a value without one is read as UTC — the timezone the API uses for
//! every timestamp it documents.

use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use serde::{de::Error as _, Deserialize, Deserializer};

/// The layouts a timestamp without a UTC offset can use.
const NAIVE_FORMATS: [&str; 2] = ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%d %H:%M:%S%.f"];

/// Parses a timestamp with or without a UTC offset.
///
/// # Arguments
///
/// * `raw` — the timestamp to parse.
///
/// # Returns
///
/// The parsed timestamp, or a message describing why it cannot be read.
fn parse_datetime(raw: &str) -> Result<DateTime<FixedOffset>, String> {
    if let Ok(value) = DateTime::parse_from_rfc3339(raw) {
        return Ok(value);
    }
    for format in NAIVE_FORMATS {
        if let Ok(naive) = NaiveDateTime::parse_from_str(raw, format) {
            return Ok(Utc.from_utc_datetime(&naive).fixed_offset());
        }
    }
    Err(format!("`{raw}` is not an RFC 3339 timestamp"))
}

/// Reads a timestamp that carries a UTC offset or does not.
///
/// # Arguments
///
/// * `deserializer` — the source of the timestamp.
///
/// # Returns
///
/// The parsed timestamp, or the error of the underlying deserializer.
pub(crate) fn deserialize_datetime<'de, D>(
    deserializer: D,
) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    parse_datetime(&raw).map_err(D::Error::custom)
}

/// Reads an optional timestamp that carries a UTC offset or does not.
///
/// # Arguments
///
/// * `deserializer` — the source of the timestamp.
///
/// # Returns
///
/// The parsed timestamp, `None` for a missing or null value, or the error of
/// the underlying deserializer.
pub(crate) fn deserialize_optional_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<String>::deserialize(deserializer)?;
    raw.map(|raw| parse_datetime(&raw))
        .transpose()
        .map_err(D::Error::custom)
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, FixedOffset, TimeZone};
    use serde::Deserialize;

    use super::{deserialize_datetime, deserialize_optional_datetime};

    #[derive(Debug, Deserialize)]
    struct Required {
        #[serde(deserialize_with = "deserialize_datetime")]
        value: DateTime<FixedOffset>,
    }

    #[derive(Debug, Deserialize)]
    struct Optional {
        #[serde(default, deserialize_with = "deserialize_optional_datetime")]
        value: Option<DateTime<FixedOffset>>,
    }

    #[test]
    fn reads_a_timestamp_with_an_offset() {
        let parsed: Required = serde_json::from_str(r#"{"value":"2024-07-24T10:38:44.000Z"}"#)
            .expect("the timestamp deserializes");
        assert_eq!(parsed.value.to_rfc3339(), "2024-07-24T10:38:44+00:00");
    }

    #[test]
    fn keeps_a_numeric_offset() {
        let parsed: Required =
            serde_json::from_str(r#"{"value":"2024-07-24T13:38:44.000+03:00"}"#)
                .expect("the timestamp deserializes");
        assert_eq!(parsed.value.to_rfc3339(), "2024-07-24T13:38:44+03:00");
    }

    #[test]
    fn reads_a_timestamp_without_an_offset_as_utc() {
        let parsed: Required = serde_json::from_str(r#"{"value":"2023-01-13T10:10:29.742918"}"#)
            .expect("the timestamp deserializes");
        assert_eq!(
            parsed.value.to_rfc3339(),
            "2023-01-13T10:10:29.742918+00:00"
        );
    }

    #[test]
    fn reads_a_timestamp_with_a_space_separator() {
        let parsed: Required = serde_json::from_str(r#"{"value":"2023-01-13 10:10:29"}"#)
            .expect("the timestamp deserializes");
        assert_eq!(parsed.value.timestamp(), 1_673_478_629);
    }

    #[test]
    fn rejects_a_value_that_is_not_a_timestamp() {
        let error = serde_json::from_str::<Required>(r#"{"value":"yesterday"}"#)
            .expect_err("the timestamp is rejected");
        assert!(error.to_string().contains("RFC 3339"));
    }

    #[test]
    fn reads_a_missing_optional_timestamp() {
        let parsed: Optional = serde_json::from_str("{}").expect("the field is optional");
        assert!(parsed.value.is_none());
    }

    #[test]
    fn reads_a_null_optional_timestamp() {
        let parsed: Optional =
            serde_json::from_str(r#"{"value":null}"#).expect("the field is optional");
        assert!(parsed.value.is_none());
    }

    #[test]
    fn reads_an_optional_timestamp_without_an_offset() {
        let parsed: Optional = serde_json::from_str(r#"{"value":"2026-07-04T05:43:00"}"#)
            .expect("the timestamp deserializes");
        let expected = chrono::Utc
            .with_ymd_and_hms(2026, 7, 4, 5, 43, 0)
            .single()
            .expect("the timestamp is valid");
        assert_eq!(parsed.value, Some(expected.fixed_offset()));
    }
}
