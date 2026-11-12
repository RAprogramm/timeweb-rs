//! Filesystem locations the automation tasks work with.

use std::path::{Path, PathBuf};

/// The root directory of the repository.
///
/// The location is derived from the manifest of this crate, so the tasks work
/// the same way no matter which directory they are started from.
///
/// # Example
///
/// ```rust
/// assert!(paths::repo_root().join("Cargo.toml").is_file());
/// ```
#[must_use]
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
}

/// The vendored Timeweb Cloud OpenAPI specification.
#[must_use]
pub fn spec_file() -> PathBuf {
    repo_root().join("openapi/timeweb-cloud.json")
}

/// The directory with the generated API modules.
#[must_use]
pub fn apis_dir() -> PathBuf {
    repo_root().join("src/apis")
}

/// The module with the generated API modules.
#[must_use]
pub fn apis_module() -> PathBuf {
    repo_root().join("src/apis.rs")
}

/// The directory with the generated models.
#[must_use]
pub fn models_dir() -> PathBuf {
    repo_root().join("src/models")
}

/// The module with the generated models.
#[must_use]
pub fn models_module() -> PathBuf {
    repo_root().join("src/models.rs")
}

/// The manifest of the published crate.
#[must_use]
pub fn manifest() -> PathBuf {
    repo_root().join("Cargo.toml")
}

/// The README of the published crate.
#[must_use]
pub fn readme() -> PathBuf {
    repo_root().join("README.md")
}

/// The generated deserialization tests.
#[must_use]
pub fn example_tests() -> PathBuf {
    repo_root().join("tests/generated_spec_examples.rs")
}
