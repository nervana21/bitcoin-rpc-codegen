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
    let commands = api["commands"].as_object().unwrap();

    let mut parsed_methods = Vec::new();

    for (name, command_array) in commands {
        let command = &command_array.as_array().unwrap()[0];
        let command_obj = command.as_object().unwrap();

        let arguments = command_obj["arguments"]
            .as_array()
            .map(|params| {
                params
                    .iter()
                    .map(|param| ApiArgument {
                        names: param["names"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|n| n.as_str().unwrap().to_string())
                            .collect(),
                        type_: param["type"].as_str().unwrap().to_string(),
                        optional: param["optional"].as_bool().unwrap_or(false),
                        description: param["description"].as_str().unwrap_or("").to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let results = command_obj["results"]
            .as_array()
            .map(|results| results.iter().map(parse_result).collect())
            .unwrap_or_default();

        let category = command_obj["category"]
            .as_str()
            .unwrap_or("other")
            .to_string();

        parsed_methods.push(ApiMethod {
            name: name.clone(),
            description: command_obj["description"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            arguments,
            results,
            category,
        });
    }

    Ok(parsed_methods)
}

fn parse_result(value: &serde_json::Value) -> ApiResult {
    let obj = value.as_object().unwrap();

    ApiResult {
        type_: obj["type"].as_str().unwrap_or("").to_string(),
        description: obj["description"].as_str().unwrap_or("").to_string(),
        key_name: obj["key_name"].as_str().unwrap_or("").to_string(),
        inner: obj["inner"]
            .as_array()
            .map(|props| props.iter().map(parse_result).collect())
            .unwrap_or_default(),
    }
}
