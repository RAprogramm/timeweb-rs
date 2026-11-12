//! The layout of the generated modules.
//!
//! `openapi-generator` writes the module of a directory into its `mod.rs`
//! file. The repository keeps the modules in the sibling file of the
//! directory instead (`src/apis.rs` next to `src/apis/`), because a module
//! file that is named after its directory is a legacy layout the Rust
//! compiler no longer requires.
//!
//! The task is idempotent: a module that already sits in its sibling file is
//! left alone.

use std::fs;

use masterror::AppResult;

use crate::paths;

/// Moves a generated module file to the sibling file of its directory.
///
/// # Arguments
///
/// * `source` — the module file the generator wrote.
/// * `target` — the file the module belongs to.
///
/// # Errors
///
/// Returns an error when the module file cannot be read or written.
fn move_module(source: &std::path::Path, target: &std::path::Path) -> AppResult<()> {
    if !source.exists() {
        return Ok(());
    }
    let text = fs::read_to_string(source)?;
    fs::write(target, text)?;
    fs::remove_file(source)?;
    Ok(())
}

/// Moves the generated modules to the sibling file of their directory.
///
/// # Errors
///
/// Returns an error when the module files cannot be read or written.
pub fn apply() -> AppResult<()> {
    move_module(&paths::apis_dir().join("mod.rs"), &paths::apis_module())?;
    move_module(&paths::models_dir().join("mod.rs"), &paths::models_module())?;
    println!("moved the generated modules to their sibling files");
    Ok(())
}
