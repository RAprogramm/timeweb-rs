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
pub fn authenticated_with_base_url(
    token: impl Into<String>,
    base_url: impl Into<String>
) -> Configuration {
    Configuration {
        base_path: base_url.into(),
        ..authenticated(token)
    }
}
