//! Retrying client wrapper around the generated API operations.

use std::{future::Future, time::Duration};

use crate::{
    apis::{Error, configuration::Configuration},
    authenticated, authenticated_with_base_url
};

/// When and how often a failed operation is retried.
///
/// Retries apply to transport failures, `429 Too Many Requests` and every
/// `5xx` response; other errors are returned immediately. Delays grow
/// exponentially from [`RetryPolicy::base_delay`], doubling per attempt and
/// capped at [`RetryPolicy::max_delay`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryPolicy {
    /// Retries after the initial attempt; `0` disables retrying.
    pub max_retries: u32,
    /// Delay before the first retry.
    pub base_delay:  Duration,
    /// Upper bound for a single backoff delay.
    pub max_delay:   Duration
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay:  Duration::from_millis(500),
            max_delay:   Duration::from_secs(8)
        }
    }
}

impl RetryPolicy {
    /// Backoff delay before the retry with the given zero-based index.
    #[must_use]
    pub fn delay_for(&self, retry_index: u32) -> Duration {
        let factor = 2u32.saturating_pow(retry_index);
        self.base_delay.saturating_mul(factor).min(self.max_delay)
    }
}

/// Whether an operation error is worth retrying.
fn is_retryable<E>(error: &Error<E>) -> bool {
    match error {
        Error::Reqwest(_) => true,
        Error::ResponseError(response) => {
            response.status.as_u16() == 429 || response.status.is_server_error()
        }
        Error::Serde(_) | Error::Io(_) => false
    }
}

/// High-level entry point: an authenticated configuration plus retries.
///
/// The generated operations stay free functions over a
/// [`Configuration`]; this wrapper owns that configuration and re-runs an
/// operation closure according to its [`RetryPolicy`]:
///
/// ```no_run
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// use timeweb_rs::{TimewebClient, apis::servers_api};
///
/// let client = TimewebClient::new("your-jwt-token");
/// let servers = client
///     .execute(|| servers_api::get_servers(client.config(), None, None))
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TimewebClient {
    config: Configuration,
    policy: RetryPolicy
}

