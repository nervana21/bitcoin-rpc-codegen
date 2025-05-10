// rpc-cli/src/main.rs

use anyhow::Result;
use clap::Parser;
use config::Config;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Path to a config file. If omitted, we'll use the default config dir.
    #[arg(long)]
    pub config: Option<PathBuf>,
    /// Override the Bitcoin Core version (takes precedence over config file)
    #[arg(long)]
    pub bitcoin_core_version: Option<String>,
}

fn main() -> Result<()> {
    // 1) parse CLI → load config
    let cli = Cli::parse();
    let mut cfg = if let Some(path) = cli.config {
        Config::from_file(path)?
    } else {
        Config::default()
    };

    // 2) apply any overrides
    if let Some(v) = cli.bitcoin_core_version {
        cfg.bitcoin.core_version = Some(v);
    }

    // 3) init logging & metrics
    logging::init();
    if cfg.metrics.enabled {
        rpc_metrics::init(&format!("0.0.0.0:{}", cfg.metrics.port))?;
    }

    // 4) run the pipeline
    pipeline::run(&cfg.pipeline.input_path, &cfg.pipeline.output_dir)?;

    Ok(())
}
