//! Decision tree for RPC return types:
//! 1. No results     → skip
//! 2. Single object  → struct with its fields
//! 3. Multi-variant  → enum or transparent wrapper
//!
//! Extracts fields in one pass, centralizes serde attrs, and names
//! things consistently.  

use crate::utils::{camel_to_snake_case, capitalize};
use anyhow::Result;
use rpc_api::{ApiMethod, ApiResult};
use std::fmt::Write as _;
use type_registry::TypeRegistry;

/* --------------------------------------------------------------------- */
/*  Primitive → Rust helpers                                             */
/* --------------------------------------------------------------------- */

fn field_ident(res: &ApiResult, idx: usize) -> String {
    if !res.key_name.is_empty() {
        // Remove angle brackets and other invalid characters for Rust identifiers
        let sanitized = res.key_name.replace(['<', '>'], "").replace('-', "_");

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
            format!("r#{snake_case}")
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

/// Code generator for producing type-safe Rust representations of Bitcoin RPC responses.
///
/// This struct is responsible for generating a single Rust source file, `{version}_types.rs`,
/// which defines one strongly-typed struct per RPC method (e.g., `GetBlockResponse`).
/// Each struct mirrors the shape of that method's return value, supporting:
/// - Primitive types (e.g., `u64`, `String`)
/// - Structs for nested objects
/// - Enums for union/multi-variant outputs
/// - Vectors for arrays
/// - `Option<T>` for nullable or conditionally present fields
///
/// All generated types are annotated for use with `serde` to support automatic (de)serialization.
/// This ensures correctness, improves developer ergonomics, and enables compile-time validation
/// of response structures across Bitcoin Core versions.
///
/// Intended to be used as part of a version-aware code generation pipeline.
pub struct ResponseTypeCodeGenerator {
    version: String,
}

impl ResponseTypeCodeGenerator {
    /// Creates a new `ResponseTypeCodeGenerator` for a specific Bitcoin Core RPC version.
    ///
    /// The provided `version` string is used to namespace or suffix generated types,
    /// ensuring compatibility with different versions of the RPC interface.
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
        }
    }
}

impl crate::CodeGenerator for ResponseTypeCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut out = String::from(
            "//! Generated RPC response types\n\
             use serde::{Deserialize, Serialize};\n\n",
        );

        for m in methods {
            let response_struct = build_return_type(m).unwrap_or_default();
            if let Some(def) = response_struct {
                out.push_str(&def);
                out.push('\n');
            }
        }

        vec![(format!("{}_types.rs", self.version.to_lowercase()), out)]
    }
}

/// Build a single response type, or return `Ok(None)` to skip.
pub fn build_return_type(method: &ApiMethod) -> Result<Option<String>> {
    if is_void(method) {
        return Ok(None);
    }

    let struct_name = response_struct_name(method);
    let mut buf = String::new();

    let doc = sanitize_doc_comment(&method.description);
    writeln!(&mut buf, "/// {doc}")?;
    writeln!(&mut buf, "#[derive(Debug, Deserialize, Serialize)]")?;

    if is_multi_variant(method) {
        // multiple object shapes or primitives → flattened struct with optional fields
        writeln!(&mut buf, "pub struct {struct_name} {{")?;
        for field in collect_fields(method) {
            let ty = if field.always_present {
                field.ty.clone()
            } else {
                format!("Option<{}>", field.ty)
            };
            writeln!(
                &mut buf,
                "    {}pub {}: {},",
                serde_attrs_for(&field),
                field.name,
                ty
            )?;
        }
        writeln!(&mut buf, "}}\n")?;
    } else {
        // single result type
        let r = &method.results[0];
        match &r.type_[..] {
            "object" if !r.inner.is_empty() => {
                writeln!(&mut buf, "pub struct {struct_name} {{")?;
                for f in &r.inner {
                    let (ty, opt) = TypeRegistry.map_result_type(f);
                    let name = field_ident(f, 0);
                    let ty = if opt {
                        format!("Option<{ty}>")
                    } else {
                        ty.to_string()
                    };
                    writeln!(
                        &mut buf,
                        "    {}pub {}: {},",
                        serde_attrs_for_field(f),
                        name,
                        ty
                    )?;
                }
                writeln!(&mut buf, "}}\n")?;
            }
            _ => {
                // primitive or array → transparent wrapper
                let (ty, _) = TypeRegistry.map_result_type(r);
                writeln!(&mut buf, "#[serde(transparent)]")?;
                writeln!(&mut buf, "pub struct {struct_name}(pub {ty});\n")?;
            }
        }
    }

    Ok(Some(buf))
}

