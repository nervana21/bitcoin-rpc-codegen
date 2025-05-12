// schema/src/normalize.rs

use thiserror::Error;

use parser::MethodHelp;
use rpc_api::ApiMethod;

/// Errors from normalization
#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("no help blocks provided")]
    NoHelpBlocks,
    // TODO: add more detailed parse errors here
}

/// Trait to convert raw help into a structured schema
pub trait SchemaNormalizer {
    fn normalize(&self, helps: &[MethodHelp]) -> Result<Vec<ApiMethod>, SchemaError>;
}

/// Default: one‑to‑one mapping; description = raw text; empty args/results
pub struct DefaultSchemaNormalizer;

impl SchemaNormalizer for DefaultSchemaNormalizer {
    fn normalize(&self, helps: &[MethodHelp]) -> Result<Vec<ApiMethod>, SchemaError> {
        if helps.is_empty() {
            return Err(SchemaError::NoHelpBlocks);
        }
        Ok(helps
            .iter()
            .map(|mh| {
                // Parse response type from help text
                let mut results = Vec::new();
                let mut in_result_section = false;
                let mut current_field = None;
                let mut current_desc = String::new();

                for line in mh.raw.lines() {
                    let line = line.trim();

                    // Look for result section
                    if line.starts_with("Result:") {
                        in_result_section = true;
                        continue;
                    }

                    // Skip empty lines and section headers
                    if line.is_empty() || line.starts_with("==") {
                        in_result_section = false;
                        continue;
                    }

                    if in_result_section {
                        // Check if this is a new field
                        if line.starts_with('"') || line.starts_with("  \"") {
                            // If we have a previous field, add it
                            if let Some((name, desc)) = current_field.take() {
                                results.push(create_api_result(name, desc));
                            }

                            // Parse new field
                            if let Some((name, desc)) = line.split_once(" : ") {
                                let name = name.trim().trim_matches('"').trim();
                                let desc = desc.trim();
                                current_field = Some((name.to_string(), desc.to_string()));
                            }
                        } else if let Some((name, _desc)) = &current_field {
                            // Append to current field's description
                            current_desc.push_str(line);
                            current_field = Some((name.clone(), current_desc.clone()));
                        }
                    }
                }

                // Add the last field if any
                if let Some((name, desc)) = current_field {
                    results.push(create_api_result(name, desc));
                }

                ApiMethod {
                    name: mh.name.clone(),
                    description: mh.raw.clone(),
                    arguments: Vec::new(), // TODO: Parse arguments too
                    results,
                }
            })
            .collect())
    }
}

fn create_api_result(name: String, desc: String) -> rpc_api::ApiResult {
    // Determine type from description
    let type_ = if desc.contains("numeric") || desc.contains("n,") {
        "number".to_string()
    } else if desc.contains("boolean") || desc.contains("true|false") {
        "boolean".to_string()
    } else if desc.contains("hex") {
        "hex".to_string()
    } else if desc.contains("time") || desc.contains("timestamp") {
        "time".to_string()
    } else if desc.contains("amount") || desc.contains("BTC") {
        "amount".to_string()
    } else if desc.contains("array") || desc.contains("json array") {
        "array".to_string()
    } else if desc.contains("object") || desc.contains("json object") {
        "object".to_string()
    } else {
        "string".to_string()
    };

    rpc_api::ApiResult {
        key_name: name,
        type_,
        description: desc.clone(),
        inner: Vec::new(),
        optional: desc.contains("optional") || desc.contains("only present if"),
    }
}
