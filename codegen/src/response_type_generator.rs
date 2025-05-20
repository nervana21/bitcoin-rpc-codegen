//! Build response‑type structs (`…Response`) from `ApiMethod`s
//! and provide `TypesCodeGenerator` for the pipeline.

use rpc_api::{ApiMethod, ApiResult};
use std::fmt::Write as _;

/* --------------------------------------------------------------------- */
/*  Primitive → Rust helpers                                             */
/* --------------------------------------------------------------------- */

fn rust_ty(res: &ApiResult) -> (&'static str, bool /*is_option*/) {
    // Map JSON‐RPC types & field names to more precise Rust types
    let base = match res.type_.as_str() {
        // Plain strings
        "string" => "String",

        // Bitcoin money values
        "amount" => "bitcoin::Amount",

        // Generic JSON numbers:
        // - use u64 for heights, sizes, timestamps
        // - otherwise treat as f64
        "number" | "numeric" => {
            let k = res.key_name.as_str();
            if k.ends_with("height")
                || k == "blocks"
                || k == "headers"
                || k.ends_with("time")
                || k.ends_with("size")
                || k.contains("count")
                || k.contains("index")
            {
                "u64"
            } else {
                "f64"
            }
        }

        // Booleans
        "boolean" => "bool",

        // Hex‑encoded values
        "hex" => {
            let k = res.key_name.as_str();
            if k.contains("txid") {
                "bitcoin::Txid"
            } else if k.contains("blockhash") {
                "bitcoin::BlockHash"
            } else if k.contains("script") {
                "bitcoin::ScriptBuf"
            } else if k.contains("pubkey") {
                "bitcoin::PublicKey"
            } else {
                "String"
            }
        }

        // Arrays: special‑case known vectors and warnings
        "array" => {
            let k = res.key_name.as_str();
            if k.contains("address") {
                "Vec<bitcoin::Address<bitcoin::address::NetworkUnchecked>>"
            } else if k.contains("txid") {
                "Vec<bitcoin::Txid>"
            } else if k.contains("blockhash") {
                "Vec<bitcoin::BlockHash>"
            } else if k.contains("script") {
                "Vec<bitcoin::ScriptBuf>"
            } else if k.contains("warning") || k.contains("error") || k.contains("message") {
                "Vec<String>"
            } else {
                "Vec<serde_json::Value>"
            }
        }

        // Nested objects
        "object" => {
            let k = res.key_name.as_str();
            if k.contains("transaction") {
                "bitcoin::Transaction"
            } else if k.contains("block") {
                "bitcoin::Block"
            } else {
                "serde_json::Value"
            }
        }

        // Fallback catch‑all
        _ => "serde_json::Value",
    };
    (base, res.optional)
}