// Helpers

/// Void = no results or all `type == "none"`.
fn is_void(m: &ApiMethod) -> bool {
    m.results.is_empty() || m.results.iter().all(|r| r.type_ == "none")
}

/// Multi-variant = more than one non‐none result.
fn is_multi_variant(m: &ApiMethod) -> bool {
    m.results.iter().filter(|r| r.type_ != "none").count() > 1
}

/// Name for both struct and file.
fn response_struct_name(m: &ApiMethod) -> String {
    format!("{}Response", capitalize(&m.name))
}

/// Gather every possible field exactly once, preserving order.
fn collect_fields(m: &ApiMethod) -> Vec<Field> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();

    for r in &m.results {
        if r.type_ == "object" {
            for f in &r.inner {
                let name = field_ident(f, 0);
                if seen.insert(name.clone()) {
                    let (ty, _) = TypeRegistry.map_result_type(f);
                    let always = is_field_always_present(&name, &m.results);
                    out.push(Field {
                        name,
                        ty: ty.to_string(),
                        always_present: always,
                    });
                }
            }
        }
    }

    out
}

/// Single field info.
struct Field {
    name: String,
    ty: String,
    always_present: bool,
}

/// Decide if a field is never optional.
fn is_field_always_present(name: &str, results: &[ApiResult]) -> bool {
    results.iter().all(|r| {
        r.type_ == "object"
            && r.inner
                .iter()
                .any(|f| field_ident(f, 0) == name && f.required)
    })
}

/// Render serde attributes for a flattened multi-variant struct field.
fn serde_attrs_for(field: &Field) -> String {
    if !field.always_present {
        "    #[serde(skip_serializing_if = \"Option::is_none\")]\n    ".into()
    } else {
        "".into()
    }
}

