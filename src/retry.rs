//! Retrying client wrapper around the generated API operations.

use std::{
    future::Future,
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH}
};

use crate::{
    apis::{Error, configuration::Configuration},
    authenticated, authenticated_ai_gateway, authenticated_with_base_url
};

/// The state of the jitter generator.
static JITTER_STATE: AtomicU64 = AtomicU64::new(0);

/// When and how often a failed operation is retried.
///
/// Retries apply to transport failures, `429 Too Many Requests` and every
/// `5xx` response; other errors are returned immediately. Delays grow
/// exponentially from [`RetryPolicy::base_delay`], doubling per attempt and
/// capped at [`RetryPolicy::max_delay`]. The client adds jitter to the delay
/// so that the clients of a fleet do not retry in lockstep, and honors the
/// `Retry-After` header of a rate-limit response.
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
    ///
    /// The delay is the nominal one: the client jitters it before sleeping,
    /// so that the clients of a fleet do not retry in lockstep.
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

/// The delay a rate-limit response asks the client to wait.
///
/// The header carries either a number of seconds or an HTTP date.
fn retry_after<E>(error: &Error<E>) -> Option<Duration> {
    let Error::ResponseError(response) = error else {
        return None;
    };
    let value = response
        .headers
        .get(reqwest::header::RETRY_AFTER)?
        .to_str()
        .ok()?
        .trim();
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    let deadline = chrono::DateTime::parse_from_rfc2822(value).ok()?;
    let now = chrono::Utc::now();
    let delay = deadline.with_timezone(&chrono::Utc) - now;
    Some(delay.to_std().unwrap_or_default())
}

/// A pseudo-random number for the backoff jitter.
fn jitter_seed() -> u64 {
    let mut state = JITTER_STATE.fetch_add(0x9E37_79B9_7F4A_7C15, Ordering::Relaxed);
    if state == 0 {
        state = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0x853c_49e6_748f_ea9b, |elapsed| {
                u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX)
            });
    }
    state ^= state >> 30;
    state = state.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    state ^= state >> 27;
    state = state.wrapping_mul(0x94d0_49bb_1331_11eb);
    state ^ (state >> 31)
}

