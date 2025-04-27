// examples/regenerate_schema_v29.rs

use anyhow::Result;
use bitcoin_rpc_codegen::parser::{ApiArgument, ApiMethod, ApiResult};
use serde_json::{json, Value};
use std::{fs, path::PathBuf};

fn main() -> Result<()> {
    let feedback_dir = PathBuf::from("feedback");
    let output_path = PathBuf::from("resources/api_v29.json");

    println!("🔍 Checking if feedback/ directory exists...");
    if !feedback_dir.exists() {
        panic!("❌ Missing feedback/ directory! Did you run verify_all_methods_v29 first?");
    }

    // 🛡️ Ensure the output directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    println!("📂 Reading feedback/...");
    let mut methods = Vec::<ApiMethod>::new();

    for entry in fs::read_dir(&feedback_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        println!("🔎 Found feedback sample: {}", name);

        let raw: Value = serde_json::from_str(&fs::read_to_string(&path)?)?;
        let results: Vec<ApiResult> = parse_rpc_json(&raw);

        methods.push(ApiMethod {
            name: name.clone(),
            arguments: Vec::<ApiArgument>::new(),
            results,
            description: String::new(),
        });
    }

    // Group methods under a top-level `commands` map
    let mut commands = serde_json::Map::new();
    for m in methods {
        commands.insert(m.name.clone(), json!([m]));
    }

    let wrapped = json!({ "commands": commands });
    fs::write(&output_path, serde_json::to_string_pretty(&wrapped)?)?;

    println!("💾 Wrote updated schema to {}", output_path.display());

    Ok(())
}

/// Recursively build an `ApiResult` tree from live JSON data.
fn parse_rpc_json(value: &Value) -> Vec<ApiResult> {
    match value {
        Value::Object(map) => {
            let mut fields = Vec::new();
            for (k, v) in map {
                let inner = parse_rpc_json(v);
                fields.push(ApiResult {
                    type_: "object".to_string(),
                    description: String::new(),
                    key_name: k.clone(),
                    inner,
                });
            }
            vec![ApiResult {
                type_: "object".to_string(),
                description: String::new(),
                key_name: String::new(),
                inner: fields,
            }]
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                vec![ApiResult {
                    type_: "array".to_string(),
                    description: String::new(),
                    key_name: String::new(),
                    inner: Vec::new(),
                }]
            } else {
                let elem_schema = parse_rpc_json(&arr[0]);
                vec![ApiResult {
                    type_: "array".to_string(),
                    description: String::new(),
                    key_name: String::new(),
                    inner: elem_schema,
                }]
            }
        }
        Value::String(_) => vec![primitive("string")],
        Value::Number(_) => vec![primitive("number")],
        Value::Bool(_) => vec![primitive("boolean")],
        Value::Null => vec![primitive("none")],
    }
}

fn primitive(t: &str) -> ApiResult {
    ApiResult {
        type_: t.to_string(),
        description: String::new(),
        key_name: String::new(),
        inner: Vec::new(),
    }
}
