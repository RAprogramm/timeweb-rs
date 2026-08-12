# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0](https://github.com/RAprogramm/timeweb-rs/compare/v0.7.0...v0.8.0) - 2026-08-12

### Added

- **BREAKING:** Regenerate SDK from updated Timeweb API spec (#68) ([3ba43a9](https://github.com/RAprogramm/timeweb-rs/commit/3ba43a9bea0458e0fab8bea64887de1762956e7d))

- Regenerate SDK from updated Timeweb API spec (#64) ([d4166a4](https://github.com/RAprogramm/timeweb-rs/commit/d4166a496d2ae8981d6dff439c283b96b09ae8cb))


## [0.7.0](https://github.com/RAprogramm/timeweb-rs/compare/v0.6.0...v0.7.0) - 2026-07-14

### Added

- **BREAKING:** Accept undocumented deploy status building from the live API (#61) ([47b00af](https://github.com/RAprogramm/timeweb-rs/commit/47b00af58f0c3cce411a4e8a19f568eff3a91b62))


## [0.6.0](https://github.com/RAprogramm/timeweb-rs/compare/v0.5.1...v0.6.0) - 2026-07-13

### Added

- **BREAKING:** Regenerate SDK from updated Timeweb API spec (#50) ([bd8a602](https://github.com/RAprogramm/timeweb-rs/commit/bd8a60241564451b98501e1d2b5534a14d5ce93b))

- Gate every API area behind its own Cargo feature (#59) ([5adead7](https://github.com/RAprogramm/timeweb-rs/commit/5adead7a5ce7e306d3271f65046d3876325fb9c6))

- Add retrying client, pagination stream and error envelope parsing (#57) ([74a35a3](https://github.com/RAprogramm/timeweb-rs/commit/74a35a37178432565550d5666e8297110799f4fe))


### Testing

- Generate deserialization tests from spec response examples (#55) ([da85c02](https://github.com/RAprogramm/timeweb-rs/commit/da85c028fa13ebef4b7e921a0847aff18a3906ec))


## [0.5.1](https://github.com/RAprogramm/timeweb-rs/compare/v0.5.0...v0.5.1) - 2026-07-05

### Other

- #42 fix: type deploy timestamps as strings to survive naive API datetimes ([#43](https://github.com/RAprogramm/timeweb-rs/pull/43)) ([f90bff3](https://github.com/RAprogramm/timeweb-rs/commit/f90bff3b158443eae293d91e4a75b8bd47c97a9a))


## [0.5.0](https://github.com/RAprogramm/timeweb-rs/compare/v0.4.0...v0.5.0) - 2026-07-03

### Other

- #39 fix: expose project apps collection and app project_id the API returns ([08e87d8](https://github.com/RAprogramm/timeweb-rs/commit/08e87d85efb70c08f0a2145e93304e8bb668f882))


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
