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

    // 🔍 DEBUG: dump the first 1k chars of the schema so we can see exactly
    eprintln!(
        "--- begin schema ({} bytes) ---\n{}{}\n--- end schema ---",
        schema_src.len(),
        &schema_src[..schema_src.len().min(1024)],
        if schema_src.len() > 1024 {
            "\n…(truncated)"
        } else {
            ""
        }
    );

    let methods: Vec<ApiMethod> =
        parse_api_json(&schema_src).context("Failed to parse api_v29.json")?;

    // 2. Spawn a regtest node
    let mut conf = Conf::default();
    conf.extra_args.push("-fallbackfee=0.0002");
    let rt = RegtestClient::new_with_conf(&conf).context("Failed to start regtest node")?;
    let client = &rt.client;

    println!("Loaded {} RPC methods from schema", methods.len());

    // 3. For each method with no required args, call and validate
    for method in &methods {
        // skip *all* methods that take any parameters,
        // so we only call the zero-arg RPCs
        if !method.arguments.is_empty() {
            println!("Skipping `{}`: has arguments", method.name);
            continue;
        }

        println!("\nCalling `{}`...", method.name);
        let response: Value = match client.call_json(&method.name, &[]) {
            Ok(v) => {
                // dump the live JSON for later schema regeneration
                fs::create_dir_all("feedback")?;
                fs::write(
                    format!("feedback/{}.json", method.name),
                    serde_json::to_string_pretty(&v)?,
                )?;
                println!(
                    "   → got {} bytes (dumped to feedback/{})",
                    v.to_string().len(),
                    method.name
                );
                v
            }
            Err(err) => {
                // print a skip note and move on instead of erroring out
                println!("  ⎯ Skipping `{}`: RPC error: {}", method.name, err);
                continue;
            }
        };

        let mut errors = Vec::new();
        compare_value(&response, &method.results, &mut Vec::new(), &mut errors);

        if errors.is_empty() {
            println!("  ✔ `{}` matches schema", method.name);
        } else {
            println!("  ✖ `{}` schema mismatches:", method.name);
            for e in errors {
                println!("    - {}", e);
            }
        }
    }

    Ok(())
}

/// Recursively compare a `serde_json::Value` against an expected `ApiResult` schema,
/// ignoring any extra fields and permitting number↔string mismatches.
fn compare_value(
    val: &Value,
    schema: &[ApiResult],
    path: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    match val {
        Value::Object(map) => {
            if let Some(obj_schema) = schema.iter().find(|r| r.type_ == "object") {
                // build a set of valid field names
                let valid_keys: HashSet<_> = obj_schema.inner.iter().map(|f| &f.key_name).collect();

                for (k, v) in map {
                    path.push(k.clone());
                    if valid_keys.contains(&k) {
                        // recurse only for known fields
                        let field_schema =
                            obj_schema.inner.iter().find(|f| &f.key_name == k).unwrap();
                        compare_value(v, &field_schema.inner, path, errors);
                    }
                    // else: silently ignore unknown fields
                    path.pop();
                }
            }
        }
        Value::Array(arr) => {
            if let Some(elem_schema) = schema.first() {
                for (i, item) in arr.iter().enumerate() {
                    path.push(i.to_string());
                    compare_value(item, &elem_schema.inner, path, errors);
                    path.pop();
                }
            }
        }
        _ => {
            // Primitive type check with number<->string fallback
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
            let okay = expected == actual
                || (expected == "string" && actual == "number")
                || (expected == "number" && actual == "string");
            if !okay {
                errors.push(format!(
                    "{}: type mismatch (schema={}, got={})",
                    path.join("."),
                    expected,
                    actual
                ));
            }
        }
    }
}
