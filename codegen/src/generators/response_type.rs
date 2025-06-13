//! Build response‑type structs (`…Result`) from `ApiMethod`s
//! and provide `TypesCodeGenerator` for the pipeline.

use crate::utils::{camel_to_snake_case, capitalize};
use crate::CodeGenerator;
use crate::TYPE_REGISTRY;
use rpc_api::{ApiMethod, ApiResult};
use std::fmt::Write as _;

/* --------------------------------------------------------------------- */
/*  Primitive → Rust helpers                                             */
/* --------------------------------------------------------------------- */

fn rust_ty(res: &ApiResult) -> (&'static str, bool /*is_option*/) {
    TYPE_REGISTRY.map_result_type(res)
}

fn field_ident(res: &ApiResult, idx: usize) -> String {
    if !res.key_name.is_empty() {
        // First convert hyphens to underscores
        let sanitized = res.key_name.replace('-', "_");

        // Convert camelCase to snake_case
        let snake_case = camel_to_snake_case(&sanitized);

        // Handle all Rust keywords that need escaping
        let needs_escape = matches!(
            snake_case.as_str(),
            "type"
                | "const"
                | "static"
                | "struct"
                | "enum"
                | "trait"
                | "impl"
                | "fn"
                | "let"
                | "mut"
                | "ref"
                | "self"
                | "Self"
                | "super"
                | "crate"
                | "extern"
                | "use"
                | "where"
                | "async"
                | "await"
                | "break"
                | "continue"
                | "else"
                | "if"
                | "loop"
                | "match"
                | "return"
                | "while"
                | "for"
                | "in"
                | "move"
                | "yield"
                | "dyn"
                | "unsafe"
                | "union"
        );

        if needs_escape {
            format!("r#{}", snake_case)
        } else {
            snake_case
        }
    } else {
        format!("field_{idx}")
    }
}

fn sanitize_doc_comment(comment: &str) -> String {
    comment
        .lines()
        .map(|line| {
            // Escape any special characters in doc comments
            line.replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\n", " ")
                .trim()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("\n    /// ")
}

/* --------------------------------------------------------------------- */
/*  Struct generators                                                    */
/* --------------------------------------------------------------------- */

/// Generates a Rust struct type for the RPC method's return value.
/// Returns None if the method has no results or if the results can't be mapped to a struct.
pub fn generate_return_type(method: &ApiMethod) -> Option<String> {
    // Skip if no results
    if method.results.is_empty() {
        return None;
    }

    let struct_name = format!("{}Response", capitalize(&method.name));
    let mut out = String::new();

    // Add doc comment
    writeln!(
        &mut out,
        "/// {}",
        sanitize_doc_comment(&method.description)
    )
    .unwrap();
    writeln!(&mut out, "#[derive(Debug, Deserialize, Serialize)]").unwrap();

    // If we have multiple result types, we need to handle all cases
    if method.results.len() > 1 {
        // Find all possible fields across all result types
        let mut all_fields = std::collections::HashMap::new();
        for result in &method.results {
            if result.type_ == "object" && !result.inner.is_empty() {
                for field in &result.inner {
                    let field_name = field_ident(field, 0);
                    let (ty, _) = rust_ty(field);
                    // If field exists in any result type, it should be optional
                    all_fields.insert(field_name, (ty, true));
                }
            } else {
                // For non-object types, create a transparent wrapper
                let (ty, _) = rust_ty(result);
                writeln!(&mut out, "#[serde(transparent)]").unwrap();
                writeln!(&mut out, "pub struct {}(pub {});", struct_name, ty).unwrap();
                return Some(out);
            }
        }

        // Generate struct with all possible fields as optional
        writeln!(&mut out, "pub struct {} {{", struct_name).unwrap();
        for (field_name, (ty, _)) in all_fields {
            let is_optional = !is_field_always_present(&field_name, &method.results);
            let field_type = if is_optional {
                format!("Option<{}>", ty)
            } else {
                ty.to_string()
            };
            writeln!(&mut out, "    pub {}: {},", field_name, field_type).unwrap();
        }
        writeln!(&mut out, "}}").unwrap();

        if method.results.iter().any(|r| r.type_ == "array") {
            // If any result is an array, we should document this
            writeln!(&mut out, "/// This response can be either an array or an object depending on the input parameters.").unwrap();
        }
    } else {
        // Single result type - use existing logic
        let r = &method.results[0];
        if r.type_ == "object" && !r.inner.is_empty() {
            writeln!(&mut out, "pub struct {} {{", struct_name).unwrap();
            for field in &r.inner {
                let field_name = field_ident(field, 0);
                let (ty, is_optional) = rust_ty(field);
                let field_type = if is_optional {
                    format!("Option<{}>", ty)
                } else {
                    ty.to_string()
                };
                writeln!(&mut out, "    pub {}: {},", field_name, field_type).unwrap();
            }
            writeln!(&mut out, "}}").unwrap();
        } else {
            let (ty, _) = rust_ty(r);
            writeln!(&mut out, "#[serde(transparent)]").unwrap();
            writeln!(&mut out, "pub struct {}(pub {});", struct_name, ty).unwrap();
        }
    }

    Some(out)
}

/// Generates a single Rust source file (`latest_types.rs`) that defines
/// strongly-typed response structs (`<MethodName>Response`) for every RPC method,
/// including support for primitive, object, array, and multi-variant results,
/// with serde (de)serialization and optional fields as needed.
pub struct ResponseTypeCodeGenerator;

impl CodeGenerator for ResponseTypeCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut out = String::from(
            "//! Result structs for RPC method returns\n\
             use serde::Deserialize;\n\
             use serde::Serialize;\n\
             use serde::de::DeserializeOwned;\n\n",
        );

        for m in methods {
            if let Some(struct_def) = generate_return_type(m) {
                out.push_str(&struct_def);
            }
        }

        vec![("latest_types.rs".to_string(), out)]
    }
}

