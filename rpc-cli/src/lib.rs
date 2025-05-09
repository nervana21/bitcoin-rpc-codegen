// rpc-cli/src/lib.rs

//! CLI interface for bitcoin-rpc-codegen
//!
//! Provides command-line tools for generating Bitcoin Core RPC clients.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Command-line interface for bitcoin-rpc-codegen
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Bitcoin Core version to generate code for (e.g. "v29")
    #[arg(long = "bitcoin-core-version", value_name = "VERSION")]
    pub core_version: Option<String>,

    /// Path to bitcoind binary (optional)
    #[arg(short = 'b', long = "bitcoind")]
    pub bitcoind_path: Option<PathBuf>,

    /// Output directory for generated code
    #[arg(short = 'o', long = "output")]
    pub output_dir: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}

/// Run the CLI application
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("bitcoin_rpc_codegen={}", level))
        .init();

    // TODO: Implement actual code generation logic
    tracing::info!("CLI initialized with version: {:?}", cli.core_version);
    tracing::info!("Bitcoind path: {:?}", cli.bitcoind_path);
    tracing::info!("Output directory: {:?}", cli.output_dir);

    Ok(())
}
