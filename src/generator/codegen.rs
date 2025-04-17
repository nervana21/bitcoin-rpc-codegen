use anyhow::Result;
use std::{collections::HashMap, fs, path::Path};

use crate::generator::{generate_client_macro, generate_mod_rs, generate_return_type};
use crate::parser::{ApiMethod, parse_api_json};

const SUPPORTED_VERSIONS: &[&str] = &[
    "v17", "v18", "v19", "v20", "v21", "v22", "v23", "v24", "v25", "v26", "v27", "v28",
];

pub fn run_codegen() -> Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let api_path = Path::new(manifest_dir).join("resources").join("api.json");
    println!("run_codegen: Using API JSON file at: {:?}", api_path);

    let api_json = fs::read_to_string(api_path)?;
    let methods = parse_api_json(&api_json)?;

    let out_dir = std::env::var("OUT_DIR")?;
    if fs::metadata(&out_dir).is_ok() {
        fs::remove_dir_all(&out_dir)?;
    }
    fs::create_dir_all(&out_dir)?;

    for version in SUPPORTED_VERSIONS {
        generate_version_code(version, &methods, &out_dir)?;
    }

    generate_mod_rs(&out_dir, SUPPORTED_VERSIONS)?;
    println!(
        "run_codegen: Code generation complete. Files saved in {:?}",
        out_dir
    );
    Ok(())
}

fn generate_version_code(version: &str, methods: &[ApiMethod], out_dir: &str) -> Result<()> {
    let root_dir = Path::new(out_dir);
    let client_dir = root_dir.join("client/src").join(version);
    let types_dir = root_dir.join("types/src").join(version);

    fs::create_dir_all(&client_dir)?;
    fs::create_dir_all(&types_dir)?;

    // Modified type_imports: we now use the fully-qualified paths for both bitcoin and the patched serde_json.
    let type_imports = r#"use bitcoin_rpc_codegen::bitcoin::amount::Amount;
use bitcoin_rpc_codegen::bitcoin::hex::Hex;
use bitcoin_rpc_codegen::bitcoin::time::Time;
use bitcoin_rpc_codegen::serde_json;
use serde::{Deserialize, Serialize};
"#;

    let mut methods_by_category: HashMap<String, Vec<&ApiMethod>> = HashMap::new();
    for method in methods {
        methods_by_category
            .entry(method.category.clone())
            .or_insert_with(Vec::new)
            .push(method);
    }

    for (category, category_methods) in &methods_by_category {
        let mut client_code = String::new();
        for method in category_methods {
            client_code.push_str(&generate_client_macro(method, version));
            client_code.push_str("\n\n");
        }
        let client_path = format!("{}/{}.rs", client_dir.display(), category);
        fs::write(&client_path, client_code)?;

        let mut types_code = String::new();
        types_code.push_str(type_imports);
        for method in category_methods {
            if let Some(type_code) = generate_return_type(method) {
                types_code.push_str(&type_code);
                types_code.push_str("\n\n");
            }
        }
        let types_path = format!("{}/{}.rs", types_dir.display(), category);
        fs::write(types_path, types_code)?;
    }

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
        fs::write(types_dir.join("traits.rs"), traits_content)?;
    }

    Ok(())
}