fn is_field_always_present(field_name: &str, results: &[ApiResult]) -> bool {
    results.iter().all(|r| {
        r.type_ == "object"
            && r.inner
                .iter()
                .any(|f| field_ident(f, 0) == field_name && !f.optional)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rpc_api::ApiResult;

    fn create_test_method(name: &str, results: Vec<ApiResult>) -> ApiMethod {
        ApiMethod {
            name: name.to_string(),
            description: "Test method".to_string(),
            results,
            ..Default::default()
        }
    }

    #[test]
    fn test_generate_return_type_basic() {
        let method = create_test_method(
            "test",
            vec![ApiResult {
                type_: "string".to_string(),
                key_name: "result".to_string(),
                ..Default::default()
            }],
        );

        let result = generate_return_type(&method).unwrap();
        assert!(result.contains("pub struct TestResponse"));
        assert!(result.contains("pub String"));
    }

    #[test]
    fn test_generate_return_type_empty() {
        let method = create_test_method("test", vec![]);
        assert!(generate_return_type(&method).is_none());
    }

    #[test]
    fn test_types_code_generator_basic() {
        let methods = vec![create_test_method(
            "test",
            vec![ApiResult {
                type_: "string".to_string(),
                key_name: "result".to_string(),
                ..Default::default()
            }],
        )];

        let generator = ResponseTypeCodeGenerator;
        let files = generator.generate(&methods);
        assert_eq!(files.len(), 1);
        assert!(files[0].0 == "latest_types.rs");
        assert!(files[0].1.contains("pub struct TestResponse"));
    }

    #[test]
    fn test_generate_return_type_multiple_results() {
        let method = create_test_method(
            "test",
            vec![
                ApiResult {
                    type_: "object".to_string(),
                    key_name: "result1".to_string(),
                    inner: vec![ApiResult {
                        type_: "string".to_string(),
                        key_name: "field1".to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                ApiResult {
                    type_: "object".to_string(),
                    key_name: "result2".to_string(),
                    inner: vec![ApiResult {
                        type_: "string".to_string(),
                        key_name: "field2".to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
        );

        let result = generate_return_type(&method).unwrap();
        assert!(result.contains("pub struct TestResponse"));
        assert!(result.contains("pub field1: Option<String>"));
        assert!(result.contains("pub field2: Option<String>"));
    }
}
