//! Regeneration of the Cargo features and the `cfg` gates of the generated
//! code.
//!
//! Every API area lives in `src/apis/<area>_api.rs` and is compiled only when
//! its Cargo feature is enabled, so a consumer who needs one area does not
//! compile the rest. The gates are derived from the generated code itself:
//!
//! 1. every `src/apis/<area>_api.rs` becomes a Cargo feature named after the
//!    area (underscores become dashes);
//! 2. the models each area actually uses are computed as the transitive closure
//!    of the `models::Type` references, starting from the area's API file and
//!    following the references between the model files;
//! 3. `src/apis/mod.rs` and `src/models/mod.rs` are rewritten in place: each
//!    `pub mod`/`pub use` line gets a `#[cfg(feature = ...)]` (single area) or
//!    `#[cfg(any(...))]` (shared) attribute. A model used by every area — or,
//!    conservatively, by none — stays unconditional;
//! 4. the marker-delimited block in `Cargo.toml` is regenerated with one empty
//!    feature per area plus a `full` aggregate.
//!
//! The task is idempotent: the attributes it wrote previously are stripped
//! before the new ones are inserted.

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt::Write,
    fs
};

use masterror::AppResult;
use regex::Regex;

use crate::paths;

/// The beginning of the generated feature block in the manifest.
const BEGIN_MARK: &str = "# BEGIN generated area features — openapi/generate_feature_gates.py";

/// The end of the generated feature block in the manifest.
const END_MARK: &str = "# END generated area features";

/// The exported model types of `src/models/mod.rs`.
fn exported_models() -> AppResult<BTreeMap<String, String>> {
    let text = fs::read_to_string(paths::models_module())?;
    let exports = Regex::new(r"pub use self::(\w+)::(\w+);").expect("the export pattern is valid");
    let mut mapping = BTreeMap::new();
    for capture in exports.captures_iter(&text) {
        let Some(module) = capture.get(1) else {
            continue;
        };
        let Some(exported) = capture.get(2) else {
            continue;
        };
        mapping.insert(exported.as_str().to_string(), module.as_str().to_string());
    }
    if mapping.is_empty() {
        return Err(crate::spec::spec_error(
            "no model exports found in src/models/mod.rs"
        ));
    }
    Ok(mapping)
}

