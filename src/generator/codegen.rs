// src/generator/codegen.rs

use anyhow::Result;
use std::collections::HashSet;
use std::fmt::Write as FmtWrite;
use std::{fs, path::Path};

use crate::parser::{parse_api_json, ApiMethod, ApiResult};

pub const SUPPORTED_MAJORS: &[u32] = &[17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];

pub fn map_type_to_rust(type_str: &str) -> String {
    match type_str {
        "string" => "String".to_string(),
        "number" => "f64".to_string(),
        "boolean" => "bool".to_string(),
        "hex" => "String".to_string(),
        "object" => "serde_json::Value".to_string(),
        "object_dynamic" => "serde_json::Value".to_string(),
        _ => "String".to_string(),
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
    let desc = format_doc_comment(description);
    if desc.is_empty() {
        format!("    pub {}: {},\n", field_name, field_type)
    } else {
        format!("{}\n    pub {}: {},\n", desc, field_name, field_type)
    }
}

pub fn sanitize_field_name(name: &str) -> String {
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

/// Generate the client‐side macro for a single RPC method,
/// pulling in *all* of its help‐text from resources/<version>_docs/<method>.txt
pub fn generate_client_macro(method: &ApiMethod, version: &str) -> String {
    let name = &method.name;
    let func_name = sanitize_method_name(name);
    let macro_name = format!("impl_client_{}__{}", version, func_name);

    // 1) Typed params list: `foo: String, bar: bool`
    let params_decl = method
        .arguments
        .iter()
        .map(|arg| format!("{}: {}", arg.names[0], map_type_to_rust(&arg.type_)))
        .collect::<Vec<_>>()
        .join(", ");

    // 2) JSON array: `json!([foo, bar])`
    let params_json = if method.arguments.is_empty() {
        "json!([])".to_string()
    } else {
        let args = method
            .arguments
            .iter()
            .map(|arg| arg.names[0].clone())
            .collect::<Vec<_>>()
            .join(", ");
        format!("json!([{}])", args)
    };

    // 3) Try to load the full help‐text dump: resources/<version>_docs/<method>.txt
    let mut docs = String::new();
    let docs_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join(format!("{}_docs", version))
        .join(format!("{}.txt", name));
    if let Ok(contents) = fs::read_to_string(&docs_path) {
        // skip the signature line and any further leading blank lines:
        for line in contents.lines().skip(1).skip_while(|l| l.trim().is_empty()) {
            docs.push_str("        ///");
            if !line.trim().is_empty() {
                docs.push(' ');
                docs.push_str(line);
            }
            docs.push('\n');
        }
    } else if !method.description.trim().is_empty() {
        // fallback to the short JSON‐schema description
        for line in method.description.lines().filter(|l| !l.trim().is_empty()) {
            docs.push_str("        /// ");
            docs.push_str(line.trim());
            docs.push('\n');
        }
    }

    // Step 4: assemble the macro
    let mut out = String::new();
    writeln!(out, "/// client impl for `{}` RPC ({})", name, version).unwrap();
    writeln!(out, "macro_rules! {} {{", macro_name).unwrap();
    writeln!(out, "    () => {{").unwrap();
    // inject our docs block (no leading empty comment, no trailing blank)
    out.push_str(&docs);
    // now the actual function
    if params_decl.is_empty() {
        writeln!(
            out,
            "        pub fn {}(&self) -> RpcResult<{}Response> {{",
            func_name,
            capitalize(name)
        )
        .unwrap();
    } else {
        writeln!(
            out,
            "        pub fn {}(&self, {}) -> RpcResult<{}Response> {{",
            func_name,
            params_decl,
            capitalize(name)
        )
        .unwrap();
    }
    writeln!(out, "            self.call(\"{}\", {})", name, params_json).unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }};").unwrap();
    writeln!(out, "}}").unwrap();

    out
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
    if !description.trim().is_empty() {
        write!(s, "{}\n", description).unwrap();
    }
    writeln!(
        s,
        "#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]"
    )
    .unwrap();
    writeln!(s, "pub struct {} {{", type_name).unwrap();
    writeln!(s, "{}", fields).unwrap();
    writeln!(s, "}}\n").unwrap();
    s
}

