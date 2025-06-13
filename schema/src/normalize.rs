use parser::MethodHelp;
use rpc_api::{ApiMethod, ApiResult};
use thiserror::Error;

/// Errors from normalization
#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("no help blocks provided")]
    NoHelpBlocks,
    #[error("malformed result section: {0}")]
    MalformedResult(String),
}

/// Trait to convert raw help into a structured schema
pub trait SchemaNormalizer {
    fn normalize(&self, helps: &[MethodHelp]) -> Result<Vec<ApiMethod>, SchemaError>;
}

/// Default implementation: one-to-one mapping with parsed result fields
pub struct DefaultSchemaNormalizer;

impl SchemaNormalizer for DefaultSchemaNormalizer {
    fn normalize(&self, helps: &[MethodHelp]) -> Result<Vec<ApiMethod>, SchemaError> {
        if helps.is_empty() {
            return Err(SchemaError::NoHelpBlocks);
        }

        let methods = helps
            .iter()
            .map(|mh| {
                let results = parse_results(&mh.raw)?;
                Ok(ApiMethod {
                    name: mh.name.clone(),
                    description: mh.raw.clone(),
                    arguments: Vec::new(),
                    results,
                })
            })
            .collect::<Result<Vec<_>, SchemaError>>()?;

        Ok(methods)
    }
}

/// Extract and parse the `Result:` section into ApiResult entries
fn parse_results(raw: &str) -> Result<Vec<ApiResult>, SchemaError> {
    let mut results = Vec::new();
    let mut in_results = false;
    let mut current_name: Option<String> = None;
    let mut current_desc = String::new();

    for line in raw.lines() {
        let trimmed = line.trim();

        if !in_results {
            if trimmed.starts_with("Result:") {
                in_results = true;
            }
            continue;
        }

        // stop if we hit another section or blank line
        if trimmed.is_empty() || trimmed.starts_with("==") {
            break;
        }

        // new field line must start with a quoted key
        if trimmed.starts_with('"') {
            if let Some((key, desc)) = trimmed.split_once(" : ") {
                // flush previous field
                if let Some(name) = current_name.take() {
                    results.push(build_api_result(name, current_desc.trim().to_owned()));
                    current_desc.clear();
                }
                // start new field
                current_name = Some(key.trim_matches('"').to_string());
                current_desc.push_str(desc.trim());
            } else {
                // malformed line in results
                return Err(SchemaError::MalformedResult(trimmed.to_string()));
            }
        } else if current_name.is_some() {
            // continuation of description
            current_desc.push(' ');
            current_desc.push_str(trimmed);
        }
    }

    // flush last field
    if let Some(name) = current_name.take() {
        results.push(build_api_result(name, current_desc.trim().to_owned()));
    }

    Ok(results)
}

/// Construct an `ApiResult` from a field name & its full description
fn build_api_result(name: String, desc: String) -> ApiResult {
    let type_ = infer_type_from_desc(&desc, &name);
    let optional = desc.contains("optional") || desc.contains("only present if");

    ApiResult {
        key_name: name,
        type_,
        description: desc.clone(),
        inner: Vec::new(),
        optional,
    }
}

/// Decide JSON type based on description & field name
fn infer_type_from_desc(desc: &str, field_name: &str) -> String {
    if desc.contains("numeric") || desc.contains("n,") {
        infer_numeric_type(desc, field_name).to_string()
    } else if desc.contains("boolean") || desc.contains("true|false") {
        "boolean".into()
    } else if desc.contains("hex") {
        "hex".into()
    } else {
        "string".into()
    }
}

fn infer_numeric_type(desc: &str, field_name: &str) -> &'static str {
    if desc.contains("amount")
        || desc.contains("BTC")
        || field_name.contains("amount")
        || field_name.contains("fee")
    {
        "amount"
    } else if desc.contains("rate")
        || desc.contains("probability")
        || desc.contains("percentage")
        || field_name.contains("rate")
        || field_name.contains("difficulty")
    {
        "numeric"
    } else if desc.contains("target")
        || desc.contains("bits")
        || field_name.contains("target")
        || field_name.contains("bits")
    {
        "bigint"
    } else {
        "number"
    }
}
