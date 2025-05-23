//! Build response‑type structs (`…Response`) from `ApiMethod`s
//! and provide `TypesCodeGenerator` for the pipeline.

use crate::TYPE_REGISTRY;
use rpc_api::{ApiMethod, ApiResult};
use std::fmt::Write as _;

/* --------------------------------------------------------------------- */
/*  Primitive → Rust helpers                                             */
/* --------------------------------------------------------------------- */

fn rust_ty(res: &ApiResult) -> (&'static str, bool /*is_option*/) {
    TYPE_REGISTRY.map_result_type(res)
}

/// Converts a camelCase string to snake_case
fn camel_to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
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
/// Capitalizes the first character of a string and converts snake_case/kebab-case to PascalCase.
pub fn capitalize(s: &str) -> String {
    s.split(|c| c == '_' || c == '-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
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
        let mut out = String::new();
        let mut used_types = std::collections::HashSet::new();
        let mut structs = Vec::new();

        // First pass: collect all structs and track used types
        for method in methods {
            if let Some(struct_def) = generate_return_type(method) {
                // Look for actual type names in the struct definition
                for line in struct_def.lines() {
                    if line.contains(": Amount") || line.contains(": Option<Amount>") {
                        used_types.insert("Amount");
                    }
                    if line.contains(": Txid") || line.contains(": Option<Txid>") {
                        used_types.insert("Txid");
                    }
                    if line.contains(": BlockHash") || line.contains(": Option<BlockHash>") {
                        used_types.insert("BlockHash");
                    }
                    if line.contains(": ScriptBuf") || line.contains(": Option<ScriptBuf>") {
                        used_types.insert("ScriptBuf");
                    }
                    if line.contains(": PublicKey") || line.contains(": Option<PublicKey>") {
                        used_types.insert("PublicKey");
                    }
                }

                // Store the struct without its imports
                let struct_def = struct_def
                    .lines()
                    .filter(|line| !line.contains("use "))
                    .collect::<Vec<_>>()
                    .join("\n");
                structs.push(struct_def);
            }
        }

        // Add imports at the top
        writeln!(&mut out, "use serde::{{Deserialize, Serialize}};").unwrap();
        if !used_types.is_empty() {
            let types: Vec<_> = used_types.into_iter().collect();
            writeln!(&mut out, "use bitcoin::{{{}}};", types.join(", ")).unwrap();
        }
        writeln!(&mut out).unwrap();

        // Add all structs
        for struct_def in structs {
            writeln!(&mut out, "{}", struct_def).unwrap();
        }

        vec![("latest_types".to_string(), out)]
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
        assert_eq!(files.len(), 1); // 1 response file

        // Check that we have the expected response file
        let file_names: Vec<_> = files.iter().map(|(name, _)| name).collect();
        assert!(file_names.contains(&&"latest_types".to_string()));
    }
}
