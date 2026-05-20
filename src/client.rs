//! Idiomatic entry point for building an authenticated API configuration.

use crate::apis::configuration::Configuration;

/// Default base URL of the Timeweb Cloud API.
pub const DEFAULT_BASE_URL: &str = "https://api.timeweb.cloud";

/// Builds a [`Configuration`] authenticated with a Timeweb Cloud JWT token.
///
/// The token is issued in the Timeweb Cloud control panel under the
/// "API и Terraform" section. It is sent as a `Bearer` `Authorization` header
/// on every request made with the returned configuration.
///
/// Pass the configuration to any function in [`crate::apis`]:
///
/// ```no_run
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// use timeweb_rs::apis::account_api;
///
/// let config = timeweb_rs::authenticated("your-jwt-token");
/// let status = account_api::get_account_status(&config).await?;
/// println!("{status:#?}");
/// # Ok(())
/// # }
/// ```
#[must_use]
pub fn authenticated(token: impl Into<String>) -> Configuration {
    Configuration {
        bearer_access_token: Some(token.into()),
        user_agent: Some(concat!("timeweb-rs/", env!("CARGO_PKG_VERSION")).to_string()),
        ..Configuration::default()
    }
}

/// Builds an authenticated [`Configuration`] targeting a custom base URL.
///
/// Useful for testing against a mock server or a proxy. For production use
/// prefer [`authenticated`], which targets [`DEFAULT_BASE_URL`].
#[must_use]
pub fn authenticated_with_base_url(
    token: impl Into<String>,
    base_url: impl Into<String>
) -> Configuration {
    Configuration {
        base_path: base_url.into(),
        ..authenticated(token)
    }
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_BASE_URL, authenticated, authenticated_with_base_url};

    #[test]
    fn authenticated_sets_bearer_token_and_default_base_url() {
        let config = authenticated("jwt-token");
        assert_eq!(config.bearer_access_token.as_deref(), Some("jwt-token"));
        assert_eq!(config.base_path, DEFAULT_BASE_URL);
    }

    #[test]
    fn authenticated_sets_crate_user_agent() {
        let config = authenticated("jwt-token");
        let user_agent = config.user_agent.expect("user agent is set");
        assert!(user_agent.starts_with("timeweb-rs/"));
    }

    #[test]
    fn authenticated_with_base_url_overrides_base_path() {
        let config = authenticated_with_base_url("jwt-token", "http://localhost:8080");
        assert_eq!(config.base_path, "http://localhost:8080");
        assert_eq!(config.bearer_access_token.as_deref(), Some("jwt-token"));
    }
}
