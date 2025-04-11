use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

pub mod generator;
pub mod parser;

use generator::code_generator::{generate_client_macro, generate_return_type};
use parser::api_parser::{parse_api_json, ApiMethod};

const SUPPORTED_VERSIONS: &[&str] =
    &["v17", "v18", "v19", "v20", "v21", "v22", "v23", "v24", "v25", "v26", "v27", "v28", "v29"];

/// Bitcoin RPC Code Generator
///
/// A tool for automatically generating version-specific Bitcoin RPC client code
/// for different Bitcoin Core versions.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the api.json file
    #[arg(short, long, default_value = "api.json")]
    api_file: PathBuf,

    /// Path to the output directory
    #[arg(short, long, default_value = "generated")]
    output_dir: PathBuf,

    /// Bitcoin Core versions to generate code for
    #[arg(short, long, default_values = SUPPORTED_VERSIONS)]
    versions: Vec<String>,

    /// Whether to clean the output directory before generating code
    #[arg(short, long, default_value = "true")]
    clean: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read and parse api.json
    let api_json = fs::read_to_string(&args.api_file)?;
    let methods = parse_api_json(&api_json)?;

    // Clean up and create output directories
    if args.clean && fs::metadata(&args.output_dir).is_ok() {
        fs::remove_dir_all(&args.output_dir)?;
    }

    // Generate code for each version
    for version in &args.versions {
        if !SUPPORTED_VERSIONS.contains(&version.as_str()) {
            println!(
                "Warning: Version {} is not in the list of supported versions. Skipping.",
                version
            );
            continue;
        }
        generate_version_code(version, &methods, &args.output_dir)?;
    }

    println!("Code generation complete. Files saved in {} directory.", args.output_dir.display());
    Ok(())
}

fn generate_version_code(version: &str, methods: &[ApiMethod], output_dir: &PathBuf) -> Result<()> {
    let client_dir = output_dir.join(format!("client/src/client_sync/{}", version));
    let types_dir = output_dir.join(format!("types/src/{}", version));

    fs::create_dir_all(&client_dir)?;
    fs::create_dir_all(&types_dir)?;

    // Common imports for type files
    let type_imports = r#"use bitcoin::{amount::Amount, hex::Hex, time::Time};
use serde::{Deserialize, Serialize};
use serde_json;

"#;

    // Group methods by category
    let mut methods_by_category: HashMap<String, Vec<&ApiMethod>> = HashMap::new();
    for method in methods {
        methods_by_category.entry(method.category.clone()).or_insert_with(Vec::new).push(method);
    }

    // Generate consolidated files for each category
    for (category, category_methods) in &methods_by_category {
        // Generate client file for category
        let mut client_code = String::new();
        for method in category_methods {
            client_code.push_str(&generate_client_macro(method, version));
            client_code.push_str("\n\n");
        }
        let client_path = client_dir.join(format!("{}.rs", category));
        fs::write(&client_path, client_code)?;

        // Generate types file for category
        let mut types_code = String::new();
        types_code.push_str(type_imports);
        for method in category_methods {
            if let Some(type_code) = generate_return_type(method) {
                types_code.push_str(&type_code);
                types_code.push_str("\n\n");
            }
        }
        let types_path = types_dir.join(format!("{}.rs", category));
        fs::write(&types_path, types_code)?;
    }

    // Generate mod.rs files
    generate_mod_files(version, &methods_by_category, &client_dir, &types_dir)?;

    // Generate traits.rs file only in the main types directory (only for v17)
    if version == "v17" {
        let traits_content = r#"// SPDX-License-Identifier: CC0-1.0

//! Common traits for Bitcoin RPC operations across different versions.
//! These traits provide a unified interface for working with different Bitcoin Core versions.

use serde::{Deserialize, Serialize};

/// Common trait for blockchain-related RPC responses
pub trait BlockchainResponse {
    type GetBlockResponse: Deserialize<'static> + Serialize;
    type GetBlockHashResponse: Deserialize<'static> + Serialize;
    type GetBlockchainInfoResponse: Deserialize<'static> + Serialize;
}

/// Common trait for wallet-related RPC responses
pub trait WalletResponse {
    type GetBalanceResponse: Deserialize<'static> + Serialize;
    type GetTransactionResponse: Deserialize<'static> + Serialize;
}
"#;
        let traits_path = output_dir.join("types/src/traits.rs");
        fs::write(&traits_path, traits_content)?;
    }

    Ok(())
}

fn generate_mod_files(
    version: &str,
    methods_by_category: &HashMap<String, Vec<&ApiMethod>>,
    client_dir: &PathBuf,
    types_dir: &PathBuf,
) -> Result<()> {
    // Generate client mod.rs
    let mut client_mod = String::new();
    client_mod.push_str("// SPDX-License-Identifier: CC0-1.0\n\n");
    client_mod.push_str(&format!("//! Bitcoin Core v{} RPC client implementation\n\n", version));

    for category in methods_by_category.keys() {
        client_mod.push_str(&format!("pub mod {};\n", category));
    }

    let client_mod_path = client_dir.join("mod.rs");
    fs::write(&client_mod_path, client_mod)?;

    // Generate types mod.rs
    let mut types_mod = String::new();
    types_mod.push_str("// SPDX-License-Identifier: CC0-1.0\n\n");
    types_mod.push_str(&format!("//! Bitcoin Core v{} RPC type definitions\n\n", version));

    for category in methods_by_category.keys() {
        types_mod.push_str(&format!("pub mod {};\n", category));
    }

    let types_mod_path = types_dir.join("mod.rs");
    fs::write(&types_mod_path, types_mod)?;

    Ok(())
}

fn generate_trait_implementations(
    version: &str,
    methods_by_category: &HashMap<String, Vec<&ApiMethod>>,
    output_dir: &PathBuf,
) -> Result<()> {
    // This function would generate trait implementations for the types
    // For now, it's a placeholder
    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(c).collect(),
    }
}
