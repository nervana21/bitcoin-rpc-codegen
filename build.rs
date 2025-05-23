use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Tell Cargo to rerun this build script if the input files change
    println!("cargo:rerun-if-changed=api.json");
    println!("cargo:rerun-if-changed=help.txt");
    println!("cargo:rerun-if-changed=midas");

    // Get workspace root and input path
    let workspace_root = env::current_dir().context("Failed to get current working directory")?;
    let input_path = workspace_root.join("api.json");

    // Use the same pipeline::run function that main.rs uses
    pipeline::run(&input_path)?;

    Ok(())
}
