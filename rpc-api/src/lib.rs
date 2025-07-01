//! rpc-api/src/lib.rs
//! Defines the canonical types, error enum, and supported-version logic.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod version;
pub use version::{Version, VersionError};

/// An RPC method's full schema
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiMethod {
    pub name: String,
    pub description: String,
    pub arguments: Vec<ApiArgument>,
    pub results: Vec<ApiResult>,
}

/// One argument to an RPC method
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiArgument {
    pub names: Vec<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub optional: bool,
    pub description: String,
}

/// One result field from an RPC method
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiResult {
    pub key_name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub inner: Vec<ApiResult>,
    pub optional: bool,
}

/// Strongly typed internal representation of an RPC method
/// This is the normalized IR from `api.json` that codegen modules depend on
#[derive(Debug, Clone)]
pub struct RpcMethod {
    pub name: String,
    pub category: String,
    pub description: String,
    pub examples: Vec<String>,
    pub params: Vec<Param>,
    pub result: Option<Type>,
}

/// A parameter to an RPC method with strongly typed information
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
    pub required: bool,
    pub description: String,
}

/// Strongly typed representation of RPC types
#[derive(Debug, Clone)]
pub enum Type {
    /// Primitive types like "string", "boolean", "number", etc.
    Primitive(String),
    /// Object with named fields
    Object(Vec<(String, Type)>),
    /// Array of a specific type
    Array(Box<Type>),
    /// Tuple of types
    Tuple(Vec<Type>),
    /// Optional type
    Option(Box<Type>),
    /// Unit type (void/none)
    Unit,
}

impl Type {
    /// Convert this type to a Rust type string
    pub fn to_rust_type(&self) -> String {
        match self {
            Type::Primitive(ty) => match ty.as_str() {
                "string" => "String".to_string(),
                "boolean" => "bool".to_string(),
                "number" => "f64".to_string(),
                "integer" => "i64".to_string(),
                "hex" => "String".to_string(),
                "time" => "u64".to_string(),
                "amount" => "f64".to_string(),
                _ => "serde_json::Value".to_string(), // fallback
            },
            Type::Object(fields) => {
                let field_defs = fields
                    .iter()
                    .map(|(name, ty)| format!("    pub {}: {},", name, ty.to_rust_type()))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{{\n{field_defs}\n}}")
            }
            Type::Array(element_ty) => format!("Vec<{}>", element_ty.to_rust_type()),
            Type::Tuple(types) => {
                let type_list = types
                    .iter()
                    .map(|ty| ty.to_rust_type())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({type_list})")
            }
            Type::Option(inner_ty) => format!("Option<{}>", inner_ty.to_rust_type()),
            Type::Unit => "()".to_string(),
        }
    }

    /// Check if this type should be wrapped in Option<T>
    pub fn is_optional(&self) -> bool {
        matches!(self, Type::Option(_))
    }
}

impl From<ApiMethod> for RpcMethod {
    fn from(api_method: ApiMethod) -> Self {
        let category = infer_category(&api_method.name);
        let examples = extract_examples(&api_method.description);

        let params = api_method
            .arguments
            .into_iter()
            .map(|arg| Param {
                name: arg.names[0].clone(),
                ty: Type::from_json_schema(&arg.type_, &arg.names[0]),
                required: !arg.optional,
                description: arg.description,
            })
            .collect();

        let result = if api_method.results.is_empty() {
            None
        } else {
            Some(Type::from_api_results(&api_method.results))
        };

        RpcMethod {
            name: api_method.name,
            category,
            description: api_method.description,
            examples,
            params,
            result,
        }
    }
}

impl Type {
    /// Convert from JSON schema type string to Type
    fn from_json_schema(schema_type: &str, _field_name: &str) -> Self {
        match schema_type {
            "string" => Type::Primitive("string".to_string()),
            "boolean" => Type::Primitive("boolean".to_string()),
            "number" => Type::Primitive("number".to_string()),
            "integer" => Type::Primitive("integer".to_string()),
            "hex" => Type::Primitive("hex".to_string()),
            "time" => Type::Primitive("time".to_string()),
            "amount" => Type::Primitive("amount".to_string()),
            "array" => Type::Array(Box::new(Type::Primitive("string".to_string()))), // default
            "object" => Type::Object(vec![]), // empty object by default
            _ => Type::Primitive(schema_type.to_string()),
        }
    }

    /// Convert from ApiResult array to Type
    pub fn from_api_results(results: &[ApiResult]) -> Self {
        if results.is_empty() {
            return Type::Unit;
        }

        if results.len() == 1 {
            let result = &results[0];
            if result.key_name.is_empty() {
                // Single unnamed result - could be primitive or complex
                if result.type_ == "object" && !result.inner.is_empty() {
                    // Create object from inner fields
                    let fields = result
                        .inner
                        .iter()
                        .map(|inner_result| {
                            let ty =
                                Type::from_json_schema(&inner_result.type_, &inner_result.key_name);
                            let ty = if inner_result.optional {
                                Type::Option(Box::new(ty))
                            } else {
                                ty
                            };
                            (inner_result.key_name.clone(), ty)
                        })
                        .collect();
                    Type::Object(fields)
                } else {
                    Type::from_json_schema(&result.type_, "")
                }
            } else {
                // Single named result - create object with one field
                Type::Object(vec![(
                    result.key_name.clone(),
                    Type::from_json_schema(&result.type_, &result.key_name),
                )])
            }
        } else {
            // Multiple results - create object with multiple fields
            let fields = results
                .iter()
                .map(|result| {
                    let ty = Type::from_json_schema(&result.type_, &result.key_name);
                    let ty = if result.optional {
                        Type::Option(Box::new(ty))
                    } else {
                        ty
                    };
                    (result.key_name.clone(), ty)
                })
                .collect();

            Type::Object(fields)
        }
    }
}

/// Infer the category of an RPC method based on its name
pub fn infer_category(method_name: &str) -> String {
    if method_name.starts_with("get") {
        "query".to_string()
    } else if method_name.starts_with("set")
        || method_name.starts_with("add")
        || method_name.starts_with("remove")
    {
        "modify".to_string()
    } else if method_name.starts_with("send") || method_name.starts_with("create") {
        "action".to_string()
    } else if method_name.starts_with("stop") || method_name.starts_with("start") {
        "control".to_string()
    } else {
        "other".to_string()
    }
}

/// Extract example usage from method description
pub fn extract_examples(description: &str) -> Vec<String> {
    // Simple heuristic: look for code blocks or "Example:" sections
    let mut examples = Vec::new();

    for line in description.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Example:") || trimmed.starts_with("```") {
            examples.push(trimmed.to_string());
        }
    }

    examples
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn parse_api_json(json: &str) -> Result<Vec<ApiMethod>, serde_json::Error> {
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

        parsed_methods.push(ApiMethod {
            name: name.clone(),
            description: command_obj["description"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            arguments,
            results,
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
        optional: obj["optional"].as_bool().unwrap_or(false),
    }
}
