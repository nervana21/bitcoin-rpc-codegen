//! Build response‑type structs (`…Response`) from `ApiMethod`s
//! and provide `TypesCodeGenerator` for the pipeline.

use rpc_api::{ApiMethod, ApiResult};
use std::fmt::Write as _;

/* --------------------------------------------------------------------- */
/*  Primitive → Rust helpers                                             */
/* --------------------------------------------------------------------- */

fn rust_ty(res: &ApiResult) -> (&'static str, bool /*is_option*/) {
    let base = match res.type_.as_str() {
        "string" | "hex" => "String",
        "number" | "amount" | "numeric" => "f64",
        "boolean" => "bool",
        "object" | "array" => "serde_json::Value",
        _ => "serde_json::Value",
    };
    (base, res.optional)
}

fn field_ident(res: &ApiResult, idx: usize) -> String {
    if !res.key_name.is_empty() {
        res.key_name.clone()
    } else {
        format!("field_{idx}")
    }
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

    // ----- Case 1: single object with `inner` fields ----------------------
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

    // ----- Case 2: multi‑field top level results -------------------------
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
        "use serde::{{Deserialize, Serialize}};\n\
         \n\
         /// Response for the `{}` RPC call.\n\
         #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]",
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
        methods
            .iter()
            .filter_map(|m| {
                let src = generate_return_type(m)?;
                Some((format!("{}_response", sanitize_method_name(&m.name)), src))
            })
            .collect()
    }
}
