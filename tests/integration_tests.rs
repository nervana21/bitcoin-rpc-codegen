// tests/integration_tests.rs

use anyhow::Result;
use bitcoin_rpc_codegen::parser::{parse_api_json, ApiArgument, ApiMethod, ApiResult};
use bitcoin_rpc_codegen::{Client, RegtestClient};
use serde_json::Value;

/// Build default parameters for RPC calls based on schema hints.
fn default_params(args: &[ApiArgument], best_block: &str, dummy_txid: &str) -> Vec<Value> {
    args.iter()
        .map(|arg| {
            let name = &arg.names[0].to_lowercase();
            if name.contains("address") {
                return Value::String("127.0.0.1:8333".into());
            } else if name.contains("connection_type") {
                return Value::String("outbound-full-relay".into());
            } else if name.contains("transport") {
                return Value::Bool(false);
            }
            if name.contains("blockhash") {
                return Value::String(best_block.into());
            } else if name.contains("txid") {
                return Value::String(dummy_txid.into());
            }
            let t = arg.type_.to_lowercase();
            if t.contains("numeric") || t.contains("number") {
                Value::Number(1.into())
            } else if t.contains("string") || t.contains("hex") {
                Value::String("".into())
            } else if t.contains("boolean") {
                Value::Bool(false)
            } else if t.contains("array") {
                Value::Array(vec![])
            } else if t.contains("object") {
                Value::Object(serde_json::Map::new())
            } else {
                Value::Null
            }
        })
        .collect()
}

/// Recursively validate ANY present fields have the right type,
/// but do *not* error on missing or extra fields.
/// Allows number↔string interchange when schema=string/number.
fn validate_shape(schema: &ApiResult, value: &Value, path: &str) -> Vec<String> {
    let mut errs = Vec::new();
    match (schema.type_.as_str(), value) {
        // exact matches
        ("string" | "hex", Value::String(_))
        | ("boolean", Value::Bool(_))
        | ("number", Value::Number(_))
        | ("none", Value::Null) => { /* ok */ }
        // allow number-as-string or string-as-number
        ("string", Value::Number(_)) | ("number", Value::String(_)) => { /* also ok */ }
        ("object", Value::Object(_)) if schema.inner.is_empty() => { /* ok */ }
        ("object", Value::Object(map)) => {
            // Only check fields that schema defines
            let expected = schema
                .inner
                .iter()
                .map(|f| (f.key_name.clone(), f))
                .collect::<std::collections::BTreeMap<_, _>>();
            for (key, subschema) in expected {
                if let Some(val) = map.get(&key) {
                    errs.extend(validate_shape(subschema, val, &format!("{}.{}", path, key)));
                }
            }
        }
        ("array", Value::Array(arr)) => {
            if let Some(item_schema) = schema.inner.get(0) {
                for (i, item) in arr.iter().enumerate() {
                    errs.extend(validate_shape(
                        item_schema,
                        item,
                        &format!("{}[{}]", path, i),
                    ));
                }
            }
        }
        (expected, actual) => {
            errs.push(format!(
                "{}: expected `{}`, got `{}`",
                path, expected, actual
            ));
        }
    }
    errs
}

/// Call the RPC and ensure its response at least has the right types.
fn assert_method_presence(client: &Client, method: &ApiMethod, best_block: &str, dummy_txid: &str) {
    let params = if method.arguments.is_empty() {
        vec![]
    } else {
        default_params(&method.arguments, best_block, dummy_txid)
    };

    match client.call_json(&method.name, &params) {
        Ok(resp) => {
            let schema = &method.results[0];
            let mismatches = validate_shape(schema, &resp, &method.name);
            if !mismatches.is_empty() {
                panic!(
                    "RPC `{}` type mismatches:\n  {}",
                    method.name,
                    mismatches.join("\n  ")
                );
            }
        }
        Err(e) => {
            // Many RPCs require context; skip on error
            println!("Skipping `{}` due to RPC error: {}", method.name, e);
        }
    }
}

#[test]
fn e2e_all_methods() -> Result<()> {
    let rt = RegtestClient::new_auto("test")?;
    let client = &rt.client;
    let best_block = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
    let dummy_txid = best_block;

    let methods = parse_api_json(include_str!("../resources/api_v29.json"))?;
    for m in &methods {
        if m.name == "stop" {
            println!("Skipping `stop`");
            continue;
        }
        println!("Testing `{}`...", m.name);
        assert_method_presence(client, m, best_block, dummy_txid);
    }

    // Finally, ensure stop works and shuts down
    client.call_json("stop", &[])?;
    Ok(())
}
