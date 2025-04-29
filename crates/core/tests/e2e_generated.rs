// tests/e2e_generated.rs

use anyhow::{Context, Result};
use bitcoin_rpc_codegen::schema::{parse_api_json, ApiArgument, ApiResult};
// use bitcoin_rpc_codegen::v29::client::getnewaddress::getnewaddress;
// use bitcoin_rpc_codegen::client::RegtestClient;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;

fn dummy_params(args: &[ApiArgument], best_block: &str, dummy_txid: &str) -> Vec<Value> {
    args.iter()
        .map(|arg| {
            let name = arg.names[0].to_lowercase();
            if name.contains("address") {
                json!("127.0.0.1:8333")
            } else if name.contains("connection_type") {
                json!("outbound-full-relay")
            } else if name.contains("transport") {
                json!(false)
            } else if name.contains("blockhash") {
                json!(best_block)
            } else if name.contains("txid") {
                json!(dummy_txid)
            } else {
                match arg.type_.as_str() {
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

fn validate_result_shape(expected: &ApiResult, value: &Value, path: &str) -> Vec<String> {
    let mut errors = Vec::new();
    match (expected.type_.as_str(), value) {
        ("string" | "hex", Value::String(_))
        | ("boolean", Value::Bool(_))
        | ("number", Value::Number(_))
        | ("none", Value::Null) => {}
        ("string", Value::Number(_)) | ("number", Value::String(_)) => {}
        ("object", Value::Object(map)) => {
            if expected.inner.is_empty() {
            } else {
                let schema_fields: BTreeMap<_, _> = expected
                    .inner
                    .iter()
                    .map(|f| (f.key_name.clone(), f))
                    .collect();
                for (key, subschema) in schema_fields {
                    if let Some(val) = map.get(&key) {
                        errors.extend(validate_result_shape(
                            subschema,
                            val,
                            &format!("{}.{}", path, key),
                        ));
                    }
                }
            }
        }
        ("array", Value::Array(arr)) => {
            if let Some(item_schema) = expected.inner.first() {
                for (i, item) in arr.iter().enumerate() {
                    errors.extend(validate_result_shape(
                        item_schema,
                        item,
                        &format!("{}[{}]", path, i),
                    ));
                }
            }
        }
        (expected_type, actual_value) => {
            errors.push(format!(
                "{}: expected `{}`, got `{}`",
                path,
                expected_type,
                match actual_value {
                    Value::Null => "null",
                    Value::Bool(_) => "boolean",
                    Value::Number(_) => "number",
                    Value::String(_) => "string",
                    Value::Array(_) => "array",
                    Value::Object(_) => "object",
                }
            ));
        }
    }
    errors
}

// #[test]
// fn e2e_generated() -> Result<()> {
//     let mut rt = RegtestClient::new_from_path(
//         "e2e_test",
//         "/Users/bitnode/bitcoin-versions/v29/bitcoin-29.0/bin/bitcoind",
//     )?;

//     let new_addr: String = rt
//         .call_json("getnewaddress", &[])?
//         .as_str()
//         .context("getnewaddress did not return a string")?
//         .to_string();
//     let _: Vec<String> = rt
//         .call_json("generatetoaddress", &[json!(101), json!(new_addr.clone())])?
//         .as_array()
//         .context("generatetoaddress did not return an array")?
//         .iter()
//         .map(|v| v.as_str().unwrap_or_default().to_string())
//         .collect();
//     let best_block: String = rt
//         .call_json("getbestblockhash", &[])?
//         .as_str()
//         .context("getbestblockhash did not return a string")?
//         .to_string();
//     let dummy_txid: String = rt
//         .call_json("sendtoaddress", &[json!(new_addr), json!(0.0001)])?
//         .as_str()
//         .context("sendtoaddress did not return a string")?
//         .to_string();

//     let schema_path = "resources/api_v29.json";
//     let schema_src = fs::read_to_string(schema_path)
//         .with_context(|| format!("Failed to read schema at `{}`", schema_path))?;
//     let methods = parse_api_json(&schema_src)
//         .with_context(|| format!("Failed to parse API schema at `{}`", schema_path))?;

//     let mut total = 0;
//     let mut success = 0;
//     let mut mismatch = 0;
//     let mut skipped = 0;

//     for method in methods.iter().filter(|m| m.name != "stop") {
//         total += 1;
//         println!("🔎 Testing method: `{}`", method.name);

//         let params = if method.arguments.is_empty() {
//             vec![]
//         } else {
//             dummy_params(&method.arguments, &best_block, &dummy_txid)
//         };

//         match rt.call_json(&method.name, &params) {
//             Ok(resp) => {
//                 let schema = &method.results[0];
//                 let mismatches = validate_result_shape(schema, &resp, &method.name);
//                 if mismatches.is_empty() {
//                     println!("✅ `{}` passed shape validation.", method.name);
//                     success += 1;
//                 } else {
//                     println!(
//                         "⚠️ Type mismatches for `{}`:\n{}",
//                         method.name,
//                         mismatches.join("\n")
//                     );
//                     mismatch += 1;
//                 }
//             }
//             Err(e) => {
//                 println!("⚠️ Skipping `{}` due to RPC error: {}", method.name, e);
//                 skipped += 1;
//             }
//         }
//     }

//     println!("🛑 Stopping node...");
//     rt.call_json("stop", &[])?;
//     rt.teardown()?;
//     println!("✅ Node shutdown verified.");

//     println!();
//     println!("🏁 Test Summary:");
//     println!("  Total methods tested: {}", total);
//     println!("  Passed validation:    {}", success);
//     println!("  Type mismatches:       {}", mismatch);
//     println!("  Skipped (RPC errors):   {}", skipped);

//     let mismatch_threshold = 20;
//     if mismatch > mismatch_threshold {
//         panic!(
//             "❌ Too many schema mismatches: {} (threshold: {})",
//             mismatch, mismatch_threshold
//         );
//     }

//     Ok(())
// }
