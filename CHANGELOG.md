# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/RAprogramm/timeweb-rs/compare/v0.1.0...v0.1.1) - 2026-05-21

### Documentation

- docs add security policy
- docs use referral link for timeweb cloud
- docs add changelog and release process
- docs add crates.io docs.rs and msrv badges

## [0.1.0] - 2026-05-20

### Added

- Initial release: async Rust SDK covering the full Timeweb Cloud API —
  313 operations across 22 areas, generated from the official OpenAPI
  specification with `openapi-generator`.
- `authenticated` and `authenticated_with_base_url` helpers for building an
  authenticated API `Configuration`.
- `openapi/normalize_spec.py` — documented spec normalizer used during
  regeneration.

[Unreleased]: https://github.com/RAprogramm/timeweb-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/RAprogramm/timeweb-rs/releases/tag/v0.1.0
