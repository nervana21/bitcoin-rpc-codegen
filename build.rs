use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=api.json");
    println!("cargo:rerun-if-changed=bitcoin-rpc-midas");

    let workspace_root = env::current_dir().context("Failed to get current working directory")?;
    let input_path = workspace_root.join("api.json");

    let version = "v29";

    println!("cargo:rustc-env=BITCOIN_CORE_VERSION={version}");

    pipeline::run(&input_path)?;

    Ok(())
}
