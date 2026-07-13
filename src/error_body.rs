//! Uniform view of the API's error response bodies.

use serde::Deserialize;

use crate::apis::Error;

/// The error envelope every Timeweb Cloud error response shares.
///
/// The generated operations type each error body per operation and status,
/// which makes uniform handling (logging, user messages) awkward. All those
/// bodies carry the same envelope, and this type extracts it from any
/// operation error without naming the per-operation entity:
///
/// ```no_run
/// # async fn run() {
/// use timeweb_rs::{ErrorDetails, apis::servers_api};
///
/// let config = timeweb_rs::authenticated("your-jwt-token");
/// if let Err(error) = servers_api::get_server(&config, 42).await {
///     if let Some(details) = ErrorDetails::from_api_error(&error) {
///         eprintln!("{}: {}", details.status_code, details.messages().join("; "));
///     }
/// }
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ErrorDetails {
    /// HTTP status code repeated in the body.
    pub status_code: u16,
    /// Machine-readable error identifier.
    pub error_code:  String,
    /// One or several human-readable messages.
    #[serde(default)]
    pub message:     Option<ErrorMessage>,
    /// Request correlation id for support tickets.
    #[serde(default)]
    pub response_id: Option<String>
}

/// An error body's `message` field: the API sends a string or a list.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum ErrorMessage {
    /// A single message.
    One(String),
    /// Several messages, typically one per invalid field.
    Many(Vec<String>)
}

impl ErrorDetails {
    /// Extracts the error envelope from any operation error.
    ///
    /// Returns `None` for transport, serialization and IO errors, and for
    /// response bodies that do not carry the envelope.
    #[must_use]
    pub fn from_api_error<E>(error: &Error<E>) -> Option<Self> {
        match error {
            Error::ResponseError(response) => serde_json::from_str(&response.content).ok(),
            Error::Reqwest(_) | Error::Serde(_) | Error::Io(_) => None
        }
    }

    /// Every message in the body, regardless of the field's shape.
    #[must_use]
    pub fn messages(&self) -> Vec<String> {
        match &self.message {
            None => Vec::new(),
            Some(ErrorMessage::One(message)) => vec![message.clone()],
            Some(ErrorMessage::Many(messages)) => messages.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrorDetails, ErrorMessage};
    use crate::apis::{Error, ResponseContent};

    fn response_error(content: &str) -> Error<()> {
        Error::ResponseError(ResponseContent {
            status:  reqwest::StatusCode::BAD_REQUEST,
            content: content.to_string(),
            entity:  None
        })
    }

    #[test]
    fn parses_a_single_message_envelope() {
        let error = response_error(
            r#"{
                "status_code": 400,
                "error_code": "bad_request",
                "message": "invalid preset",
                "response_id": "3037b284-a6ac-4dc7-b6d7-1f624dcfcec6"
            }"#
        );
        let details = ErrorDetails::from_api_error(&error).expect("envelope parses");
        assert_eq!(details.status_code, 400);
        assert_eq!(details.error_code, "bad_request");
        assert_eq!(details.messages(), vec!["invalid preset".to_string()]);
        assert_eq!(
            details.response_id.as_deref(),
            Some("3037b284-a6ac-4dc7-b6d7-1f624dcfcec6")
        );
    }

    #[test]
    fn parses_a_message_list_envelope() {
        let error = response_error(
            r#"{"status_code": 400, "error_code": "bad_request", "message": ["a", "b"]}"#
        );
        let details = ErrorDetails::from_api_error(&error).expect("envelope parses");
        assert_eq!(
            details.message,
            Some(ErrorMessage::Many(vec!["a".to_string(), "b".to_string()]))
        );
        assert_eq!(details.messages(), vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn tolerates_a_missing_message() {
        let error = response_error(r#"{"status_code": 500, "error_code": "internal"}"#);
        let details = ErrorDetails::from_api_error(&error).expect("envelope parses");
        assert!(details.message.is_none());
        assert!(details.messages().is_empty());
    }

    #[test]
    fn returns_none_for_non_envelope_bodies_and_other_errors() {
        assert!(ErrorDetails::from_api_error(&response_error("<html>oops</html>")).is_none());
        let serde_error: Error<()> =
            Error::Serde(serde_json::from_str::<u32>("x").expect_err("must fail"));
        assert!(ErrorDetails::from_api_error(&serde_error).is_none());
    }
}