pub fn generate_mod_rs(output_dir: &str, versions: &[&str]) -> std::io::Result<()> {
    let mod_rs = "pub mod client;\npub mod types;\n";
    fs::write(Path::new(output_dir).join("mod.rs"), mod_rs)?;

    let mut client_mod = String::new();
    for v in versions {
        writeln!(client_mod, "pub use self::{}::*;", v).unwrap();
    }
    let client_path = Path::new(output_dir).join("client").join("mod.rs");
    fs::create_dir_all(client_path.parent().unwrap())?;
    fs::write(client_path, client_mod)?;

    let mut types_mod = String::new();
    for v in versions {
        writeln!(types_mod, "pub use self::{}_types::*;", v).unwrap();
    }
    let types_path = Path::new(output_dir).join("types").join("mod.rs");
    fs::create_dir_all(types_path.parent().unwrap())?;
    fs::write(types_path, types_mod)?;

    Ok(())
}

pub fn run_codegen() -> Result<()> {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let api = Path::new(manifest).join("resources").join("api.json");
    if !api.exists() {
        return Err(anyhow::anyhow!("API JSON not found at {:?}", api));
    }
    let data = fs::read_to_string(&api)?;
    let methods = parse_api_json(&data)?;

    let out_dir = std::env::var("OUT_DIR")?;
    if fs::metadata(&out_dir).is_ok() {
        fs::remove_dir_all(&out_dir)?;
    }
    fs::create_dir_all(&out_dir)?;

    let vers: Vec<String> = SUPPORTED_MAJORS.iter().map(|m| format!("v{}", m)).collect();
    let vers_refs: Vec<&str> = vers.iter().map(String::as_str).collect();

    for &v in &vers_refs {
        generate_version_code(v, &methods, &out_dir)?;
    }
    generate_mod_rs(&out_dir, &vers_refs)?;
    println!("Codegen complete. Output in {}", out_dir);
    Ok(())
}

pub fn generate_version_code(version: &str, methods: &[ApiMethod], out_dir: &str) -> Result<()> {
    let root = Path::new(out_dir);
    let client_dir = root.join("client/src").join(version);
    let types_dir = root.join("types/src").join(version);

    for dir in [&client_dir, &types_dir] {
        if dir.exists() {
            fs::remove_dir_all(dir)?;
        }
        fs::create_dir_all(dir)?;
    }

    let imports = "use serde::{Deserialize, Serialize};\n\n";
    let mut client = String::new();
    let mut types = String::from(imports);

    for m in methods {
        client.push_str(&generate_client_macro(m, version));
        client.push_str("\n\n");
        if let Some(tc) = generate_return_type(m) {
            types.push_str(&tc);
            types.push_str("\n\n");
        }
    }

    fs::write(client_dir.join("methods.rs"), client)?;
    fs::write(types_dir.join("types.rs"), types)?;

    // Inline the v17 traits template rather than include_str!
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

pub fn generate_struct_fields(result: &ApiResult) -> String {
    let mut out = String::new();
    let mut seen = HashSet::new();
    for f in &result.inner {
        let name = if f.key_name.is_empty() {
            "result".to_string()
        } else {
            sanitize_field_name(&f.key_name)
        };
        if !seen.insert(name.clone()) {
            continue;
        }
        let ty = get_field_type(f);
        out.push_str(&format_struct_field(&name, &ty, &f.description));
    }
    out
}

pub fn generate_client_stubs(mod_path: &Path, methods: &[ApiMethod]) -> anyhow::Result<()> {
    let mut out = String::new();
    out.push_str("// AUTO-GENERATED by bitcoin-rpc-codegen — do not edit\n\n");
    out.push_str("use serde_json::json;\nuse serde_json::Value;\nuse crate::regtest::RpcResult;\nuse crate::regtest::RpcClient;\n\nimpl RpcClient {\n");

    for m in methods {
        for line in m.description.lines().filter(|l| !l.trim().is_empty()) {
            writeln!(out, "    /// {}", line.trim())?;
        }
        let fn_name = sanitize_method_name(&m.name);
        let mut sigs = Vec::new();
        let mut args = Vec::new();
        for arg in &m.arguments {
            let name = &arg.names[0];
            sigs.push(format!("{}: {}", name, map_type_to_rust(&arg.type_)));
            args.push(format!("json!({})", name));
        }
        let sig = if sigs.is_empty() {
            "".into()
        } else {
            format!(", {}", sigs.join(", "))
        };
        let params = if args.is_empty() {
            "&[]".into()
        } else {
            format!("&[{}]", args.join(", "))
        };

        writeln!(
            out,
            "    pub fn {}(&self{}) -> RpcResult<Value> {{",
            fn_name, sig
        )?;
        writeln!(out, "        self.call_json(\"{}\", {})", m.name, params)?;
        writeln!(out, "    }}\n")?;
    }

    out.push_str("}\n");
    fs::write(mod_path, out)?;
    Ok(())
}