fn field_ident(res: &ApiResult, idx: usize) -> String {
    if !res.key_name.is_empty() {
        // First convert hyphens to underscores
        let sanitized = res.key_name.replace('-', "_");

        // Handle all Rust keywords that need escaping
        let needs_escape = matches!(
            sanitized.as_str(),
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
            format!("r#{}", sanitized)
        } else {
            sanitized
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
    if method.results.is_empty() {
        println!("skip {}  → results empty", method.name);
        return None; // void return
    }

    // ----- Case 1: single object with `inner` fields ----------------------
    if method.results.len() == 1 && method.results[0].type_ == "object" {
        if method.results[0].inner.is_empty() {
            println!("skip {}  → single object but inner empty", method.name);
            return None;
        }
        return build_struct(method, &method.results[0].inner).or_else(|| {
            println!("skip {}  → build_struct(inner) returned None", method.name);
            None
        });
    }

    // ----- Case 2: multi‑field top level results -------------------------
    if method.results.iter().any(|r| !r.type_.eq("none")) {
        return build_struct(method, &method.results).or_else(|| {
            println!(
                "skip {}  → build_struct(top‑level) returned None",
                method.name
            );
            None
        });
    }

    println!("skip {}  → fell through", method.name);
    None
}

fn build_struct(method: &ApiMethod, fields: &[ApiResult]) -> Option<String> {
    let struct_name = capitalize(&method.name) + "Response";
    let mut out = String::new();

    // First analyze which types we actually need
    let mut needed_types = std::collections::HashSet::new();
    for res in fields {
        if res.type_ == "none" {
            continue;
        }
        match res.type_.as_str() {
            "amount" => {
                needed_types.insert("Amount");
            }
            "hex" => {
                let k = res.key_name.as_str();
                if k.contains("txid") {
                    needed_types.insert("Txid");
                } else if k.contains("blockhash") {
                    needed_types.insert("BlockHash");
                } else if k.contains("script") {
                    needed_types.insert("ScriptBuf");
                } else if k.contains("pubkey") {
                    needed_types.insert("PublicKey");
                }
            }
            _ => {}
        }
    }

    // Generate imports based on needed types
    writeln!(&mut out, "use serde::{{Deserialize, Serialize}};").ok()?;
    if !needed_types.is_empty() {
        let types: Vec<_> = needed_types.into_iter().collect();
        writeln!(&mut out, "use bitcoin::{{{}}};", types.join(", ")).ok()?;
    }
    writeln!(&mut out, "use bitcoin::address::NetworkUnchecked;\n").ok()?;

    writeln!(
        &mut out,
        "/// Response for the `{}` RPC call.\n#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]",
        method.name
    )
    .ok()?;
    writeln!(&mut out, "pub struct {} {{", struct_name).ok()?;

    for (idx, res) in fields.iter().enumerate() {
        if res.type_ == "none" {
            continue; // nothing to map
        }
        let (ty, is_opt) = rust_ty(res);
        let ident = field_ident(res, idx);

        if !res.description.is_empty() {
            let doc_comment = sanitize_doc_comment(&res.description);
            writeln!(&mut out, "    /// {}", doc_comment).ok()?;
        }

        if is_opt {
            writeln!(
                &mut out,
                "    #[serde(skip_serializing_if = \"Option::is_none\")]"
            )
            .ok()?;
            writeln!(&mut out, "    pub {ident}: Option<{ty}>,").ok()?;
        } else {
            writeln!(&mut out, "    pub {ident}: {ty},").ok()?;
        }
    }

    writeln!(&mut out, "}}\n").ok()?;
    Some(out)
}

/* --------------------------------------------------------------------- */
/*  Utils                                                                 */
/* --------------------------------------------------------------------- */
/// Capitalizes the first character of a string.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Sanitizes a method name for use as a filename.
pub fn sanitize_method_name(name: &str) -> String {
    name.to_string()
}

/* --------------------------------------------------------------------- */
/*  CodeGenerator impl                                                   */
/* --------------------------------------------------------------------- */

use crate::CodeGenerator;

/// Emits one `<method>_response.rs` file per RPC method.
pub struct TypesCodeGenerator;

impl CodeGenerator for TypesCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut files = Vec::new();

        // Generate individual response type files
        for m in methods {
            if let Some(src) = generate_return_type(m) {
                files.push((format!("{}_response", sanitize_method_name(&m.name)), src));
            }
        }

        // Generate mod.rs with re-exports
        let mut mod_rs = String::new();
        for m in methods {
            if !m.results.is_empty() {
                let mod_name = format!("{}_response", sanitize_method_name(&m.name));
                let type_name = format!("{}Response", capitalize(&m.name));
                writeln!(mod_rs, "pub mod {};", mod_name).unwrap();
                writeln!(mod_rs, "pub use {}::{};", mod_name, type_name).unwrap();
            }
        }
        files
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rpc_api::{ApiMethod, ApiResult};

    fn create_test_method(name: &str, results: Vec<ApiResult>) -> ApiMethod {
        ApiMethod {
            name: name.to_string(),
            description: format!("Test method {}", name),
            arguments: vec![],
            results,
        }
    }

    #[test]
    fn test_generate_return_type_basic() {
        let method = create_test_method(
            "test",
            vec![ApiResult {
                key_name: "value".to_string(),
                type_: "string".to_string(),
                description: "A test value".to_string(),
                optional: false,
                inner: vec![],
            }],
        );

        let result = generate_return_type(&method).unwrap();
        assert!(result.contains("pub struct TestResponse"));
        assert!(result.contains("pub value: String"));
        assert!(result.contains("use serde::{Deserialize, Serialize}"));
    }

    #[test]
    fn test_generate_return_type_empty() {
        let method = create_test_method("test", vec![]);
        let result = generate_return_type(&method);
        assert!(result.is_none());
    }

    #[test]
    fn test_types_code_generator_basic() {
        let methods = vec![
            create_test_method(
                "test1",
                vec![ApiResult {
                    key_name: "value".to_string(),
                    type_: "string".to_string(),
                    description: "A test value".to_string(),
                    optional: false,
                    inner: vec![],
                }],
            ),
            create_test_method(
                "test2",
                vec![ApiResult {
                    key_name: "value".to_string(),
                    type_: "string".to_string(),
                    description: "A test value".to_string(),
                    optional: false,
                    inner: vec![],
                }],
            ),
        ];

        let generator = TypesCodeGenerator;
        let files = generator.generate(&methods);

        // Check that we have the right number of files
        assert_eq!(files.len(), 2); // 2 response files

        // Check that we have the expected response files
        let file_names: Vec<_> = files.iter().map(|(name, _)| name).collect();
        assert!(file_names.contains(&&"test1_response".to_string()));
        assert!(file_names.contains(&&"test2_response".to_string()));
    }
}