/// The model modules each model module references.
fn module_references(
    types: &BTreeMap<String, String>
) -> AppResult<BTreeMap<String, BTreeSet<String>>> {
    let pattern =
        Regex::new(r"\bmodels::([A-Za-z0-9_]+)").expect("the reference pattern is valid");
    let mut edges = BTreeMap::new();
    for entry in fs::read_dir(paths::models_dir())? {
        let path = entry?.path();
        if path.extension().is_none_or(|extension| extension != "rs")
            || path.file_name().and_then(|name| name.to_str()) == Some("mod.rs")
        {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        let text = fs::read_to_string(&path)?;
        let referenced = pattern
            .captures_iter(&text)
            .filter_map(|capture| capture.get(1))
            .filter_map(|name| types.get(name.as_str()))
            .map(|module| (*module).clone())
            .collect::<BTreeSet<String>>();
        let mut referenced = referenced;
        referenced.remove(stem);
        edges.insert(stem.to_string(), referenced);
    }
    Ok(edges)
}

/// The ordered list of API areas paired with the model modules each area
/// transitively uses.
type AreaClosures = (Vec<String>, BTreeMap<String, BTreeSet<String>>);

/// The API areas and the model modules each of them transitively uses.
fn area_closures(
    types: &BTreeMap<String, String>,
    edges: &BTreeMap<String, BTreeSet<String>>
) -> AppResult<AreaClosures> {
    let pattern =
        Regex::new(r"\bmodels::([A-Za-z0-9_]+)").expect("the reference pattern is valid");
    let mut areas = Vec::new();
    let mut closures = BTreeMap::new();
    for entry in fs::read_dir(paths::apis_dir())? {
        let path = entry?.path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        let Some(area) = stem.strip_suffix("_api") else {
            continue;
        };
        areas.push(area.to_string());
        let text = fs::read_to_string(&path)?;
        let mut queue: VecDeque<String> = pattern
            .captures_iter(&text)
            .filter_map(|capture| capture.get(1))
            .filter_map(|name| types.get(name.as_str()))
            .map(|module| (*module).clone())
            .collect();
        let mut closure = BTreeSet::new();
        while let Some(module) = queue.pop_front() {
            if !closure.insert(module.clone()) {
                continue;
            }
            if let Some(referenced) = edges.get(&module) {
                queue.extend(referenced.iter().cloned());
            }
        }
        closures.insert(area.to_string(), closure);
    }
    if areas.is_empty() {
        return Err(crate::spec::spec_error(
            "no *_api.rs modules found under src/apis"
        ));
    }
    areas.sort();
    Ok((areas, closures))
}

/// The model modules that must always compile.
///
/// A module referenced by no area (the generator emits a few orphans) stays
/// unconditional, and so must everything it transitively references —
/// otherwise the orphan would fail to compile in a partial build.
fn unconditional_modules(
    edges: &BTreeMap<String, BTreeSet<String>>,
    closures: &BTreeMap<String, BTreeSet<String>>
) -> BTreeSet<String> {
    let gated: BTreeSet<String> = closures
        .values()
        .flat_map(|closure| closure.iter().cloned())
        .collect();
    let mut queue: VecDeque<String> = edges
        .keys()
        .filter(|module| !gated.contains(*module))
        .cloned()
        .collect();
    let mut always = BTreeSet::new();
    while let Some(module) = queue.pop_front() {
        if !always.insert(module.clone()) {
            continue;
        }
        if let Some(referenced) = edges.get(&module) {
            queue.extend(referenced.iter().cloned());
        }
    }
    always
}

/// The Cargo feature name of an API area.
fn feature_name(area: &str) -> String {
    area.replace('_', "-")
}

/// The `cfg` attribute that gates a model module.
fn cfg_attribute(areas: &[String]) -> String {
    let features: Vec<String> = areas.iter().map(|area| feature_name(area)).collect();
    if features.len() == 1 {
        return format!("#[cfg(feature = \"{}\")]", features[0]);
    }
    let clauses = features
        .iter()
        .map(|feature| format!("feature = \"{feature}\""))
        .collect::<Vec<String>>()
        .join(", ");
    format!("#[cfg(any({clauses}))]")
}

/// Rewrites `src/apis/mod.rs` with one gate per API area.
fn gate_apis_mod(areas: &[String]) -> AppResult<()> {
    let path = paths::apis_module();
    let text = fs::read_to_string(&path)?;
    let strip =
        Regex::new(r"(?m)^\s*#\[cfg\((?:feature|any)[^\n]*\)\]\s*\n").expect("valid pattern");
    let mut text = strip.replace_all(&text, "").to_string();
    for area in areas {
        let declaration = format!("pub mod {area}_api;");
        let gated = format!(
            "#[cfg(feature = \"{}\")]\n{declaration}",
            feature_name(area)
        );
        text = text.replace(&declaration, &gated);
    }
    fs::write(path, text)?;
    Ok(())
}

/// Rewrites `src/models/mod.rs` with the gate of every model module.
fn gate_models_mod(
    areas: &[String],
    closures: &BTreeMap<String, BTreeSet<String>>,
    always: &BTreeSet<String>
) -> AppResult<()> {
    let path = paths::models_module();
    let text = fs::read_to_string(&path)?;
    let modules = Regex::new(r"(?m)^\s*pub mod (\w+);").expect("valid pattern");
    let exports = Regex::new(r"(?m)^\s*pub use self::(\w+)::(\w+);").expect("valid pattern");

    let mut module_names = Vec::new();
    for capture in modules.captures_iter(&text) {
        if let Some(name) = capture.get(1) {
            module_names.push(name.as_str().to_string());
        }
    }
    let mut module_exports: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for capture in exports.captures_iter(&text) {
        let (Some(module), Some(exported)) = (capture.get(1), capture.get(2)) else {
            continue;
        };
        module_exports
            .entry(module.as_str().to_string())
            .or_default()
            .push(exported.as_str().to_string());
    }
    if module_names.len() != module_exports.len() {
        return Err(crate::spec::spec_error(format!(
            "src/models/mod.rs has {} modules but exports for {}",
            module_names.len(),
            module_exports.len()
        )));
    }

    let mut used_by: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for area in areas {
        if let Some(closure) = closures.get(area) {
            for module in closure {
                used_by
                    .entry(module.clone())
                    .or_default()
                    .push(area.clone());
            }
        }
    }

    let mut output = String::new();
    for module in &module_names {
        let mut attribute = String::new();
        if !always.contains(module) {
            let mut areas_using = used_by.get(module).cloned().unwrap_or_default();
            areas_using.sort();
            if !areas_using.is_empty() && areas_using.len() < areas.len() {
                attribute = format!("{}\n", cfg_attribute(&areas_using));
            }
        }
        let _ = writeln!(output, "{attribute}pub mod {module};");
        for exported in module_exports.get(module).into_iter().flatten() {
            let _ = writeln!(output, "{attribute}pub use self::{module}::{exported};");
        }
    }
    fs::write(path, output)?;
    Ok(())
}

/// Rewrites the generated feature block of the manifest.
fn rewrite_cargo_features(areas: &[String]) -> AppResult<()> {
    let path = paths::manifest();
    let text = fs::read_to_string(&path)?;
    if !text.contains(BEGIN_MARK) || !text.contains(END_MARK) {
        return Err(crate::spec::spec_error(format!(
            "markers not found in Cargo.toml; add {BEGIN_MARK:?} and {END_MARK:?}"
        )));
    }
    let mut block = String::from(BEGIN_MARK);
    for area in areas {
        let _ = write!(block, "\n{} = []", feature_name(area));
    }
    block.push_str("\nfull = [");
    for area in areas {
        let _ = write!(block, "\n  \"{}\",", feature_name(area));
    }
    block.push_str("\n]");
    let _ = write!(block, "\n{END_MARK}");

    let pattern = Regex::new(&format!(
        "{}(?s).*?{}",
        regex::escape(BEGIN_MARK),
        regex::escape(END_MARK)
    ))
    .expect("the marker pattern is valid");
    let text = pattern.replace(&text, block.as_str()).to_string();
    fs::write(path, text)?;
    Ok(())
}

/// Regenerates the features and the gates of the generated code.
///
/// # Errors
///
/// Returns an error when the generated sources cannot be read or written.
pub fn regenerate() -> AppResult<()> {
    let types = exported_models()?;
    let edges = module_references(&types)?;
    let (areas, closures) = area_closures(&types, &edges)?;
    let always = unconditional_modules(&edges, &closures);
    gate_apis_mod(&areas)?;
    gate_models_mod(&areas, &closures, &always)?;
    rewrite_cargo_features(&areas)?;
    let total = edges.len();
    let gated: BTreeSet<&String> = closures
        .values()
        .flat_map(|closure| closure.iter())
        .filter(|module| !always.contains(*module))
        .collect();
    println!(
        "gated {} areas; {total} model modules ({} unconditional, {} feature-gated)",
        areas.len(),
        total - gated.len(),
        gated.len()
    );
    Ok(())
}
