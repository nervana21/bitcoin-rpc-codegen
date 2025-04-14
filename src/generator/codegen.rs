use anyhow::Result;
use std::{collections::HashMap, fs, path::Path};

use crate::generator::{generate_client_macro, generate_mod_rs, generate_return_type};
use crate::parser::{ApiMethod, parse_api_json};

const SUPPORTED_VERSIONS: &[&str] = &[
    "v17", "v18", "v19", "v20", "v21", "v22", "v23", "v24", "v25", "v26", "v27", "v28",
];

/// Reads the API JSON file from the resources folder, parses it,
/// and generates code into the OUT_DIR (in a subdirectory named "generated").
pub fn run_codegen() -> Result<()> {
    // 1. Locate api.json in the resources folder (relative to the crate root).
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let api_path = Path::new(manifest_dir).join("resources").join("api.json");
    println!("run_codegen: Using API JSON file at: {:?}", api_path);

    let api_json = fs::read_to_string(api_path)?;
    let methods = parse_api_json(&api_json)?;

    // 2. Use OUT_DIR for generated code.
    let out_dir = std::env::var("OUT_DIR")?;
    let generated_path = Path::new(&out_dir);

    if fs::metadata(&generated_path).is_ok() {
        fs::remove_dir_all(&generated_path)?;
    }
    fs::create_dir_all(&generated_path)?;

    // 3. For each supported version, generate the files.
    for version in SUPPORTED_VERSIONS {
        generate_version_code(version, &methods, &generated_path)?;
    }

    // 4. Generate the root mod.rs for the generated folder.
    generate_mod_rs(generated_path.to_str().unwrap(), SUPPORTED_VERSIONS)?;

    println!(
        "run_codegen: Code generation complete. Files saved in {:?}",
        generated_path
    );
    Ok(())
}

/// Generates code for a specific version into the provided generated path.
fn generate_version_code(
    version: &str,
    methods: &[ApiMethod],
    generated_path: &Path,
) -> Result<()> {
    // Use the generated_path (from OUT_DIR) as our root.
    let root_dir = generated_path.to_path_buf();
    let client_dir = root_dir.join("client/src").join(version);
    let types_dir = root_dir.join("types/src").join(version);

    fs::create_dir_all(&client_dir)?;
    fs::create_dir_all(&types_dir)?;

    // Common imports for generated type files.
    let type_imports = r#"use bitcoin::{amount::Amount, hex::Hex, time::Time};
use serde::{Deserialize, Serialize};
use serde_json;

"#;

    // Group API methods by category.
    let mut methods_by_category: HashMap<String, Vec<&ApiMethod>> = HashMap::new();
    for method in methods {
        methods_by_category
            .entry(method.category.clone())
            .or_insert_with(Vec::new)
            .push(method);
    }

    // Generate client and types files for each category.
    for (category, category_methods) in &methods_by_category {
        // Generate client file.
        let mut client_code = String::new();
        for method in category_methods {
            client_code.push_str(&generate_client_macro(method, version));
            client_code.push_str("\n\n");
        }
        let client_path = format!("{}/{}.rs", client_dir.display(), category);
        fs::write(&client_path, client_code)?;

        // Generate types file.
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

    // For version "v17", generate a traits file.
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
        fs::write(generated_path.join("types/src/traits.rs"), traits_content)?;
    }

    Ok(())
}
