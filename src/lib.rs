//! # timeweb-rs
//!
//! Async Rust SDK for the [Timeweb Cloud](https://timeweb.cloud/?i=137383) API.
//!
//! The [`apis`] and [`models`] modules are generated from the official Timeweb
//! Cloud `OpenAPI` specification, so the SDK covers the full public API
//! surface: cloud servers, managed databases, Kubernetes, projects, domains, S3
//! storage, load balancers, firewalls, mail, AI agents and everything else
//! exposed by the control panel.
//!
//! ## Authentication
//!
//! Timeweb Cloud authenticates requests with a JWT token issued in the control
//! panel (section "API и Terraform"). Build an authenticated configuration
//! with [`authenticated`] and pass it to any function in [`apis`]:
//!
//! ```no_run
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use timeweb_rs::apis::servers_api;
//!
//! let config = timeweb_rs::authenticated("your-jwt-token");
//! let servers = servers_api::get_servers(&config, None, None).await?;
//! println!("{servers:#?}");
//! # Ok(())
//! # }
//! ```
//!
//! ## Layout
//!
//! * [`apis`] — one module per API area ([`apis::servers_api`],
//!   [`apis::databases_api`], ...); every operation is a free async function
//!   taking `&Configuration` as its first argument.
//! * [`models`] — request and response types.
//! * [`apis::configuration::Configuration`] — connection settings and
//!   credentials; build it with [`authenticated`].
//! * [`apis::Error`] — the error type returned by every API call.

/// Generated API client modules, one per Timeweb Cloud API area.
///
/// Produced by `openapi-generator`; lint groups are relaxed for this generated
/// code so the hand-written crate code stays under strict lints.
#[allow(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    unused_imports,
    rustdoc::all
)]
pub mod apis;

/// Generated request and response types.
///
/// Produced by `openapi-generator`; see [`apis`] for the lint rationale.
#[allow(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    unused_imports,
    rustdoc::all
)]
pub mod models;

mod client;

pub use client::{DEFAULT_BASE_URL, authenticated, authenticated_with_base_url};
