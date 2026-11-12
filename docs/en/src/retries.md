# Retries and timeouts

## The timeout

Every configuration built with `authenticated` uses an HTTP client with a
30 second timeout, so a stalled connection cannot block a task forever. The
timeout is a constant:

```rust
use std::time::Duration;
use timeweb_rs::DEFAULT_TIMEOUT;

assert_eq!(DEFAULT_TIMEOUT, Duration::from_secs(30));
```

A request that runs into the timeout fails with `Error::Reqwest`.

Bring your own client when the default does not fit — a proxy, a connection
pool or a different timeout:

```rust
let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(5))
    .build()?;
let config = timeweb_rs::authenticated_with_client("your-jwt-token", client);
```

## The retry policy

`TimewebClient` retries an operation when the failure is worth retrying:

- a transport failure — the request never reached the API;
- `429 Too Many Requests`;
- any `5xx` response.

Every other error is returned immediately. The delay grows exponentially from
the base delay, doubles per attempt and is capped by the maximum delay; half
of it is random, so the clients of a fleet do not retry in lockstep.

```rust
use std::time::Duration;
use timeweb_rs::RetryPolicy;

let policy = RetryPolicy {
    max_retries: 5,
    base_delay:  Duration::from_millis(200),
    max_delay:   Duration::from_secs(10),
};

let client = TimewebClient::new("your-jwt-token").with_policy(policy);
```

The default policy retries three times, starting from 500 milliseconds and
capping the delay at eight seconds.

## Retry-After

A `429` may carry a `Retry-After` header, either as a number of seconds or as
an HTTP date. The client honors it: the delay becomes exactly what the API
asked for, capped by the maximum delay of the policy.

## Idempotency

**Only retry idempotent operations.** A `POST` that creates a resource may
reach the API even when the response never arrives, so retrying it can create
the resource twice.

`execute` retries whatever you give it, because the policy is a property of
the client, not of the operation. For an operation that is not idempotent,
use `execute_once`:

```rust
let server = client
    .execute_once(|| servers_api::create_server(client.config(), &request))
    .await?;
```

## The policy is public

`RetryPolicy::delay_for` returns the nominal delay of a retry — the delay
before the jitter is applied. It is deterministic, which makes it easy to
assert on in tests.
