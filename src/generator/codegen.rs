use anyhow::Result;
use std::collections::HashSet;
use std::fmt::Write;
use std::{fs, path::Path};

use crate::parser::{parse_api_json, ApiMethod, ApiResult};

pub const SUPPORTED_VERSIONS: &[&str] = &[
    "v17", "v18", "v19", "v20", "v21", "v22", "v23", "v24", "v25", "v26", "v27", "v28",
];

// Add the missing functions that are referenced from mod.rs
pub fn map_type_to_rust(type_str: &str) -> String {
    match type_str {
        "string" => "String".to_string(),
        "number" => "f64".to_string(),
        "boolean" => "bool".to_string(),
        "hex" => "String".to_string(),
        "object" => "serde_json::Value".to_string(),
        "object_dynamic" => "serde_json::Value".to_string(),
        _ => {
            // Fallback unknown or custom types (e.g., Network, Mine) to String
            "String".to_string()
        }
    }
}

pub fn sanitize_method_name(name: &str) -> String {
    name.replace("-", "_").to_lowercase()
}

pub fn capitalize(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

// Implement the functions that were previously imported from mod.rs
pub fn format_doc_comment(description: &str) -> String {
    description
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| format!("/// {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn format_struct_field(field_name: &str, field_type: &str, description: &str) -> String {
    let formatted_description = format_doc_comment(description);
    format!(
        "{}\n    pub {}: {},\n",
        formatted_description, field_name, field_type
    )
}

pub fn sanitize_field_name(name: &str) -> String {
    // Filter to alphanumeric or underscore, lowercase letters.
    let filtered: String = name
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c == '_' {
                Some(c)
            } else {
                None
            }
        })
        .collect();

    // If the first character is a digit, prefix with underscore.
    if filtered
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        format!("_{}", filtered)
    } else {
        filtered
    }
}

pub fn get_field_type(field: &ApiResult) -> String {
    match field.type_.as_str() {
        "array" | "array-fixed" => "Vec<serde_json::Value>".to_string(),
        "object" if !field.inner.is_empty() => "serde_json::Value".to_string(),
        _ => map_type_to_rust(&field.type_),
    }
}

pub fn generate_client_macro(method: &ApiMethod, version: &str) -> String {
    let method_name = sanitize_method_name(&method.name);
    let macro_name = format!("impl_client_{}__{}", version, method_name);
    let description = format_doc_comment(&method.description);
    let params = method
        .arguments
        .iter()
        .map(|arg| format!("{}: {}", arg.names[0], map_type_to_rust(&arg.type_)))
        .collect::<Vec<_>>()
        .join(", ");

    let mut func = String::new();
    writeln!(func, "/// {}", description).unwrap();
    writeln!(func, "macro_rules! {} {{", macro_name).unwrap();
    writeln!(func, "    () => {{").unwrap();
    writeln!(func, "        pub fn {}(\n            &self,\n            {params}\n        ) -> RpcResult<{}Response> {{", method_name, capitalize(&method.name)).unwrap();
    writeln!(
        func,
        "            self.call(\"{}\", json!([{}]))\n        }}",
        method.name, params
    )
    .unwrap();
    writeln!(func, "    }};\n}}").unwrap();
    func
}

pub fn generate_return_type(method: &ApiMethod) -> Option<String> {
    if method.results.is_empty() {
        return None;
    }
    let result = &method.results[0];
    if result.type_.eq_ignore_ascii_case("none") {
        return None;
    }
    let type_name = format!("{}Response", capitalize(&method.name));
    let formatted_description = format_doc_comment(&method.description);
    let fields = if result.inner.is_empty() {
        format!("    pub result: {},", get_return_type(result))
    } else {
        generate_struct_fields(result)
    };
    Some(generate_struct(&type_name, &formatted_description, &fields))
}

fn get_return_type(result: &ApiResult) -> String {
    if result.type_.eq_ignore_ascii_case("object") && !result.inner.is_empty() {
        "serde_json::Value".to_string()
    } else {
        map_type_to_rust(&result.type_)
    }
}

fn generate_struct(type_name: &str, description: &str, fields: &str) -> String {
    let mut s = String::new();
    writeln!(s, "/// Response for the {} RPC call.", type_name).unwrap();
    writeln!(s, "{}", description).unwrap();
    writeln!(
        s,
        "#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]"
    )
    .unwrap();
    writeln!(s, "pub struct {} {{", type_name).unwrap();
    writeln!(s, "{}", fields).unwrap();
    writeln!(s, "}}").unwrap();
    s
}

pub fn generate_mod_rs(output_dir: &str, versions: &[&str]) -> std::io::Result<()> {
    use std::fmt::Write; // Import the Write trait for writeln!
    let mod_rs_content = "pub mod client;\npub mod types;\n";
    fs::write(Path::new(output_dir).join("mod.rs"), mod_rs_content)?;

    // Client mod.rs
    let mut client_mod_rs = String::new();
    for version in versions {
        writeln!(client_mod_rs, "pub use self::{}::*;", version)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }
    let client_mod_path = Path::new(output_dir).join("client").join("mod.rs");
    fs::create_dir_all(client_mod_path.parent().unwrap())?;
    fs::write(client_mod_path, client_mod_rs)?;

    // Types mod.rs
    let mut types_mod_rs = String::new();
    for version in versions {
        writeln!(types_mod_rs, "pub use self::{}_types::*;", version)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }
    let types_mod_path = Path::new(output_dir).join("types").join("mod.rs");
    fs::create_dir_all(types_mod_path.parent().unwrap())?;
    fs::write(types_mod_path, types_mod_rs)?;

    Ok(())
}

pub fn run_codegen() -> Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let api_path = Path::new(manifest_dir).join("resources").join("api.json");
    if !api_path.exists() {
        return Err(anyhow::anyhow!(
            "API JSON file not found at: {:?}",
            api_path
        ));
    }
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

pub fn generate_version_code(version: &str, methods: &[ApiMethod], out_dir: &str) -> Result<()> {
    let root_dir = Path::new(out_dir);
    let client_dir = root_dir.join("client/src").join(version);
    let types_dir = root_dir.join("types/src").join(version);

    for dir in &[&client_dir, &types_dir] {
        if dir.exists() {
            fs::remove_dir_all(dir)?;
        }
        fs::create_dir_all(dir)?;
    }

    // Modified type_imports: only include necessary imports
    let type_imports = r#"use serde::{Deserialize, Serialize};
"#;

    let mut client_code = String::new();
    let mut types_code = String::from(type_imports);

    for method in methods {
        client_code.push_str(&generate_client_macro(method, version));
        client_code.push_str("\n\n");

        if let Some(type_code) = generate_return_type(method) {
            types_code.push_str(&type_code);
            types_code.push_str("\n\n");
        }
    }

    fs::write(client_dir.join("methods.rs"), client_code)?;
    fs::write(types_dir.join("types.rs"), types_code)?;

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

/// Generate field definitions for complex types, deduping by key name.
pub fn generate_struct_fields(result: &ApiResult) -> String {
    let mut fields = String::new();
    let mut seen = HashSet::new();
    for field in &result.inner {
        let field_name = if field.key_name.is_empty() {
            "result".to_string()
        } else {
            sanitize_field_name(&field.key_name)
        };
        if !seen.insert(field_name.clone()) {
            continue;
        }
        let field_type = get_field_type(field);
        fields.push_str(&format_struct_field(
            &field_name,
            &field_type,
            &field.description,
        ));
    }
    fields
}
