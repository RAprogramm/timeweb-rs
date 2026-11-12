//! Idiomatic entry point for building an authenticated API configuration.

use std::time::Duration;

use crate::apis::configuration::Configuration;

/// Default base URL of the Timeweb Cloud API.
pub const DEFAULT_BASE_URL: &str = "https://api.timeweb.cloud";

/// Default base URL of the Timeweb AI Gateway API.
///
/// The AI Gateway is a separate API surface with its own host: the
/// operations of the `ai_gateway_inference_api` module talk to it instead of
/// to [`DEFAULT_BASE_URL`].
pub const AI_GATEWAY_BASE_URL: &str = "https://api.timeweb.ai";

/// Default timeout of a request.
///
/// The HTTP client of a configuration built with [`authenticated`] gives up
/// after this timeout, so a stalled connection cannot block a task forever.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Builds the HTTP client of a configuration.
///
/// # Arguments
///
/// * `timeout` — the timeout of a request.
///
/// # Returns
///
/// The client, or the default one when the timeout cannot be applied.
fn http_client(timeout: Duration) -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(timeout)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new())
}

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
        client: http_client(DEFAULT_TIMEOUT),
        ..Configuration::default()
    }
}

/// Builds a [`Configuration`] authenticated with a token and a custom client.
///
/// Use it when the caller needs its own HTTP client — a proxy, a connection
/// pool, a different timeout or a custom TLS setup:
///
/// ```no_run
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = reqwest::Client::builder()
///     .timeout(std::time::Duration::from_secs(5))
///     .build()?;
/// let config = timeweb_rs::authenticated_with_client("your-jwt-token", client);
/// # Ok(())
/// # }
/// ```
#[must_use]
pub fn authenticated_with_client(
    token: impl Into<String>,
    client: reqwest::Client
) -> Configuration {
    Configuration {
        client,
        ..authenticated(token)
    }
}

/// Builds a [`Configuration`] authenticated with an AI Gateway key.
///
/// The AI Gateway has its own host ([`AI_GATEWAY_BASE_URL`]) and its own key
/// (`sk-...`), created in the control panel or with
/// `POST /api/v1/cloud-ai/ai-keys`. The operations of the
/// `ai_gateway_inference_api` module expect a configuration built this way:
///
/// ```no_run
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// use timeweb_rs::apis::ai_gateway_inference_api;
///
/// let config = timeweb_rs::authenticated_ai_gateway("your-ai-gateway-key");
/// let models = ai_gateway_inference_api::get_ai_gateway_models(&config).await?;
/// println!("{models:#?}");
/// # Ok(())
/// # }
/// ```
#[must_use]
pub fn authenticated_ai_gateway(key: impl Into<String>) -> Configuration {
    Configuration {
        base_path: AI_GATEWAY_BASE_URL.to_string(),
        ..authenticated(key)
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
    use super::{
        AI_GATEWAY_BASE_URL, DEFAULT_BASE_URL, DEFAULT_TIMEOUT, authenticated,
        authenticated_ai_gateway, authenticated_with_base_url, authenticated_with_client
    };

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

    #[test]
    fn authenticated_ai_gateway_targets_the_ai_gateway_host() {
        let config = authenticated_ai_gateway("sk-test");
        assert_eq!(config.base_path, AI_GATEWAY_BASE_URL);
        assert_eq!(config.bearer_access_token.as_deref(), Some("sk-test"));
    }

    #[test]
    fn authenticated_uses_a_client_with_the_default_timeout() {
        let config = authenticated("jwt-token");
        assert_eq!(DEFAULT_TIMEOUT, std::time::Duration::from_secs(30));
        assert!(config.client.get("https://example.com").build().is_ok());
    }

    #[test]
    fn authenticated_with_client_keeps_the_client() {
        let client = reqwest::Client::new();
        let config = authenticated_with_client("jwt-token", client);
        assert_eq!(config.bearer_access_token.as_deref(), Some("jwt-token"));
    }
}
