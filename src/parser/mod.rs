use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiMethod {
    pub name: String,
    pub description: String,
    pub arguments: Vec<ApiArgument>,
    pub results: Vec<ApiResult>,
    pub category: String,
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

pub fn parse_api_json(json: &str) -> Result<Vec<ApiMethod>> {
    let api: serde_json::Value = serde_json::from_str(json)?;
    let commands = match api.get("commands").and_then(|c| c.as_object()) {
        Some(obj) => obj,
        None => return Ok(vec![]),
    };

    let mut parsed_methods = Vec::new();

    for (name, command_array) in commands {
        if let Some(command) = command_array.as_array().and_then(|a| a.get(0)) {
            let empty_map = serde_json::Map::new();
            let command_obj = command.as_object().unwrap_or(&empty_map);

            let arguments = command_obj
                .get("arguments")
                .and_then(|a| a.as_array())
                .map(|params| {
                    params
                        .iter()
                        .map(|param| ApiArgument {
                            names: param["names"]
                                .as_array()
                                .unwrap_or(&vec![])
                                .iter()
                                .map(|n| n.as_str().unwrap_or("").to_string())
                                .collect(),
                            type_: param["type"].as_str().unwrap_or("string").to_string(),
                            optional: param["optional"].as_bool().unwrap_or(false),
                            description: param["description"].as_str().unwrap_or("").to_string(),
                        })
                        .collect()
                })
                .unwrap_or_default();

            let results = command_obj
                .get("results")
                .and_then(|r| r.as_array())
                .map(|results| results.iter().map(parse_result).collect())
                .unwrap_or_default();

            parsed_methods.push(ApiMethod {
                name: name.clone(),
                description: command_obj
                    .get("description")
                    .and_then(|d| d.as_str())
                    .unwrap_or("")
                    .to_string(),
                arguments,
                results,
            });
        }
    }

    Ok(parsed_methods)
}

fn parse_result(value: &serde_json::Value) -> ApiResult {
    let empty_map = serde_json::Map::new();
    let obj = value.as_object().unwrap_or(&empty_map);

    let inner = obj
        .get("inner")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().map(parse_result).collect())
        .unwrap_or_default();

    ApiResult {
        type_: obj
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("string")
            .to_string(),
        description: obj
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        key_name: obj
            .get("key_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        inner,
    }
}
