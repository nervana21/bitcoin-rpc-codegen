// pipeline/src/main.rs

use anyhow::Result;
use clap::Parser as ClapParser;
use logging::init;
use std::path::PathBuf;

/// Simple pipeline: parse help → normalize → generate RPC modules.
#[derive(ClapParser)]
#[command(author, version, about)]
struct Opts {
    /// Path to the `bitcoin-cli help` dump
    #[arg(short, long, default_value = "../resources/help.txt")]
    input: PathBuf,

    /// Where to write generated modules (e.g. client/src/generated)
    #[arg(short, long, default_value = "../client/src/generated")]
    out_dir: PathBuf,
}

fn main() -> Result<()> {
    init();

    let opts = Opts::parse();
    tracing::info!(
        "Starting pipeline; input={:?}, out_dir={:?}",
        opts.input,
        opts.out_dir
    );

    pipeline::run(&opts.input, &opts.out_dir)
}
