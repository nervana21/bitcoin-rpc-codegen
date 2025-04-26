use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiMethod {
    pub name: String,
    pub description: String,
    pub arguments: Vec<ApiArgument>,
    pub results: Vec<ApiResult>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiArgument {
    pub names: Vec<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub optional: bool,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiResult {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub key_name: String,
    pub inner: Vec<ApiResult>,
}

// use crate::parser::ApiMethod;
use anyhow::{Context, Error};
use serde_json::Value;

/// Parses your `api_v29.json` into a Vec<ApiMethod>,
/// with verbose logging to stderr to show exactly what’s happening.
pub fn parse_api_json(json: &str) -> Result<Vec<ApiMethod>, Error> {
    // 1) parse the raw JSON
    let v: Value = serde_json::from_str(json).context("Invalid API JSON schema")?;

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

    // 2) now safely grab the commands map (or panic with our debug hint)
    let commands = v
        .get("commands")
        .expect("`commands` key missing—see debug above")
        .as_object()
        .expect("`commands` was not an object");

    // 3) build your Vec<ApiMethod> as before
    let mut all = Vec::new();
    for (name, raw_methods) in commands {
        let arr = raw_methods
            .as_array()
            .expect(&format!("Methods for `{}` not an array", name));
        for raw in arr {
            let mut method: ApiMethod = serde_json::from_value(raw.clone())
                .context(format!("Deserializing ApiMethod for `{}` failed", name))?;
            method.name = name.clone();
            all.push(method);
        }
    }
    Ok(all)
}

#[test]
fn parses_nested_inner_fields() {
    let json = r#"
    {
      "commands": {
        "dummy": [
          {
            "name": "dummy",
            "description": "desc",
            "arguments": [],
            "results": [
              {
                "type": "object",
                "description": "",
                "key_name": "result",
                "inner": [
                  {
                    "type": "string",
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