impl TimewebClient {
    /// Client for the production API, authenticated with a JWT token.
    #[must_use]
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            config: authenticated(token),
            policy: RetryPolicy::default()
        }
    }

    /// Client for a custom base URL (mock server, proxy).
    #[must_use]
    pub fn with_base_url(token: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            config: authenticated_with_base_url(token, base_url),
            policy: RetryPolicy::default()
        }
    }

    /// Replaces the retry policy.
    #[must_use]
    pub const fn with_policy(mut self, policy: RetryPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// The configuration to pass to the generated operations.
    #[must_use]
    pub const fn config(&self) -> &Configuration {
        &self.config
    }

    /// Runs an operation, retrying it under the client's [`RetryPolicy`].
    ///
    /// The closure is invoked once per attempt. The final error is returned
    /// unchanged, so call sites match on [`Error`] exactly as they would
    /// without retries.
    ///
    /// # Errors
    ///
    /// Returns the last error once the operation fails with a non-retryable
    /// error or the policy's retries are exhausted.
    pub async fn execute<T, E, F, Fut>(&self, operation: F) -> Result<T, Error<E>>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, Error<E>>>
    {
        let mut retries_left = self.policy.max_retries;
        let mut retry_index = 0u32;
        loop {
            match operation().await {
                Err(error) if retries_left > 0 && is_retryable(&error) => {
                    tokio::time::sleep(self.policy.delay_for(retry_index)).await;
                    retries_left -= 1;
                    retry_index += 1;
                }
                result => return result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};

    use super::{RetryPolicy, TimewebClient, is_retryable};
    use crate::apis::{Error, ResponseContent};

    fn response_error(status: u16) -> Error<()> {
        Error::ResponseError(ResponseContent {
            status:  reqwest::StatusCode::from_u16(status).expect("valid status"),
            content: String::new(),
            entity:  None
        })
    }

    fn fast_client() -> TimewebClient {
        TimewebClient::with_base_url("token", "http://localhost:1").with_policy(RetryPolicy {
            max_retries: 3,
            base_delay:  std::time::Duration::ZERO,
            max_delay:   std::time::Duration::ZERO
        })
    }

    #[test]
    fn too_many_requests_and_server_errors_are_retryable() {
        assert!(is_retryable(&response_error(429)));
        assert!(is_retryable(&response_error(500)));
        assert!(is_retryable(&response_error(503)));
    }

    #[test]
    fn client_errors_and_decode_errors_are_not_retryable() {
        assert!(!is_retryable(&response_error(404)));
        assert!(!is_retryable(&response_error(400)));
        let serde_error: Error<()> =
            Error::Serde(serde_json::from_str::<u32>("oops").expect_err("must fail"));
        assert!(!is_retryable(&serde_error));
    }

    #[test]
    fn backoff_doubles_and_saturates_at_the_cap() {
        let policy = RetryPolicy {
            max_retries: 5,
            base_delay:  std::time::Duration::from_millis(100),
            max_delay:   std::time::Duration::from_millis(350)
        };
        assert_eq!(policy.delay_for(0), std::time::Duration::from_millis(100));
        assert_eq!(policy.delay_for(1), std::time::Duration::from_millis(200));
        assert_eq!(policy.delay_for(2), std::time::Duration::from_millis(350));
        assert_eq!(policy.delay_for(9), std::time::Duration::from_millis(350));
    }

    #[test]
    fn default_policy_retries_three_times_from_half_a_second() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_retries, 3);
        assert_eq!(policy.delay_for(0), std::time::Duration::from_millis(500));
        assert_eq!(policy.max_delay, std::time::Duration::from_secs(8));
    }

    #[tokio::test]
    async fn execute_retries_retryable_errors_until_success() {
        let attempts = AtomicU32::new(0);
        let result: Result<u32, Error<()>> = fast_client()
            .execute(|| {
                let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                async move {
                    if attempt < 2 {
                        Err(response_error(429))
                    } else {
                        Ok(7)
                    }
                }
            })
            .await;
        assert_eq!(result.expect("succeeds on the third attempt"), 7);
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn execute_stops_on_non_retryable_errors() {
        let attempts = AtomicU32::new(0);
        let result: Result<u32, Error<()>> = fast_client()
            .execute(|| {
                attempts.fetch_add(1, Ordering::SeqCst);
                async { Err(response_error(404)) }
            })
            .await;
        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn execute_exhausts_retries_and_returns_the_last_error() {
        let attempts = AtomicU32::new(0);
        let result: Result<u32, Error<()>> = fast_client()
            .execute(|| {
                attempts.fetch_add(1, Ordering::SeqCst);
                async { Err(response_error(503)) }
            })
            .await;
        match result {
            Err(Error::ResponseError(response)) => assert_eq!(response.status.as_u16(), 503),
            other => panic!("expected the final 503 to surface, got {other:?}")
        }
        assert_eq!(attempts.load(Ordering::SeqCst), 4);
    }

    #[tokio::test]
    async fn execute_retries_transport_errors() {
        let client = fast_client();
        let result: Result<(), Error<()>> = client
            .execute(|| async {
                match reqwest::get("http://127.0.0.1:1/unreachable").await {
                    Ok(_) => Ok(()),
                    Err(error) => Err(Error::Reqwest(error))
                }
            })
            .await;
        assert!(matches!(result, Err(Error::Reqwest(_))));
    }

    #[test]
    fn client_exposes_its_configuration() {
        let client = TimewebClient::new("jwt");
        assert_eq!(client.config().bearer_access_token.as_deref(), Some("jwt"));
        let custom = TimewebClient::with_base_url("jwt", "http://localhost:8080");
        assert_eq!(custom.config().base_path, "http://localhost:8080");
    }
}
