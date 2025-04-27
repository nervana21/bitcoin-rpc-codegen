use anyhow::{Context, Result};
use bitcoin_rpc_codegen::parser::{ApiArgument, ApiMethod, ApiResult};
use bitcoin_rpc_codegen::RegtestClient;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

/// Return safe minimalist dummy parameters based on name and type heuristics.
fn dummy_params(args: &[ApiArgument]) -> Vec<Value> {
    args.iter()
        .map(|arg| {
            let name = arg.names[0].to_lowercase();
            let type_ = arg.type_.to_lowercase();

            if name.contains("blockhash") || name.contains("txid") {
                json!("00".repeat(64)) // 32-byte hex string (safe dummy txid/blockhash)
            } else if name.contains("address") {
                json!("") // Safer: empty string, not invalid bech32 garbage
            } else if name.contains("amount") || name.contains("fee") || name.contains("value") {
                json!(0.001) // Tiny plausible amount
            } else if name.contains("height")
                || name.contains("index")
                || name.contains("conf_target")
                || name.contains("timeout")
            {
                json!(1) // Small integer
            } else if name.contains("passphrase") || name.contains("password") {
                json!("passphrase") // Plausible password value
            } else if name.contains("wallet")
                || name.contains("filename")
                || name.contains("filepath")
            {
                json!("dummy_wallet") // Plausible wallet or file name
            } else if name.contains("descriptor") {
                json!("wpkh([abcd1234/84h/0h/0h]xpub6C.../0/*)") // Good descriptor string for APIs needing it
            } else if name.contains("flag") {
                json!("avoid_reuse") // Common known wallet flag
            } else if name.contains("outputs")
                || name.contains("recipients")
                || name.contains("addresses")
            {
                json!([]) // Empty arrays are safe for lists
            } else {
                // Fallback purely by type
                match type_.as_str() {
                    "string" | "hex" => json!(""),
                    "number" => json!(0),
                    "boolean" => json!(false),
                    "array" => json!([]),
                    "object" => json!({}),
                    _ => Value::Null,
                }
            }
        })
        .collect()
}

/// Infer the basic type from a JSON value.
fn infer_type(value: &Value) -> String {
    match value {
        Value::String(_) => "string".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Array(_) => "array".to_string(),
        Value::Object(_) => "object".to_string(),
        Value::Null => "none".to_string(),
    }
}

/// Very basic introspector to flatten and infer first result type.
fn infer_result_schema(value: &Value) -> ApiResult {
    let type_ = infer_type(value);
    let inner = match value {
        Value::Object(map) => map
            .iter()
            .map(|(k, v)| ApiResult {
                key_name: k.clone(),
                type_: infer_type(v),
                description: String::new(),
                inner: vec![],
            })
            .collect(),
        Value::Array(arr) => arr
            .get(0)
            .map(|v| {
                vec![ApiResult {
                    key_name: "".to_string(),
                    type_: infer_type(v),
                    description: String::new(),
                    inner: vec![],
                }]
            })
            .unwrap_or_default(),
        _ => vec![],
    };

    ApiResult {
        type_,
        description: String::new(),
        key_name: String::new(),
        inner,
    }
}

fn main() -> Result<()> {
    let mut rt = RegtestClient::new_auto("discover")?;
    let client = &rt.client;

    println!("🔍 Discovering real RPC schema...");

    let schema_path = Path::new("resources/schemas/api_v29.json");
    let schema_src = fs::read_to_string(schema_path)
        .with_context(|| format!("Failed to read `{}`", schema_path.display()))?;
    let methods = bitcoin_rpc_codegen::parser::parse_api_json(&schema_src)?;

    let mut discovered = Vec::new();
    let mut successes = Vec::new();
    let mut failures = Vec::new();

    for method in methods.iter().filter(|m| m.name != "stop") {
        let params = dummy_params(&method.arguments);

        println!("📡 Calling `{}`...", method.name);

        match client.call_json(&method.name, &params) {
            Ok(resp) => {
                successes.push(method.name.clone());
                let result_schema = infer_result_schema(&resp);

                discovered.push(ApiMethod {
                    name: method.name.clone(),
                    arguments: method.arguments.clone(), // Reuse original args
                    results: vec![result_schema],
                    description: format!("Discovered dynamically at {}", chrono::Utc::now()),
                });
            }
            Err(err) => {
                failures.push(method.name.clone());
                println!("⚠️ Skipping `{}` due to RPC error: {}", method.name, err);
            }
        }
    }

    println!();
    println!("✅ Discovery complete. Writing output...");

    let output_path = "real_api_v29.json";
    let json = serde_json::to_string_pretty(&serde_json::json!({ "commands": discovered }))?;
    fs::write(output_path, json).with_context(|| format!("Failed to write `{}`", output_path))?;

    println!("📚 Output written to `{}`!", output_path);
    println!();

    println!("📊 Summary:");
    println!("✅ Successful calls: {}/{}", successes.len(), methods.len());
    println!("⚠️ Failed calls: {}/{}", failures.len(), methods.len());

    if !failures.is_empty() {
        println!("\n⚠️ Failed methods:");
        for method_name in &failures {
            println!("- {}", method_name);
        }
    }

    rt.teardown()?;
    Ok(())
}
