// examples/generate_v29.rs

use anyhow::Result;
use bitcoin_rpc_codegen::generator::{generate_mod_rs, generate_version_code};
use bitcoin_rpc_codegen::parser::parse_api_json;
use std::{fs, path::PathBuf};

fn main() -> Result<()> {
    println!("🔧 Generating client + types for v29 from resources/api_v29.json...");

    // 1. Read and parse the v29 schema
    let schema_src = fs::read_to_string("resources/api_v29.json")?;
    let methods = parse_api_json(&schema_src)?;

    // 2. Prepare a clean output directory
    let out_root = PathBuf::from("target/generated/v29");
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;

    println!("📦 Writing code to {}", out_root.display());

    // 3. Emit per‐method code under client/src/v29 and types/src/v29
    generate_version_code("v29", &methods, out_root.to_str().unwrap())?;

    // 4. Wire up the `mod.rs` files so you can `pub use v29::*;`
    generate_mod_rs(out_root.to_str().unwrap(), &["v29"])?;

    println!("✅ Codegen complete!");
    Ok(())
}
