# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/RAprogramm/timeweb-rs/compare/v0.3.0...v0.4.0) - 2026-06-30

### Fixed

- Relax ip type enum and apps frontend preset requests for live deserialization ([#37](https://github.com/RAprogramm/timeweb-rs/pull/37)) ([d204134](https://github.com/RAprogramm/timeweb-rs/commit/d20413455c72dab93df744297ab4e521c894bce7))


## [0.2.0](https://github.com/RAprogramm/timeweb-rs/compare/v0.1.4...v0.2.0) - 2026-06-30

**Breaking release.** Re-publishes the breaking API-model changes that first
shipped in 0.1.4 under a correct SemVer increment — for a `0.x` crate the minor
component is the version-significant bump for breaking changes, so consumers
pinned to `0.1` are not silently broken. Prefer this version over 0.1.4.

### Changed

- **BREAKING:** numeric identifier fields are now `i64` integers instead of
  `f64` floats, so ids serialize and display as integers.
- **BREAKING:** the account `Status` model gains `login`, `registered_at`,
  `is_password_set` and `two_factor_method` fields returned by
  `GET /account/status`.
- **BREAKING:** response collection properties are renamed to match what the
  live API actually sends — `ssh-keys` → `ssh_keys` and `knowledgebases` →
  `knowledge_bases`.
- Raised the minimum supported Rust version (MSRV) to 1.96.

## [0.1.4](https://github.com/RAprogramm/timeweb-rs/compare/v0.1.3...v0.1.4) - 2026-06-30

Superseded by 0.2.0: this tag carried the breaking account-status,
property-rename and integer-id changes under a patch number before they were
re-released with a correct SemVer bump. Use 0.2.0 instead.

### Fixed

- Correct the `ssh_keys` response field name so SSH key listings deserialize
  instead of returning an empty list.

### Changed

- **BREAKING:** account status `login` field, response property renames and
  integer (`i64`) identifier fields (see 0.2.0 for the full description).

## [0.1.3](https://github.com/RAprogramm/timeweb-rs/compare/v0.1.2...v0.1.3) - 2026-05-28

### Fixed

- tolerate null response_id from the Timeweb API ([#18](https://github.com/RAprogramm/timeweb-rs/pull/18))

## [0.1.2](https://github.com/RAprogramm/timeweb-rs/compare/v0.1.1...v0.1.2) - 2026-05-21

### Changed

- Add a dedicated Codecov coverage job with upload and test analytics, a
  `nextest` profile and a coverage badge ([#8](https://github.com/RAprogramm/timeweb-rs/pull/8)).
  No library code changes.

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

[Unreleased]: https://github.com/RAprogramm/timeweb-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/RAprogramm/timeweb-rs/releases/tag/v0.2.0
[0.1.4]: https://github.com/RAprogramm/timeweb-rs/releases/tag/v0.1.4
[0.1.3]: https://github.com/RAprogramm/timeweb-rs/releases/tag/v0.1.3
[0.1.2]: https://github.com/RAprogramm/timeweb-rs/releases/tag/v0.1.2
[0.1.1]: https://github.com/RAprogramm/timeweb-rs/releases/tag/v0.1.1
[0.1.0]: https://github.com/RAprogramm/timeweb-rs/releases/tag/v0.1.0
</content>
</invoke>