/// Render serde attrs for a single `ApiResult`.
fn serde_attrs_for_field(r: &ApiResult) -> String {
    if !r.required {
        "#[serde(skip_serializing_if = \"Option::is_none\")]\n    ".into()
    } else {
        "".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CodeGenerator;
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
    fn test_response_type_generator_generate() {
        let generator = ResponseTypeCodeGenerator::new("v28");

        // Test with empty methods
        let result = generator.generate(&[]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "v28_types.rs");
        assert!(result[0].1.contains("//! Generated RPC response types"));
        assert!(result[0].1.contains("use serde::{Deserialize, Serialize}"));

        // Test with void methods (should be skipped)
        let void_method = create_test_method("void", vec![]);
        let result = generator.generate(&[void_method]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "v28_types.rs");
        assert!(result[0].1.contains("//! Generated RPC response types"));
        assert!(!result[0].1.contains("pub struct VoidResponse")); // Should not generate struct for void method

        // Test with single primitive result
        let primitive_method = create_test_method(
            "primitive",
            vec![ApiResult {
                type_: "string".to_string(),
                key_name: "result".to_string(),
                ..Default::default()
            }],
        );
        let result = generator.generate(&[primitive_method]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "v28_types.rs");
        assert!(result[0].1.contains("pub struct PrimitiveResponse"));
        assert!(result[0].1.contains("pub String"));

        // Test with multiple methods
        let methods = vec![
            create_test_method(
                "method1",
                vec![ApiResult {
                    type_: "string".to_string(),
                    key_name: "result".to_string(),
                    ..Default::default()
                }],
            ),
            create_test_method(
                "method2",
                vec![ApiResult {
                    type_: "number".to_string(),
                    key_name: "result".to_string(),
                    ..Default::default()
                }],
            ),
        ];
        let result = generator.generate(&methods);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "v28_types.rs");
        assert!(result[0].1.contains("pub struct Method1Response"));
        assert!(result[0].1.contains("pub struct Method2Response"));
        assert!(result[0].1.contains("pub String"));
        assert!(result[0].1.contains("pub u64"));
    }

    #[test]
    fn test_response_type_generator_version_handling() {
        // Test different version strings
        let generator_v28 = ResponseTypeCodeGenerator::new("v28");
        let generator_v29 = ResponseTypeCodeGenerator::new("V29");

        let method = create_test_method(
            "test",
            vec![ApiResult {
                type_: "string".to_string(),
                key_name: "result".to_string(),
                ..Default::default()
            }],
        );

        let result_v28 = generator_v28.generate(std::slice::from_ref(&method));
        let result_v29 = generator_v29.generate(&[method]);

        assert_eq!(result_v28[0].0, "v28_types.rs");
        assert_eq!(result_v29[0].0, "v29_types.rs");
    }

    #[test]
    fn test_response_type_generator_mixed_methods() {
        let generator = ResponseTypeCodeGenerator::new("test");

        let methods = vec![
            // Void method (should be skipped)
            create_test_method("void", vec![]),
            // Primitive method (should generate struct)
            create_test_method(
                "primitive",
                vec![ApiResult {
                    type_: "string".to_string(),
                    key_name: "result".to_string(),
                    ..Default::default()
                }],
            ),
            // Object method (should generate struct)
            create_test_method(
                "object",
                vec![ApiResult {
                    type_: "object".to_string(),
                    key_name: "result".to_string(),
                    inner: vec![ApiResult {
                        type_: "string".to_string(),
                        key_name: "field".to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
            ),
        ];

        let result = generator.generate(&methods);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "test_types.rs");

        let output = &result[0].1;
        // Should not contain void method
        assert!(!output.contains("pub struct VoidResponse"));
        // Should contain other methods
        assert!(output.contains("pub struct PrimitiveResponse"));
        assert!(output.contains("pub struct ObjectResponse"));
        assert!(output.contains("pub field: Option<String>"));
    }

    #[test]
    fn test_build_return_type() {
        // Test void method (no results)
        let void_method = create_test_method("void", vec![]);
        let void_result = build_return_type(&void_method);
        assert!(
            void_result.is_ok(),
            "build_return_type should succeed for void method"
        );
        assert!(
            void_result.unwrap().is_none(),
            "Void method should return None"
        );

        // Test void method (all results are "none")
        let void_method_with_none = create_test_method(
            "void",
            vec![ApiResult {
                type_: "none".to_string(),
                ..Default::default()
            }],
        );
        let void_none_result = build_return_type(&void_method_with_none);
        assert!(
            void_none_result.is_ok(),
            "build_return_type should succeed for void method with none"
        );
        assert!(
            void_none_result.unwrap().is_none(),
            "Void method with none should return None"
        );

        // Test single primitive result
        let primitive_method = create_test_method(
            "primitive",
            vec![ApiResult {
                type_: "string".to_string(),
                key_name: "result".to_string(),
                ..Default::default()
            }],
        );
        let primitive_result = build_return_type(&primitive_method);
        assert!(
            primitive_result.is_ok(),
            "build_return_type should succeed for primitive method"
        );
        let primitive_code = primitive_result.unwrap().unwrap();
        assert!(
            primitive_code.contains("pub struct PrimitiveResponse"),
            "Primitive response should contain 'pub struct PrimitiveResponse'"
        );
        assert!(
            primitive_code.contains("pub String"),
            "Primitive response should contain 'pub String'"
        );

        // Test single object result
        let object_method = create_test_method(
            "object",
            vec![ApiResult {
                type_: "object".to_string(),
                key_name: "result".to_string(),
                inner: vec![
                    ApiResult {
                        type_: "string".to_string(),
                        key_name: "field1".to_string(),
                        ..Default::default()
                    },
                    ApiResult {
                        type_: "number".to_string(),
                        key_name: "difficulty".to_string(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
        );
        let object_result = build_return_type(&object_method);
        assert!(
            object_result.is_ok(),
            "build_return_type should succeed for object method"
        );
        let object_code = object_result.unwrap().unwrap();
        assert!(
            object_code.contains("pub struct ObjectResponse"),
            "Object response should contain 'pub struct ObjectResponse'"
        );
        assert!(
            object_code.contains("pub field1: Option<String>"),
            "Object response should contain 'pub field1: Option<String>'"
        );
        assert!(
            object_code.contains("pub difficulty: Option<f64>"),
            "Object response should contain 'pub difficulty: Option<f64>'"
        );

        // Test multi-variant result
        let multi_method = create_test_method(
            "multi",
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
                        type_: "number".to_string(),
                        key_name: "difficulty".to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
        );
        let multi_result = build_return_type(&multi_method);
        assert!(
            multi_result.is_ok(),
            "build_return_type should succeed for multi-variant method"
        );
        let multi_code = multi_result.unwrap().unwrap();
        assert!(
            multi_code.contains("pub struct MultiResponse"),
            "Multi-variant response should contain 'pub struct MultiResponse'"
        );
        assert!(
            multi_code.contains("pub field1: Option<String>"),
            "Multi-variant response should contain 'pub field1: Option<String>'"
        );
        assert!(
            multi_code.contains("pub difficulty: Option<f64>"),
            "Multi-variant response should contain 'pub difficulty: Option<f64>'"
        );
    }

    #[test]
    fn test_sanitize_doc_comment() {
        // Test basic sanitization
        let input = "Test comment";
        let result = sanitize_doc_comment(input);
        assert_eq!(result, "Test comment");

        // Test with special characters
        let input = "Test \"quoted\" comment with \\backslashes\\";
        let result = sanitize_doc_comment(input);
        assert!(result.contains("\\\"quoted\\\""));
        assert!(result.contains("\\\\backslashes\\\\"));

        // Test with newlines
        let input = "Line 1\nLine 2\nLine 3";
        let result = sanitize_doc_comment(input);
        assert!(result.contains("Line 1"));
        assert!(result.contains("Line 2"));
        assert!(result.contains("Line 3"));
        assert!(result.contains("/// "));
    }

    #[test]
    fn test_field_ident() {
        // Test basic field name
        let result = ApiResult {
            key_name: "test_field".to_string(),
            ..Default::default()
        };
        assert_eq!(field_ident(&result, 0), "test_field");

        // Test camelCase conversion
        let result = ApiResult {
            key_name: "testField".to_string(),
            ..Default::default()
        };
        assert_eq!(field_ident(&result, 0), "test_field");

        // Test with hyphens
        let result = ApiResult {
            key_name: "test-field".to_string(),
            ..Default::default()
        };
        assert_eq!(field_ident(&result, 0), "test_field");

        // Test empty key_name (should use index)
        let result = ApiResult {
            key_name: "".to_string(),
            ..Default::default()
        };
        assert_eq!(field_ident(&result, 21), "field_21");

        // Test Rust keyword (should be escaped)
        let result = ApiResult {
            key_name: "type".to_string(),
            ..Default::default()
        };
        assert_eq!(field_ident(&result, 0), "r#type");
    }

    #[test]
    fn test_serde_attrs_for() {
        // Test field that is always present (should return empty string)
        let field = Field {
            name: "test_field".to_string(),
            ty: "String".to_string(),
            always_present: true,
        };
        assert_eq!(serde_attrs_for(&field), "");

        // Test field that is not always present (should return serde attribute)
        let field = Field {
            name: "test_field".to_string(),
            ty: "String".to_string(),
            always_present: false,
        };
        let result = serde_attrs_for(&field);
        assert!(result.contains("#[serde(skip_serializing_if = \"Option::is_none\")]"));
        assert!(result.contains("    ")); // Should include indentation
    }

    #[test]
    fn test_serde_attrs_for_field() {
        // Test non-optional field (should return empty string)
        let result = ApiResult {
            key_name: "test_field".to_string(),
            type_: "string".to_string(),
            description: "Test field".to_string(),
            inner: vec![],
            required: true,
        };
        assert_eq!(serde_attrs_for_field(&result), "");

        // Test optional field (should return serde attribute)
        let result = ApiResult {
            key_name: "test_field".to_string(),
            type_: "string".to_string(),
            description: "Test field".to_string(),
            inner: vec![],
            required: false,
        };
        let output = serde_attrs_for_field(&result);
        assert!(output.contains("#[serde(skip_serializing_if = \"Option::is_none\")]"));
        assert!(output.contains("    ")); // Should include indentation
    }

    #[test]
    fn test_is_field_always_present() {
        // Test field that appears in all object results and is never optional
        let results = vec![
            ApiResult {
                key_name: "result1".to_string(),
                type_: "object".to_string(),
                description: "First result".to_string(),
                inner: vec![ApiResult {
                    key_name: "test_field".to_string(),
                    type_: "string".to_string(),
                    description: "Test field".to_string(),
                    inner: vec![],
                    required: true,
                }],
                required: true,
            },
            ApiResult {
                key_name: "result2".to_string(),
                type_: "object".to_string(),
                description: "Second result".to_string(),
                inner: vec![ApiResult {
                    key_name: "test_field".to_string(),
                    type_: "string".to_string(),
                    description: "Test field".to_string(),
                    inner: vec![],
                    required: true,
                }],
                required: true,
            },
        ];
        assert!(is_field_always_present("test_field", &results));

        // Test field that appears in all object results but is optional in one
        let results = vec![
            ApiResult {
                key_name: "result1".to_string(),
                type_: "object".to_string(),
                description: "First result".to_string(),
                inner: vec![ApiResult {
                    key_name: "test_field".to_string(),
                    type_: "string".to_string(),
                    description: "Test field".to_string(),
                    inner: vec![],
                    required: true,
                }],
                required: true,
            },
            ApiResult {
                key_name: "result2".to_string(),
                type_: "object".to_string(),
                description: "Second result".to_string(),
                inner: vec![ApiResult {
                    key_name: "test_field".to_string(),
                    type_: "string".to_string(),
                    description: "Test field".to_string(),
                    inner: vec![],
                    required: false, // This makes it not always present
                }],
                required: true,
            },
        ];
        assert!(!is_field_always_present("test_field", &results));

        // Test field that doesn't appear in all results
        let results = vec![
            ApiResult {
                key_name: "result1".to_string(),
                type_: "object".to_string(),
                description: "First result".to_string(),
                inner: vec![ApiResult {
                    key_name: "test_field".to_string(),
                    type_: "string".to_string(),
                    description: "Test field".to_string(),
                    inner: vec![],
                    required: true,
                }],
                required: true,
            },
            ApiResult {
                key_name: "result2".to_string(),
                type_: "object".to_string(),
                description: "Second result".to_string(),
                inner: vec![], // No test_field here
                required: true,
            },
        ];
        assert!(!is_field_always_present("test_field", &results));

        // Test field that appears in non-object results (should be false)
        let results = vec![ApiResult {
            key_name: "result1".to_string(),
            type_: "string".to_string(), // Not an object
            description: "First result".to_string(),
            inner: vec![],
            required: true,
        }];
        assert!(!is_field_always_present("test_field", &results));
    }

    #[test]
    fn test_build_return_type_with_empty_inner() {
        // Test object with empty inner array (should hit the match guard)
        let method = create_test_method(
            "empty_object",
            vec![ApiResult {
                type_: "object".to_string(),
                key_name: "result".to_string(),
                description: "Empty object".to_string(),
                inner: vec![], // Empty inner array
                required: true,
            }],
        );
        let result = build_return_type(&method).unwrap().unwrap();
        // Should generate transparent wrapper, not struct
        assert!(result.contains("#[serde(transparent)]"));
        assert!(result.contains("pub struct EmptyObjectResponse"));
        assert!(!result.contains("pub struct EmptyObjectResponse {")); // Should not be a struct with fields
    }
}
