//! Repository automation for the `timeweb-rs` workspace.
//!
//! The published crate is generated from the Timeweb Cloud OpenAPI
//! specification. This binary carries the whole generation pipeline: it
//! normalizes the upstream specification, regenerates the Cargo features and
//! the `cfg` gates of the generated modules, post-processes the generated
//! models and rewrites the derived facts quoted in the README.
//!
//! Every task is deterministic and idempotent, so the pipeline can be run
//! locally and in CI without producing spurious diffs.
//!
//! # Usage
//!
//! ```sh
//! cargo xtask normalize-spec openapi/timeweb-cloud.json /tmp/normalized.json
//! cargo xtask feature-gates
//! cargo xtask example-tests
//! cargo xtask patch-models
//! cargo xtask readme --check
//! ```

mod examples;
mod features;
mod layout;
mod models;
mod paths;
mod readme;
mod spec;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use masterror::AppResult;

/// Command-line interface of the workspace automation tasks.
#[derive(Debug, Parser)]
#[command(
    name = "xtask",
    about = "Repository automation for the timeweb-rs workspace",
    version
)]
struct Cli {
    /// The task to run.
    #[command(subcommand)]
    command: Command
}

/// The tasks the automation binary provides.
#[derive(Debug, Subcommand)]
enum Command {
    /// Normalize an OpenAPI specification before the client generation.
    NormalizeSpec {
        /// The specification to read.
        input:  PathBuf,
        /// The file to write the normalized specification to.
        output: PathBuf
    },
    /// Regenerate the Cargo features and the `cfg` gates of the generated code.
    FeatureGates,
    /// Move the generated modules to the sibling file of their directory.
    Layout,
    /// Regenerate the deserialization tests built from the spec examples.
    ExampleTests,
    /// Post-process the generated models so the API responses deserialize.
    PatchModels,
    /// Refresh the statistics quoted in the README.
    Readme {
        /// Fail instead of rewriting the README when it is stale.
        #[arg(long)]
        check: bool
    }
}

fn main() -> AppResult<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::NormalizeSpec {
            input,
            output
        } => spec::normalize_file(&input, &output),
        Command::FeatureGates => features::regenerate(),
        Command::Layout => layout::apply(),
        Command::ExampleTests => examples::regenerate(),
        Command::PatchModels => models::patch_all(),
        Command::Readme {
            check
        } => readme::refresh(check)
    }
}
