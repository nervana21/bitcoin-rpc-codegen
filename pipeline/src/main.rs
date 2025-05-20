#![allow(warnings)]
//! **RPC Code Generation Pipeline**
//!
//! This executable provides a structured pipeline for generating strongly-typed Rust RPC client
//! libraries from a Bitcoin Core RPC JSON specification (`api.json`) or help text dump (`help.txt`).
//! It automates the entire workflow, including parsing the RPC schema, cleaning previous outputs,
//! and generating code.
//!
//! ### Pipeline Steps:
//! 1. **CLI Argument Parsing:** Configures paths for input API file and generated output directory.
//!    Supports both `api.json` and `help.txt` inputs, with Bitcoin Core version tagging.
//! 2. **Preparation:** Creates and initializes subdirectories (`transport/`, `types/`, `test_node/`)
//!    with their respective `mod.rs` files.
//! 3. **API Parsing:** Reads and parses the provided input (JSON schema or help text) into a normalized format.
//! 4. **Code Generation:** Generates three main components:
//!    - Transport layer for RPC communication
//!    - Type definitions in `types/latest_types/`
//!    - Test node helper utilities
//! 5. **Namespace Scaffolding:** Creates module structures and exports for easy integration.
//! 6. **Finalization:** Writes the final `lib.rs` with convenient re-exports and a prelude for simple usage.
//!
//! **Usage**: Primarily intended to automate RPC client updates seamlessly, minimizing manual maintenance
//! and ensuring consistent and reliable API compatibility across versions of Bitcoin Core.
//!
//! The generated code is organized into three main directories:
//! - `transport/`: Contains the RPC client implementation and transport layer
//! - `types/`: Contains generated type definitions, with `latest_types/` for the most recent version
//! - `test_node/`: Contains utilities for testing with a Bitcoin node
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};

/// Command-line flags
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Input file (`help.txt` or `api.json`)
    #[arg(short = 'i', long = "input", default_value = "api.json")]
    input: PathBuf,

    /// Directory that will receive the generated `transport/`, `types/`, …
    #[arg(short = 'o', long = "output", default_value = "pipeline/src/generated")]
    output: PathBuf,

    /// Bitcoin Core version tag to embed in the docs (e.g. `v29`)
    #[arg(long = "bitcoin-core-version")]
    core_version: Option<String>,

    /// Emit verbose (debug-level) logs
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

/// Initialise `tracing` with a sensible default filter.
fn init_tracing(verbose: bool) {
    let default_filter = if verbose { "debug" } else { "info" };

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(format!("pipeline={default_filter}"))),
        )
        .init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing(cli.verbose);

    tracing::info!(
        input = %cli.input.display(),
        output = %cli.output.display(),
        core_version = ?cli.core_version,
        "Starting RPC-code-generation pipeline"
    );

    // TODO: pass `cli.core_version` into the pipeline once `pipeline::run`
    // accepts it; for now we simply ignore it.
    pipeline::run(&cli.input, &cli.output)?;

    tracing::info!(
        "✅ Finished — generated sources in {}",
        cli.output.display()
    );
    Ok(())
}
