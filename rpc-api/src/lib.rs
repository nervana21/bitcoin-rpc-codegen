//! rpc-api/src/lib.rs
//! Defines the canonical types, error enum, and supported-version logic.

use indexmap::IndexMap;
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
#[derive(Debug, Clone, PartialEq)]
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

    // Handle the canonical rpcs structure
    let rpcs = api["rpcs"].as_object().unwrap();

    let mut parsed_methods = Vec::new();

    for (name, command_obj) in rpcs {
        let command_obj = command_obj.as_object().unwrap();

        // --- DIAGNOSTIC PRINTS ---
        println!("\n[DIAGNOSTIC] Processing command: {name}");
        println!(
            "[DIAGNOSTIC] command_obj keys: {:?}",
            command_obj.keys().collect::<Vec<_>>()
        );
        println!("[DIAGNOSTIC] Full command_obj: {:#?}", command_obj);

        // If you want to see the full JSON for this command:
        println!(
            "[DIAGNOSTIC] Full command JSON: {}",
            serde_json::to_string_pretty(command_obj).unwrap()
        );

        // --- Support both v28 (array) and v29 (object) argument formats ---
        let arguments = match command_obj.get("arguments") {
            Some(args) if args.is_array() => {
                // v28 style
                args.as_array()
                    .unwrap()
                    .iter()
                    .map(|param| ApiArgument {
                        names: param
                            .get("names")
                            .and_then(|n| n.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_else(Vec::new),
                        type_: param
                            .get("type")
                            .and_then(|t| t.as_str())
                            .unwrap_or("string")
                            .to_string(),
                        optional: param
                            .get("optional")
                            .and_then(|b| b.as_bool())
                            .unwrap_or(false),
                        description: param
                            .get("description")
                            .and_then(|d| d.as_str())
                            .unwrap_or("")
                            .to_string(),
                    })
                    .collect()
            }
            Some(args) if args.is_object() => {
                // v29 style (JSON Schema) - Use IndexMap to preserve order
                let properties = args
                    .get("properties")
                    .and_then(|p| p.as_object())
                    .map(|obj| {
                        // Convert to IndexMap to preserve insertion order
                        let mut index_map = IndexMap::new();
                        for (key, value) in obj {
                            index_map.insert(key.clone(), value.clone());
                        }
                        index_map
                    })
                    .unwrap_or_else(IndexMap::new);

                let required: std::collections::HashSet<_> = args
                    .get("required")
                    .and_then(|arr| arr.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                    .unwrap_or_default();

                // Now iterate in preserved order
                properties
                    .iter()
                    .map(|(name, prop)| ApiArgument {
                        names: vec![name.clone()],
                        type_: prop
                            .get("type")
                            .and_then(|t| t.as_str())
                            .unwrap_or("string")
                            .to_string(),
                        optional: !required.contains(name.as_str()),
                        description: prop
                            .get("description")
                            .and_then(|d| d.as_str())
                            .unwrap_or("")
                            .to_string(),
                    })
                    .collect()
            }
            _ => vec![], // No arguments
        };

        // --- DIAGNOSTIC PRINTS FOR RESULTS ---
        if !command_obj.contains_key("results") {
            println!("[ERROR] Command '{name}' is missing 'results' key!");
        }

        let results = command_obj
            .get("results")
            .and_then(|r| r.as_array())
            .map(|arr| arr.iter().map(parse_result).collect())
            .unwrap_or_else(Vec::new);

        let description = command_obj
            .get("description")
            .and_then(|d| d.as_str())
            .unwrap_or("")
            .to_string();

        parsed_methods.push(ApiMethod {
            name: name.clone(),
            description,
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
        inner: obj
            .get("inner")
            .and_then(|v| v.as_array())
            .map(|props| props.iter().map(parse_result).collect())
            .unwrap_or_default(),
        optional: obj["optional"].as_bool().unwrap_or(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_example_lines() {
        let desc = r#"
            This method does something.

            Example:
            bitcoin-cli getblockchaininfo

            ```
            bitcoin-cli getblockchaininfo
            ```
        "#;

        let examples = extract_examples(desc);

        // Should extract both the "Example:" line and the code block line(s)
        assert!(examples.iter().any(|l| l.starts_with("Example:")));
        assert!(examples.iter().any(|l| l.starts_with("```")));
        assert!(
            !examples.is_empty(),
            "Should extract at least one example line"
        );
    }

    #[test]
    fn extract_examples_cases() {
        use super::extract_examples;

        // Both Example: and code block
        let desc = "This method.\nExample: foo\n```\nbar\n```\n";
        let ex = extract_examples(desc);
        assert_eq!(ex, vec!["Example: foo", "```", "```"]);

        // Only Example:
        let desc = "Blah\nExample: test\nBlah";
        let ex = extract_examples(desc);
        assert_eq!(ex, vec!["Example: test"]);

        // Only code block
        let desc = "Blah\n```\nBlah";
        let ex = extract_examples(desc);
        assert_eq!(ex, vec!["```"]);

        // Neither
        let desc = "No examples here.";
        let ex = extract_examples(desc);
        assert!(ex.is_empty());
    }

    #[test]
    fn test_optional_argument_handling() {
        // Test that optional arguments are properly marked as not required
        let api_method = ApiMethod {
            name: "test".into(),
            description: "test".into(),
            arguments: vec![
                ApiArgument {
                    names: vec!["required".into()],
                    type_: "string".into(),
                    optional: false,
                    description: "required".into(),
                },
                ApiArgument {
                    names: vec!["optional".into()],
                    type_: "string".into(),
                    optional: true,
                    description: "optional".into(),
                },
            ],
            results: vec![],
        };

        let rpc_method: RpcMethod = api_method.into();
        assert_eq!(rpc_method.params.len(), 2);
        assert_eq!(rpc_method.params[0].required, true); // !false = true
        assert_eq!(rpc_method.params[1].required, false); // !true = false
    }

    #[test]
    fn test_from_api_results_edge_cases() {
        // Test object type with empty inner fields
        let result = ApiResult {
            key_name: "".into(),
            type_: "object".into(),
            description: "".into(),
            inner: vec![], // empty inner
            optional: false,
        };
        let ty = Type::from_api_results(&[result]);
        // Should fall through to else branch and call from_json_schema
        assert!(matches!(ty, Type::Object(_)));

        // Test non-object type with non-empty inner (edge case)
        let result = ApiResult {
            key_name: "".into(),
            type_: "string".into(), // not object
            description: "".into(),
            inner: vec![ApiResult {
                key_name: "foo".into(),
                type_: "string".into(),
                description: "".into(),
                inner: vec![],
                optional: false,
            }],
            optional: false,
        };
        let ty = Type::from_api_results(&[result]);
        // Should fall through to else branch
        assert!(matches!(ty, Type::Primitive(_)));
    }

    #[test]
    fn test_from_json_schema_all_cases() {
        // Test all match arms to ensure they're not dead code
        let test_cases = vec![
            ("string", Type::Primitive("string".into())),
            ("boolean", Type::Primitive("boolean".into())),
            ("number", Type::Primitive("number".into())),
            ("integer", Type::Primitive("integer".into())),
            ("hex", Type::Primitive("hex".into())),
            ("time", Type::Primitive("time".into())),
            ("amount", Type::Primitive("amount".into())),
            (
                "array",
                Type::Array(Box::new(Type::Primitive("string".into()))),
            ),
            ("object", Type::Object(vec![])),
        ];

        for (input, expected) in test_cases {
            let result = Type::from_json_schema(input, "test");
            assert_eq!(result, expected, "Failed for input: {}", input);
        }

        // Test fallback case
        let result = Type::from_json_schema("unknown_type", "test");
        assert!(matches!(result, Type::Primitive(_)));
    }

    #[test]
    fn test_parse_result_function() {
        // Test that parse_result actually parses JSON correctly
        let json_value = serde_json::json!({
            "type": "string",
            "description": "test description",
            "key_name": "test_key",
            "inner": [],
            "optional": true
        });

        let result = parse_result(&json_value);
        assert_eq!(result.type_, "string");
        assert_eq!(result.description, "test description");
        assert_eq!(result.key_name, "test_key");
        assert_eq!(result.optional, true);
        assert!(result.inner.is_empty());
    }

    #[test]
    fn test_parse_result_with_inner_fields() {
        // Test parse_result with nested inner fields
        let json_value = serde_json::json!({
            "type": "object",
            "description": "parent",
            "key_name": "parent_key",
            "inner": [{
                "type": "string",
                "description": "child",
                "key_name": "child_key",
                "inner": [],
                "optional": false
            }],
            "optional": false
        });

        let result = parse_result(&json_value);
        assert_eq!(result.type_, "object");
        assert_eq!(result.key_name, "parent_key");
        assert_eq!(result.inner.len(), 1);
        assert_eq!(result.inner[0].type_, "string");
        assert_eq!(result.inner[0].key_name, "child_key");
    }

    #[test]
    fn test_from_api_results_complex_logic() {
        // Test the specific condition: result.type_ == "object" && !result.inner.is_empty()

        // Case 1: object type with non-empty inner (should create Object with fields)
        let result = ApiResult {
            key_name: "".into(),
            type_: "object".into(),
            description: "".into(),
            inner: vec![ApiResult {
                key_name: "field1".into(),
                type_: "string".into(),
                description: "".into(),
                inner: vec![],
                optional: false,
            }],
            optional: false,
        };
        let ty = Type::from_api_results(&[result]);
        if let Type::Object(fields) = ty {
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].0, "field1");
        } else {
            panic!("Expected Type::Object for object with non-empty inner");
        }

        // Case 2: object type with empty inner (should fall through to else)
        let result = ApiResult {
            key_name: "".into(),
            type_: "object".into(),
            description: "".into(),
            inner: vec![], // empty inner
            optional: false,
        };
        let ty = Type::from_api_results(&[result]);
        // Should call from_json_schema which returns Type::Object(vec![])
        assert!(matches!(ty, Type::Object(_)));
    }

    #[test]
    fn test_from_json_schema_each_arm_individually() {
        // Test each match arm individually to ensure they're not dead code

        // Test "string" arm
        let result = Type::from_json_schema("string", "test");
        assert!(matches!(result, Type::Primitive(s) if s == "string"));

        // Test "boolean" arm
        let result = Type::from_json_schema("boolean", "test");
        assert!(matches!(result, Type::Primitive(s) if s == "boolean"));

        // Test "number" arm
        let result = Type::from_json_schema("number", "test");
        assert!(matches!(result, Type::Primitive(s) if s == "number"));

        // Test "integer" arm
        let result = Type::from_json_schema("integer", "test");
        assert!(matches!(result, Type::Primitive(s) if s == "integer"));

        // Test "hex" arm
        let result = Type::from_json_schema("hex", "test");
        assert!(matches!(result, Type::Primitive(s) if s == "hex"));

        // Test "time" arm
        let result = Type::from_json_schema("time", "test");
        assert!(matches!(result, Type::Primitive(s) if s == "time"));

        // Test "amount" arm
        let result = Type::from_json_schema("amount", "test");
        assert!(matches!(result, Type::Primitive(s) if s == "amount"));

        // Test "array" arm
        let result = Type::from_json_schema("array", "test");
        assert!(matches!(result, Type::Array(_)));

        // Test "object" arm
        let result = Type::from_json_schema("object", "test");
        assert!(matches!(result, Type::Object(_)));
    }

    #[test]
    fn test_parse_result_not_default() {
        // Test that parse_result doesn't return default values
        let json_value = serde_json::json!({
            "type": "custom_type",
            "description": "custom description",
            "key_name": "custom_key",
            "inner": [{
                "type": "nested_type",
                "description": "nested",
                "key_name": "nested_key",
                "inner": [],
                "optional": true
            }],
            "optional": false
        });

        let result = parse_result(&json_value);

        // Verify it's not default values
        assert_ne!(result.type_, "");
        assert_ne!(result.description, "");
        assert_ne!(result.key_name, "");
        assert!(!result.inner.is_empty());
        assert_eq!(result.optional, false);

        // Verify nested parsing worked
        assert_eq!(result.inner[0].type_, "nested_type");
        assert_eq!(result.inner[0].key_name, "nested_key");
        assert_eq!(result.inner[0].optional, true);
    }
}
