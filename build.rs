use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=api.json");
    println!("cargo:rerun-if-changed=help.txt");
    println!("cargo:rerun-if-changed=bitcoin-rpc-midas");

    let workspace_root = env::current_dir().context("Failed to get current working directory")?;
    let input_path = workspace_root.join("api.json");

    pipeline::run(&input_path)?;

    Ok(())
}
