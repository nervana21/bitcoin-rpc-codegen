//! Decision tree for RPC return types:
//! 1. No results     → skip
//! 2. Single object  → struct with its fields
//! 3. Multi-variant  → enum or transparent wrapper
//!
//! Extracts fields in one pass, centralizes serde attrs, and names
//! things consistently.  

use crate::utils::{camel_to_snake_case, capitalize};
use crate::TYPE_REGISTRY;
use anyhow::Result;
use rpc_api::{ApiMethod, ApiResult};
use std::fmt::Write as _;

/* --------------------------------------------------------------------- */
/*  Primitive → Rust helpers                                             */
/* --------------------------------------------------------------------- */

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
/// This struct is responsible for generating a single Rust source file, `latest_types.rs`,
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
                    let (ty, opt) = TYPE_REGISTRY.map_result_type(f);
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
                let (ty, _) = TYPE_REGISTRY.map_result_type(r);
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
                    let (ty, _) = TYPE_REGISTRY.map_result_type(f);
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
                .any(|f| field_ident(f, 0) == name && !f.optional)
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
    if r.optional {
        "#[serde(skip_serializing_if = \"Option::is_none\")]\n    ".into()
    } else {
        "".into()
    }
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
    fn test_build_return_type() {
        // Test void method (no results)
        let void_method = create_test_method("void", vec![]);
        assert!(build_return_type(&void_method).unwrap().is_none());

        // Test void method (all results are "none")
        let void_method = create_test_method(
            "void",
            vec![ApiResult {
                type_: "none".to_string(),
                ..Default::default()
            }],
        );
        assert!(build_return_type(&void_method).unwrap().is_none());

        // Test single primitive result
        let primitive_method = create_test_method(
            "primitive",
            vec![ApiResult {
                type_: "string".to_string(),
                key_name: "result".to_string(),
                ..Default::default()
            }],
        );
        let result = build_return_type(&primitive_method).unwrap().unwrap();
        assert!(result.contains("pub struct PrimitiveResponse"));
        assert!(result.contains("pub String"));

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
        let result = build_return_type(&object_method).unwrap().unwrap();
        assert!(result.contains("pub struct ObjectResponse"));
        assert!(result.contains("pub field1: String"));
        assert!(result.contains("pub difficulty: f64"));

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
        let result = build_return_type(&multi_method).unwrap().unwrap();
        assert!(result.contains("pub struct MultiResponse"));
        assert!(result.contains("pub field1: Option<String>"));
        assert!(result.contains("pub difficulty: Option<f64>"));
    }
}
