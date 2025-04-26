// examples/regenerate_schema_v29.rs

use anyhow::Result;
use bitcoin_rpc_codegen::parser::{ApiMethod, ApiResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs};

/// Mirror the top-level shape of resources/api_v29.json
#[derive(Serialize, Deserialize)]
struct Schema {
    commands: HashMap<String, Vec<ApiMethod>>,
}

fn main() -> Result<()> {
    // 1. Load the existing file into our Schema struct
    let mut schema: Schema = serde_json::from_str(include_str!("../resources/api_v29.json"))?;

    // 2. For each dump in feedback/, parse and replace that method’s `results`
    for entry in fs::read_dir("feedback")? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let cmd = path.file_stem().unwrap().to_string_lossy().to_string();
        let raw: Value = serde_json::from_str(&fs::read_to_string(&path)?)?;

        if let Some(methods) = schema.commands.get_mut(&cmd) {
            for m in methods.iter_mut() {
                m.results = parse_rpc_json(&raw);
            }
            println!("  • updated results for `{}`", cmd);
        } else {
            eprintln!("  ⚠️ no existing schema for `{}`, skipping", cmd);
        }
    }

    // 3. Write back with the exact same “commands” wrapper
    let out = serde_json::to_string_pretty(&schema)?;
    fs::write("resources/api_v29.json", &out)?;
    println!("\n✅ Wrote updated schema to resources/api_v29.json");

    Ok(())
}

/// Recursively convert a serde_json::Value into your Vec<ApiResult> tree.
fn parse_rpc_json(raw: &Value) -> Vec<ApiResult> {
    match raw {
        Value::Object(map) => {
            let mut fields = Vec::with_capacity(map.len());
            for (k, v) in map {
                fields.push(ApiResult {
                    type_: infer_type(v),
                    key_name: k.clone(),
                    description: String::new(),
                    inner: parse_rpc_json(v),
                });
            }
            vec![ApiResult {
                type_: "object".into(),
                key_name: String::new(),
                description: String::new(),
                inner: fields,
            }]
        }
        Value::Array(arr) => {
            // Sample the first element if present
            let elem_schema = arr.get(0).map_or_else(Vec::new, parse_rpc_json);
            vec![ApiResult {
                type_: "array".into(),
                key_name: String::new(),
                description: String::new(),
                inner: elem_schema,
            }]
        }
        _ => vec![ApiResult {
            type_: infer_type(raw),
            key_name: String::new(),
            description: String::new(),
            inner: Vec::new(),
        }],
    }
}

/// Map a JSON Value to your schema’s `type_` string
fn infer_type(v: &Value) -> String {
    match v {
        Value::String(_) => "string".into(),
        Value::Number(_) => "number".into(),
        Value::Bool(_) => "boolean".into(),
        Value::Null => "none".into(),
        Value::Object(_) => "object".into(),
        Value::Array(_) => "array".into(),
    }
}