/// The delay with the jitter applied.
///
/// Half of the delay is fixed and half is random, so the clients of a fleet
/// spread their retries over the window instead of retrying in lockstep.
fn jittered(delay: Duration) -> Duration {
    let floor = delay / 2;
    let spread = delay.checked_sub(floor).unwrap_or_default();
    let nanos = u64::try_from(spread.as_nanos()).unwrap_or(u64::MAX);
    if nanos == 0 {
        return floor;
    }
    floor + Duration::from_nanos(jitter_seed() % nanos)
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

    /// Client for the AI Gateway API, authenticated with an AI Gateway key.
    #[must_use]
    pub fn ai_gateway(key: impl Into<String>) -> Self {
        Self {
            config: authenticated_ai_gateway(key),
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
    /// Only idempotent operations should be retried: a `POST` that creates a
    /// resource may reach the server even when the response never arrives, so
    /// retrying it can create the resource twice. Use
    /// [`TimewebClient::execute_once`] for the operations that are not
    /// idempotent.
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
                    let delay = retry_after(&error).map_or_else(
                        || jittered(self.policy.delay_for(retry_index)),
                        |delay| delay.min(self.policy.max_delay)
                    );
                    tokio::time::sleep(delay).await;
                    retries_left -= 1;
                    retry_index += 1;
                }
                result => return result
            }
        }
    }

    /// Runs an operation once, without retries.
    ///
    /// Use it for the operations that are not idempotent: a `POST` that
    /// creates a resource may reach the server even when the response never
    /// arrives, so retrying it can create the resource twice.
    ///
    /// # Errors
    ///
    /// Returns the error of the operation.
    pub async fn execute_once<T, E, F, Fut>(&self, operation: F) -> Result<T, Error<E>>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, Error<E>>>
    {
        operation().await
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::atomic::{AtomicU32, Ordering},
        time::Duration
    };

    use super::{RetryPolicy, TimewebClient, is_retryable, retry_after};
    use crate::apis::{Error, ResponseContent};

    fn response_error(status: u16) -> Error<()> {
        Error::ResponseError(ResponseContent {
            status:  reqwest::StatusCode::from_u16(status).expect("valid status"),
            content: String::new(),
            entity:  None,
            headers: reqwest::header::HeaderMap::new()
        })
    }

    fn response_error_with_retry_after(status: u16, value: &str) -> Error<()> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::RETRY_AFTER,
            reqwest::header::HeaderValue::from_str(value).expect("valid header")
        );
        Error::ResponseError(ResponseContent {
            status: reqwest::StatusCode::from_u16(status).expect("valid status"),
            content: String::new(),
            entity: None,
            headers
        })
    }

    fn fast_client() -> TimewebClient {
        TimewebClient::with_base_url("token", "http://localhost:1").with_policy(RetryPolicy {
            max_retries: 3,
            base_delay:  Duration::ZERO,
            max_delay:   Duration::ZERO
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
            base_delay:  Duration::from_millis(100),
            max_delay:   Duration::from_millis(350)
        };
        assert_eq!(policy.delay_for(0), Duration::from_millis(100));
        assert_eq!(policy.delay_for(1), Duration::from_millis(200));
        assert_eq!(policy.delay_for(2), Duration::from_millis(350));
        assert_eq!(policy.delay_for(9), Duration::from_millis(350));
    }

    #[test]
    fn default_policy_retries_three_times_from_half_a_second() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_retries, 3);
        assert_eq!(policy.delay_for(0), Duration::from_millis(500));
        assert_eq!(policy.max_delay, Duration::from_secs(8));
    }

    #[test]
    fn the_retry_after_header_is_read() {
        let error = response_error_with_retry_after(429, "12");
        assert_eq!(retry_after(&error), Some(Duration::from_secs(12)));
    }

    #[test]
    fn a_response_without_a_retry_after_header_has_no_delay() {
        let error = response_error(429);
        assert_eq!(retry_after(&error), None);
    }

    #[test]
    fn a_retry_after_date_is_read() {
        let deadline = chrono::Utc::now() + chrono::Duration::seconds(30);
        let value = deadline.to_rfc2822();
        let error = response_error_with_retry_after(429, &value);
        let delay = retry_after(&error).expect("the header carries a date");
        assert!(delay <= Duration::from_secs(30));
        assert!(delay >= Duration::from_secs(25));
    }

    #[test]
    fn the_jitter_stays_inside_the_delay() {
        let delay = Duration::from_secs(10);
        for _ in 0..16 {
            let jittered = super::jittered(delay);
            assert!(jittered >= delay / 2);
            assert!(jittered <= delay);
        }
    }

    #[test]
    fn the_jitter_of_a_zero_delay_is_zero() {
        assert_eq!(super::jittered(Duration::ZERO), Duration::ZERO);
    }

    #[test]
    fn the_jitter_seed_is_a_number() {
        let seed = super::jitter_seed();
        assert_ne!(seed, 0);
    }

    #[test]
    fn the_ai_gateway_client_targets_the_ai_gateway_host() {
        let client = TimewebClient::ai_gateway("sk-test");
        assert_eq!(client.config().base_path, crate::AI_GATEWAY_BASE_URL);
        assert_eq!(
            client.config().bearer_access_token.as_deref(),
            Some("sk-test")
        );
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
                let error = reqwest::get("http://127.0.0.1:1/unreachable")
                    .await
                    .expect_err("the address is unreachable");
                Err(Error::Reqwest(error))
            })
            .await;
        assert!(matches!(result, Err(Error::Reqwest(_))));
    }

    #[tokio::test]
    async fn execute_once_runs_the_operation_a_single_time() {
        let attempts = AtomicU32::new(0);
        let result: Result<u32, Error<()>> = fast_client()
            .execute_once(|| {
                attempts.fetch_add(1, Ordering::SeqCst);
                async { Err(response_error(503)) }
            })
            .await;
        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn client_exposes_its_configuration() {
        let client = TimewebClient::new("jwt");
        assert_eq!(client.config().bearer_access_token.as_deref(), Some("jwt"));
        let custom = TimewebClient::with_base_url("jwt", "http://localhost:8080");
        assert_eq!(custom.config().base_path, "http://localhost:8080");
    }
}
