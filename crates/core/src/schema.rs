// crate/core/src/schema.rs

//! Schema extraction for Bitcoin RPC Code Generator (core crate).
//!
//! Parses raw `bitcoin-cli help` text files into a structured JSON schema.

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::{fs, io::Write, path::Path};

use crate::error::{CoreError, SchemaError};

/// Represents a single RPC argument.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiArgument {
    /// Parameter names (aliases)
    pub names: Vec<String>,
    /// Base type (e.g. "string", "numeric")
    pub type_: String,
    /// True if the argument is optional
    pub optional: bool,
    /// Description text
    pub description: String,
}

/// Represents a single RPC result field (possibly nested).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResult {
    /// JSON type ("string", "number", "object", "array", "boolean", "none")
    pub type_: String,
    /// Field key name (empty for array items)
    pub key_name: String,
    /// Description text
    pub description: String,
    /// Nested inner fields
    pub inner: Vec<ApiResult>,
}

/// Represents a full RPC method schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiMethod {
    pub name: String,
    pub type_: String,
    pub description: String,
    pub arguments: Vec<ApiArgument>,
    pub results: Vec<ApiResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<String>,
}

/// Parse a single help-text document into an ApiMethod.
pub fn parse_method_doc(name: &str, doc: &str) -> ApiMethod {
    let description = extract_description(doc);
    let arg_re = Regex::new(r#"^\s*\d+\.\s+"?([^"\s]+)"?\s*\(([^)]+)\)\s*(.*)$"#).unwrap();
    let arguments = infer_arguments(doc, &arg_re);
    let results = infer_results(doc);
    let examples = extract_examples(doc);
    ApiMethod {
        name: name.to_string(),
        type_: String::new(),
        description,
        arguments,
        results,
        examples,
    }
}

/// Walk `docs_dir`, parse each `<method>.txt`, and write JSON schema to `out_file`.
pub fn extract_api_docs(docs_dir: &Path, out_file: &Path) -> Result<(), CoreError> {
    let mut commands = Map::new();
    for entry in fs::read_dir(docs_dir).map_err(|e| {
        CoreError::Schema(SchemaError::InvalidFormat(format!(
            "reading docs_dir failed: {e}"
        )))
    })? {
        let path = entry
            .map_err(|e| {
                CoreError::Schema(SchemaError::InvalidFormat(format!(
                    "iterating docs_dir failed: {e}"
                )))
            })?
            .path();
        if path.extension().and_then(|e| e.to_str()) != Some("txt") {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap();
        let content = fs::read_to_string(&path).map_err(|e| {
            CoreError::Schema(SchemaError::InvalidFormat(format!(
                "reading {path:?} failed: {e}"
            )))
        })?;
        let method = parse_method_doc(stem, &content);
        commands.insert(stem.to_string(), json!([method]));
    }
    let wrapper = json!({ "commands": commands });
    let mut f = fs::File::create(out_file).map_err(|e| {
        CoreError::Schema(SchemaError::InvalidFormat(format!(
            "creating {out_file:?} failed: {e}"
        )))
    })?;
    writeln!(f, "{}", serde_json::to_string_pretty(&wrapper)?).map_err(|e| {
        CoreError::Schema(SchemaError::InvalidFormat(format!(
            "writing {out_file:?} failed: {e}"
        )))
    })?;
    Ok(())
}

/// Extracts the method description (skip signature and initial blank lines).
fn extract_description(doc: &str) -> String {
    doc.lines()
        .skip_while(|l| l.trim().is_empty())
        .skip(1)
        .skip_while(|l| l.trim().is_empty())
        .take_while(|l| {
            let t = l.trim();
            !t.starts_with("Arguments:")
                && !t.starts_with("Result")
                && !t.starts_with("Returns:")
                && !t.starts_with("Examples:")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Infers arguments by regex on the "Arguments:" block, extracting base type.
fn infer_arguments(doc: &str, re: &Regex) -> Vec<ApiArgument> {
    let mut args = Vec::new();
    let mut in_args = false;
    for line in doc.lines() {
        let t = line.trim();
        if t.starts_with("Arguments:") {
            in_args = true;
            continue;
        }
        if in_args {
            if t.is_empty() || t.ends_with(':') {
                break;
            }
            if let Some(c) = re.captures(line) {
                let name = c[1].to_string();
                let raw_typ = c[2].to_lowercase();
                let optional = raw_typ.contains("optional");
                let base_type = raw_typ.split(',').next().unwrap().trim().to_string();
                let desc = c[3].trim().to_string();
                args.push(ApiArgument {
                    names: vec![name],
                    type_: base_type,
                    optional,
                    description: desc,
                });
            }
        }
    }
    args
}

/// Parses the "Result" block into nested ApiResult structures.
fn infer_results(doc: &str) -> Vec<ApiResult> {
    let mut in_res = false;
    let mut stack: Vec<(usize, Vec<ApiResult>)> = vec![(0, Vec::new())];
    let field_re = Regex::new(r#"^"([^"]+)"\s*\(([^)]+)\)\s*(.*)$"#).unwrap();

    for line in doc.lines() {
        let t = line.trim();
        if t.starts_with("Result") || t.starts_with("Returns") {
            in_res = true;
            continue;
        }
        if in_res
            && (t.starts_with("Arguments:")
                || t.starts_with("Examples:")
                || t.chars().next().is_some_and(|c| c.is_ascii_digit()))
        {
            break;
        }
        if !in_res || t.is_empty() {
            continue;
        }

        let depth = line.chars().take_while(|c| c.is_whitespace()).count();
        let (key_name, type_hint, description) = if let Some(cap) = field_re.captures(t) {
            (
                cap[1].to_string(),
                cap[2].to_lowercase(),
                cap[3].trim().to_string(),
            )
        } else {
            (String::new(), String::new(), t.to_string())
        };

        let typ = if type_hint.contains("boolean") {
            "boolean"
        } else if type_hint.contains("numeric") {
            "number"
        } else if type_hint.contains("json object") {
            "object"
        } else if type_hint.contains("json null") {
            "none"
        } else {
            "string"
        };

        let node = ApiResult {
            type_: typ.to_string(),
            key_name,
            description,
            inner: Vec::new(),
        };

        while depth < stack.last().unwrap().0 {
            let (_, mut children) = stack.pop().unwrap();
            stack
                .last_mut()
                .unwrap()
                .1
                .last_mut()
                .unwrap()
                .inner
                .append(&mut children);
        }
        stack.last_mut().unwrap().1.push(node);
        stack.push((depth, Vec::new()));
    }

    while stack.len() > 1 {
        let (_, mut children) = stack.pop().unwrap();
        stack
            .last_mut()
            .unwrap()
            .1
            .last_mut()
            .unwrap()
            .inner
            .append(&mut children);
    }

    let result = stack.pop().unwrap().1;
    if result.is_empty() {
        vec![ApiResult {
            type_: "none".into(),
            key_name: String::new(),
            description: String::new(),
            inner: Vec::new(),
        }]
    } else {
        result
    }
}

/// Extracts example usage from the help text.
fn extract_examples(doc: &str) -> Option<String> {
    let mut in_examples = false;
    let mut examples = Vec::new();

    for line in doc.lines() {
        let t = line.trim();
        if t.starts_with("Examples:") {
            in_examples = true;
            continue;
        }
        if in_examples {
            if t.is_empty() || t.starts_with("Arguments:") || t.starts_with("Result") {
                break;
            }
            examples.push(t.to_string());
        }
    }

    if examples.is_empty() {
        None
    } else {
        Some(examples.join("\n"))
    }
}

/// Parse a full JSON schema (generated by `extract_api_docs`) into a list of ApiMethod.
/// Empty or whitespace input yields an empty Vec.
///
/// This enhanced version includes verbose logging to stderr to show exactly what's happening.
pub fn parse_api_json(input: &str) -> Result<Vec<ApiMethod>, CoreError> {
    if input.trim().is_empty() {
        return Ok(Vec::new());
    }

    // 1) parse the raw JSON
    let v: Value = serde_json::from_str(input).map_err(|e| {
        CoreError::Schema(SchemaError::InvalidFormat(format!(
            "Failed to deserialize API JSON schema: {e}"
        )))
    })?;

    // 🔍 DEBUG #1: top‐level keys
    if let Value::Object(o) = &v {
        let keys: Vec<String> = o.keys().cloned().collect();
        eprintln!("[parse_api_json] top‐level keys = {:?}", keys);
    } else {
        eprintln!("[parse_api_json] top‐level is not an object: {:?}", v);
    }

    // 🔍 DEBUG #2: inspect `commands`
    match v.get("commands") {
        Some(cmds) => {
            let cnt = cmds.as_object().map(|m| m.len()).unwrap_or(0);
            eprintln!("[parse_api_json] found `commands` with {} entries", cnt);
        }
        None => {
            eprintln!("[parse_api_json] *** MISSING `commands` key! ***");
        }
    }

    // 2) now safely grab the commands map (or use empty map as fallback)
    let empty_map = Map::new();
    let commands = v
        .get("commands")
        .and_then(|c| c.as_object())
        .unwrap_or(&empty_map);

    // 3) build your Vec<ApiMethod>
    let mut methods = Vec::new();
    for (name, arr) in commands {
        if let Some(items) = arr.as_array() {
            for item in items {
                let mut m: ApiMethod = serde_json::from_value(item.clone()).map_err(|e| {
                    CoreError::Schema(SchemaError::InvalidFormat(format!(
                        "Invalid ApiMethod entry for {name}: {e}"
                    )))
                })?;
                m.name = name.clone();
                methods.push(m);
            }
        }
    }
    Ok(methods)
}

#[test]
fn parses_nested_inner_fields() {
    let json = r#"
    {
      "commands": {
        "dummy": [
          {
            "name": "dummy",
            "type_": "method",
            "description": "desc",
            "arguments": [],
            "results": [
              {
                "type_": "object",
                "description": "",
                "key_name": "result",
                "inner": [
                  {
                    "type_": "string",
                    "description": "nested",
                    "key_name": "foo",
                    "inner": []
                  }
                ]
              }
            ]
          }
        ]
      }
    }"#;

    let methods = parse_api_json(json).unwrap();
    let result = &methods[0].results[0];
    let inner = &result.inner[0];

    assert_eq!(methods[0].name, "dummy");
    assert_eq!(result.key_name, "result");
    assert_eq!(result.type_, "object");
    assert_eq!(inner.key_name, "foo");
    assert_eq!(inner.type_, "string");
    assert_eq!(inner.description, "nested");
}
