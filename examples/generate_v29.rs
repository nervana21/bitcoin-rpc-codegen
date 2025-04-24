// examples/generate_v29.rs

use anyhow::Result;
use bitcoin_rpc_codegen::generator::{
    generate_client_mod_rs, generate_types_mod_rs, generate_version_code,
};
use bitcoin_rpc_codegen::parser::{parse_api_json, ApiMethod};
use std::{fs, path::PathBuf};

fn main() -> Result<()> {
    println!("🔧 Generating client + types for v29 from api_v29.json...");

    // Step 1: Parse schema
    let schema = fs::read_to_string("resources/api_v29.json")?;
    let methods: Vec<ApiMethod> = parse_api_json(&schema)?;

    // Step 2: Set output path
    let out_root = PathBuf::from("target/generated/v29");
    let client_dir = out_root.join("client");
    let types_dir = out_root.join("types");

    // Step 3: Run codegen
    if client_dir.exists() {
        fs::remove_dir_all(&client_dir)?;
    }
    if types_dir.exists() {
        fs::remove_dir_all(&types_dir)?;
    }

    fs::create_dir_all(&client_dir)?;
    fs::create_dir_all(&types_dir)?;

    println!("📦 Output: {}", out_root.display());
    generate_version_code("v29", &methods, out_root.to_str().unwrap())?;
    generate_client_mod_rs(&vec!["v29".to_string()], &client_dir)?;
    generate_types_mod_rs(&vec!["v29".to_string()], &types_dir)?;

    println!("✅ Codegen complete.");
    Ok(())
}
