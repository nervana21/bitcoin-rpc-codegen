use std::collections::HashMap;
use std::fs;

use anyhow::Result;

pub mod generator;
pub mod parser;

use generator::code_generator::{generate_client_macro, generate_return_type};
use parser::api_parser::{parse_api_json, ApiMethod};

const SUPPORTED_VERSIONS: &[&str] =
    &["v17", "v18", "v19", "v20", "v21", "v22", "v23", "v24", "v25", "v26", "v27", "v28"];

fn main() -> Result<()> {
    // Read and parse api.json
    let api_json = fs::read_to_string("api.json")?;
    let methods = parse_api_json(&api_json)?;

    // Clean up and create output directories
    if fs::metadata("generated").is_ok() {
        fs::remove_dir_all("generated")?;
    }

    // Generate code for each version
    for version in SUPPORTED_VERSIONS {
        generate_version_code(version, &methods)?;
    }

    println!("Code generation complete. Files saved in generated/ directory.");
    Ok(())
}

fn generate_version_code(version: &str, methods: &[ApiMethod]) -> Result<()> {
    let client_dir = format!("generated/client/src/client_sync/{}", version);
    let types_dir = format!("generated/types/src/{}", version);

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
        let client_path = format!("{}/{}.rs", client_dir, category);
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
        let types_path = format!("{}/{}.rs", types_dir, category);
        fs::write(&types_path, types_code)?;
    }

    // Generate mod.rs files
    generate_mod_files(version, &methods_by_category)?;

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

/// Common trait for mining-related RPC responses
pub trait MiningResponse {
    type GetBlockTemplateResponse: Deserialize<'static> + Serialize;
    type GetMiningInfoResponse: Deserialize<'static> + Serialize;
}

/// Common trait for network-related RPC responses
pub trait NetworkResponse {
    type GetNetworkInfoResponse: Deserialize<'static> + Serialize;
    type GetPeerInfoResponse: Deserialize<'static> + Serialize;
}

/// Common trait for control-related RPC responses
pub trait ControlResponse {
    type GetMemoryInfoResponse: Deserialize<'static> + Serialize;
    type HelpResponse: Deserialize<'static> + Serialize;
    type UptimeResponse: Deserialize<'static> + Serialize;
}

/// Common trait for raw transaction-related RPC responses
pub trait RawTransactionResponse {
    type GetRawTransactionResponse: Deserialize<'static> + Serialize;
    type DecodeRawTransactionResponse: Deserialize<'static> + Serialize;
    type CreateRawTransactionResponse: Deserialize<'static> + Serialize;
}

/// Common trait for utility-related RPC responses
pub trait UtilResponse {
    type ValidateAddressResponse: Deserialize<'static> + Serialize;
    type CreateMultiSigResponse: Deserialize<'static> + Serialize;
    type VerifyMessageResponse: Deserialize<'static> + Serialize;
}

/// Common trait for signer-related RPC responses
pub trait SignerResponse {
    type SignRawTransactionResponse: Deserialize<'static> + Serialize;
    type SignRawTransactionWithKeyResponse: Deserialize<'static> + Serialize;
}
"#;
        fs::write("generated/types/src/traits.rs", traits_content)?;
    }

    // Generate version-specific trait implementations
    generate_trait_implementations(version, &methods_by_category)?;

    Ok(())
}

fn generate_mod_files(
    version: &str,
    methods_by_category: &HashMap<String, Vec<&ApiMethod>>,
) -> Result<()> {
    // Generate client mod.rs
    let mut client_mod = String::new();
    for category in methods_by_category.keys() {
        client_mod.push_str(&format!("pub mod {};\n", category));
    }
    fs::write(format!("generated/client/src/client_sync/{}/mod.rs", version), client_mod)?;

    // Generate types mod.rs
    let mut types_mod = String::new();
    if version == "v17" {
        types_mod.push_str("pub mod traits;\n");
    }
    for category in methods_by_category.keys() {
        types_mod.push_str(&format!("pub mod {};\n", category));
    }
    fs::write(format!("generated/types/src/{}/mod.rs", version), types_mod)?;

    Ok(())
}

fn generate_trait_implementations(
    version: &str,
    methods_by_category: &HashMap<String, Vec<&ApiMethod>>,
) -> Result<()> {
    let mut impl_code = String::new();
    impl_code.push_str(&format!("use crate::traits::*;\n\n"));
    impl_code.push_str(&format!("impl BlockchainResponse for {} {{\n", version));

    // Add implementations for each method in the blockchain category
    if let Some(methods) = methods_by_category.get("blockchain") {
        for method in methods {
            let type_name = format!("{}Response", capitalize(&method.name));
            impl_code.push_str(&format!("    type {} = {};\n", type_name, type_name));
        }
    }
    impl_code.push_str("}\n\n");

    // Add similar implementations for other categories
    for (category, methods) in methods_by_category {
        if category != "blockchain" {
            let trait_name = format!("{}Response", capitalize(&category));
            impl_code.push_str(&format!("impl {} for {} {{\n", trait_name, version));
            for method in methods {
                let type_name = format!("{}Response", capitalize(&method.name));
                impl_code.push_str(&format!("    type {} = {};\n", type_name, type_name));
            }
            impl_code.push_str("}\n\n");
        }
    }

    fs::write(format!("generated/types/src/{}/traits.rs", version), impl_code)?;
    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
