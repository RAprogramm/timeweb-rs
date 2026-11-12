//! Refresh of the statistics quoted in the README.
//!
//! The README quotes the number of API operations and areas and lists every
//! generated `apis` module. Those figures are derived facts: one module per
//! API area lives in `src/apis/<area>_api.rs` and every operation is a
//! `pub async fn` inside one of those modules. This module recomputes both
//! from the working tree and rewrites the two README fragments that quote
//! them, so a regeneration commit can never leave the documentation stale.
//!
//! The task is deterministic: the module list is sorted and wrapped at 79
//! columns, so consecutive runs produce byte-identical output.

use std::fs;

use masterror::AppResult;
use regex::Regex;

use crate::paths;

/// The width the API coverage paragraph is wrapped at.
const COVERAGE_WIDTH: usize = 79;

/// The names of the generated API modules.
fn collect_modules() -> AppResult<Vec<String>> {
    let mut modules = Vec::new();
    for entry in fs::read_dir(paths::apis_dir())? {
        let path = entry?.path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        if !stem.ends_with("_api") {
            continue;
        }
        modules.push(stem.to_string());
    }
    if modules.is_empty() {
        return Err(crate::spec::spec_error(
            "no *_api.rs modules found under src/apis"
        ));
    }
    modules.sort();
    Ok(modules)
}

/// The number of generated operations across the API modules.
fn count_operations(modules: &[String]) -> AppResult<usize> {
    let pattern = Regex::new(r"(?m)^pub async fn ").expect("the operation pattern is valid");
    let mut total = 0;
    for module in modules {
        let text = fs::read_to_string(paths::apis_dir().join(format!("{module}.rs")))?;
        total += pattern.find_iter(&text).count();
    }
    Ok(total)
}

/// Wraps the paragraph the way the previous implementation did.
fn wrap(text: &str, width: usize) -> String {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if line.is_empty() {
            line.push_str(word);
            continue;
        }
        if line.chars().count() + 1 + word.chars().count() <= width {
            line.push(' ');
            line.push_str(word);
            continue;
        }
        lines.push(std::mem::take(&mut line));
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines.join("\n")
}

/// Renders the API coverage paragraph.
fn render_coverage(modules: &[String]) -> String {
    let names = modules
        .iter()
        .map(|module| format!("`{module}`"))
        .collect::<Vec<String>>()
        .join(", ");
    let paragraph = format!("One module per API area: {names}.");
    format!("{}\n", wrap(&paragraph, COVERAGE_WIDTH))
}

/// Rewrites the derived fragments of the README.
///
/// # Arguments
///
/// * `text` — the current README text.
/// * `modules` — the names of the generated API modules.
///
/// # Errors
///
/// Returns an error when the README does not contain the expected fragments.
pub fn updated_readme(text: &str, modules: &[String]) -> AppResult<String> {
    let operations = count_operations(modules)?;
    let stats =
        Regex::new(r"\*\*\d+( operations across\s+)\d+( areas\*\*)").expect("valid pattern");
    if stats.find(text).is_none() {
        return Err(crate::spec::spec_error(
            "expected exactly one operations-count fragment in the README"
        ));
    }
    let mut text = stats
        .replace_all(text, |captures: &regex::Captures<'_>| {
            format!(
                "**{operations}{}{}{}",
                &captures[1],
                modules.len(),
                &captures[2]
            )
        })
        .to_string();

    let coverage = Regex::new(r"(?s)(## API coverage\n\n)One module per API area:.*?\.\n")
        .expect("valid pattern");
    if coverage.find(&text).is_none() {
        return Err(crate::spec::spec_error(
            "expected exactly one API-coverage paragraph in the README"
        ));
    }
    text = coverage
        .replace_all(&text, |captures: &regex::Captures<'_>| {
            format!("{}{}", &captures[1], render_coverage(modules))
        })
        .to_string();
    Ok(text)
}

/// Refreshes the statistics quoted in the README.
///
/// # Arguments
///
/// * `check` — fail instead of rewriting the README when it is stale.
///
/// # Errors
///
/// Returns an error when the README cannot be read or written, or when it is
/// stale and `check` is set.
pub fn refresh(check: bool) -> AppResult<()> {
    let path = paths::readme();
    let current = fs::read_to_string(&path)?;
    let modules = collect_modules()?;
    let fresh = updated_readme(&current, &modules)?;
    if fresh == current {
        println!("README is up to date.");
        return Ok(());
    }
    if check {
        return Err(crate::spec::spec_error(
            "README statistics are stale; run `cargo xtask readme`"
        ));
    }
    fs::write(path, fresh)?;
    println!("README statistics updated.");
    Ok(())
}
