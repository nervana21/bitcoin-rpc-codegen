// pipeline/src/main.rs

use anyhow::Result;
use clap::Parser as ClapParser;
use logging::init;
use std::path::PathBuf;

/// Simple pipeline: parse help → normalize → generate RPC modules.
#[derive(ClapParser)]
#[command(author, version, about)]
struct Opts {
    /// Path to the `bitcoin-cli help` dump (required if not using discovery mode)
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Path to the bitcoin-cli binary (required if using discovery mode)
    #[arg(short, long)]
    bitcoind: Option<PathBuf>,

    /// Where to write generated modules (e.g. client/src/generated)
    #[arg(short, long, default_value = "../client/src/generated")]
    out_dir: PathBuf,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // Initialize logging with appropriate level
    if opts.verbose {
        init_with_level(tracing::Level::DEBUG);
    } else {
        init();
    }

    // Validate that either input or bitcoind is provided
    if opts.input.is_none() && opts.bitcoind.is_none() {
        anyhow::bail!("Either --input or --bitcoind must be provided");
    }

    if let Some(bitcoind) = opts.bitcoind {
        tracing::info!(
            "Starting discovery pipeline; bitcoind={:?}, out_dir={:?}",
            bitcoind,
            opts.out_dir
        );
        pipeline::run_discovery(&bitcoind, &opts.out_dir)
    } else {
        let input = opts.input.unwrap();
        tracing::info!(
            "Starting file-based pipeline; input={:?}, out_dir={:?}",
            input,
            opts.out_dir
        );
        pipeline::run(&input, &opts.out_dir)
    }
}

fn init_with_level(level: tracing::Level) {
    use tracing_subscriber::FmtSubscriber;
    FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_level(true)
        .with_ansi(true)
        .pretty()
        .init();
}
