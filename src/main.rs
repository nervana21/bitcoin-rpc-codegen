use std::collections::HashMap;
use std::fs;

use anyhow::Result;

pub mod generator;
pub mod node_client;
pub mod parser;

use generator::{generate_client_macro, generate_mod_rs, generate_return_type};
use parser::{ApiMethod, parse_api_json};

const SUPPORTED_VERSIONS: &[&str] = &["v28"];
// &["v17", "v18", "v19", "v20", "v21", "v22", "v23", "v24", "v25", "v26", "v27", "v28"];

fn main() -> Result<()> {
    // Read and parse api.json
    let api_json = fs::read_to_string("api.json")?;
    let methods = parse_api_json(&api_json)?;

    // Clean up and create output directories
    if fs::metadata("src/generated").is_ok() {
        fs::remove_dir_all("src/generated")?;
    }

    // Generate code for each version
    for version in SUPPORTED_VERSIONS {
        generate_version_code(version, &methods)?;
    }

    println!("Code generation complete. Files saved in src/generated/ directory.");
    Ok(())
}

fn generate_version_code(version: &str, methods: &[ApiMethod]) -> Result<()> {
    let root_dir = std::env::current_dir()?.join("src/generated");
    let client_dir = root_dir.join("client/src").join(version);
    let types_dir = root_dir.join("types/src").join(version);

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
        methods_by_category
            .entry(method.category.clone())
            .or_insert_with(Vec::new)
            .push(method);
    }

    // Generate consolidated files for each category
    for (category, category_methods) in &methods_by_category {
        // Generate client file for category
        let mut client_code = String::new();
        for method in category_methods {
            client_code.push_str(&generate_client_macro(method, version));
            client_code.push_str("\n\n");
        }
        let client_path = format!("{}/{}.rs", client_dir.display(), category);
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
        let types_path = format!("{}/{}.rs", types_dir.display(), category);
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

    // Generate mod.rs file in the root generated directory
    generate_mod_rs(root_dir.to_str().unwrap())?;

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
    fs::write(
        format!("generated/client/src/{}/mod.rs", version),
        client_mod,
    )?;

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

    fs::write(
        format!("generated/types/src/{}/traits.rs", version),
        impl_code,
    )?;
    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
#[cfg(test)]
mod tests {
    use crate::node_client::NodeClient;
    use serde_json::Value;

    #[test]
    fn test_getblockchaininfo() {
        // Replace these with your actual regtest node parameters.
        let url = "http://localhost:18443";
        let user = "rpcuser";
        let password = "rpcpassword";

        let client = match NodeClient::new(url, user, password) {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to create NodeClient: {}", e);
                return;
            }
        };

        std::thread::sleep(std::time::Duration::from_secs(1));

        // For an empty parameters list, annotate the slice type.
        let response: Value = match client.call("getblockchaininfo", &[] as &[serde_json::Value]) {
            Ok(response) => response,
            Err(e) => {
                eprintln!("RPC call failed: {}", e);
                return;
            }
        };

        println!("getblockchaininfo response: {:#?}", response);

        let chain = match response.get("chain").and_then(|v| v.as_str()) {
            Some(chain) => chain,
            None => {
                eprintln!("Chain field not found in the response");
                return;
            }
        };

        assert_eq!(chain, "regtest", "Expected chain to be 'regtest'");
    }
}
