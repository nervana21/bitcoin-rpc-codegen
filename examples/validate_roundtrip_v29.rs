// examples/validate_roundtrip_v29.rs

use anyhow::{Context, Result};
use bitcoin_rpc_codegen::parser::{parse_api_json, ApiMethod, ApiResult};
use bitcoin_rpc_codegen::{Conf, RegtestClient};
use serde_json::Value;
use std::{collections::HashSet, fs};

fn main() -> Result<()> {
    // 1. Load the v29 schema
    let schema_src = fs::read_to_string("resources/api_v29.json")
        .context("Failed to read resources/api_v29.json")?;
    let methods: Vec<ApiMethod> =
        parse_api_json(&schema_src).context("Failed to parse api_v29.json")?;

    // 2. Extract zero-arg names from schema
    let mut zero_arg: Vec<_> = methods
        .iter()
        .filter(|m| m.arguments.is_empty())
        .map(|m| m.name.clone())
        .collect();

    // 3. Sort them (help output is already alphabetical, but explicit is clearer):
    zero_arg.sort();

    // 4. Pull out "stop" (we know it must exist) and push to the end:
    if let Some(pos) = zero_arg.iter().position(|n| n == "stop") {
        let stop = zero_arg.remove(pos);
        zero_arg.push(stop);
    } else {
        panic!("Schema did not contain a zero-arg `stop` RPC");
    }

    eprintln!("\n🔍 Zero-arg RPCs (alphabetical, with `stop` last):");
    for (i, name) in zero_arg.iter().enumerate() {
        eprintln!("  {:>2}. {}", i + 1, name);
    }

    // 5. Spin up your regtest node
    let mut conf = Conf::default();
    conf.extra_args.push("-fallbackfee=0.0002");
    let rt = RegtestClient::new_with_conf(&conf).context("Failed to start regtest node")?;
    let client = &rt.client;

    println!("\nLoaded {} RPC methods from schema", methods.len());

    // 6. Call every zero-arg *except* the last (`stop`), skipping on RPC error
    for name in &zero_arg[..zero_arg.len() - 1] {
        println!("\nCalling `{}`...", name);
        let v: Value = match client.call_json(name, &[]) {
            Ok(v) => {
                // dump for later schema regeneration
                fs::create_dir_all("feedback")?;
                fs::write(
                    format!("feedback/{}.json", name),
                    serde_json::to_string_pretty(&v)?,
                )?;
                println!(
                    "   → got {} bytes (dumped to feedback/{})",
                    v.to_string().len(),
                    name
                );
                v
            }
            Err(err) => {
                println!("  ⎯ Skipping `{}`: RPC error: {}", name, err);
                continue;
            }
        };

        // 7. Validate shape against schema
        let method = methods.iter().find(|m| &m.name == name).unwrap();
        let mut errors = Vec::new();
        compare_value(&v, &method.results, &mut Vec::new(), &mut errors);

        if errors.is_empty() {
            println!("  ✔ `{}` matches schema", name);
        } else {
            println!("  ✖ `{}` mismatches:", name);
            for e in errors {
                println!("    - {}", e);
            }
        }
    }

    // 8. Finally call `stop`
    println!("\nCalling `stop` (last)...");
    let stop_response: Value = client.call_json("stop", &[]).context("stop RPC failed")?;
    println!(
        "   → stop RPC returned {} bytes",
        stop_response.to_string().len()
    );

    // 9. Prove the node has shut down
    println!("\n🔒 Verifying shutdown: next RPC must fail.");
    match client.call_json("getblockcount", &[]) {
        Ok(_) => panic!("Expected RPC to fail after stop, but it succeeded"),
        Err(err) => println!("   → getblockcount failed as expected: {}", err),
    }

    Ok(())
}

/// Recursively compare a JSON value against an ApiResult schema, ignoring extra fields
/// and allowing number⟷string mismatches.
fn compare_value(
    val: &Value,
    schema: &[ApiResult],
    path: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    match val {
        Value::Object(map) => {
            if let Some(obj_schema) = schema.iter().find(|r| r.type_ == "object") {
                let valid: HashSet<_> = obj_schema.inner.iter().map(|f| &f.key_name).collect();
                for (k, v) in map {
                    path.push(k.clone());
                    if valid.contains(&k) {
                        let field = obj_schema.inner.iter().find(|f| &f.key_name == k).unwrap();
                        compare_value(v, &field.inner, path, errors);
                    }
                    path.pop();
                }
            }
        }
        Value::Array(arr) => {
            if let Some(elem) = schema.first() {
                for (i, item) in arr.iter().enumerate() {
                    path.push(i.to_string());
                    compare_value(item, &elem.inner, path, errors);
                    path.pop();
                }
            }
        }
        _ => {
            let expected = schema
                .first()
                .map(|r| r.type_.as_str())
                .unwrap_or("unknown");
            let actual = match val {
                Value::String(_) => "string",
                Value::Number(_) => "number",
                Value::Bool(_) => "boolean",
                Value::Null => "none",
                _ => "unknown",
            };
            let ok = expected == actual
                || (expected == "string" && actual == "number")
                || (expected == "number" && actual == "string");
            if !ok {
                errors.push(format!(
                    "{}: expected={}, actual={}",
                    path.join("."),
                    expected,
                    actual
                ));
            }
        }
    }
}
