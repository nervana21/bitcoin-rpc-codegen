//! # RPC‑to‑Rust Macro Generator
//!
//! `generate_client_macro` turns **one** `ApiMethod` + a version tag  
//! (e.g. `"v28"`) ➜ a `macro_rules! impl_client_v28__<method>()` string.
//!
//! **What you get**
//! ```rust,ignore
//! impl_client_v29__getblockchaininfo!();   // zero‑param example
//! impl_client_v29__listunspent!();         // auto‑adds *_default & typed wrapper
//! impl_client_v29__invalidateblock!();     // wraps legit `null` → `()`
//! ```
//!
//! Each macro expands to an `impl Client { … }` block containing a strongly‑typed
//! wrapper that builds the param list, calls `self.call`, and converts the result
//! into the right Rust type—no boilerplate on your side.
//!
//! **Why bother?**  
//! * Centralises all string‑munging; the rest of code‑gen stays data‑driven.  
//! * Pure function: emit the string, write it wherever you like.

use crate::generators::doc_comment::format_doc_comment;
use crate::utils::capitalize;
use rpc_api::ApiMethod;
use type_registry::TypeRegistry;

/// Generates a client-side macro implementation for an RPC method
pub fn generate_client_macro(method: &ApiMethod, version: &str) -> String {
    let method_name = &method.name;
    let macro_name = format!("impl_client_{version}__{method_name}");
    let description = format_doc_comment(&method.description);
    let mut function_defs = Vec::new();

    // Compute return type once
    let return_ty = get_return_type_from_results(&method.results);

    // Helper to format the call expression with correct empty‑slice cast
    let make_call = |req: &str, opt: &str| {
        if req.is_empty() && opt.is_empty() {
            // no required, no optional
            format!(
                "self.call(\"{}\", &[] as &[serde_json::Value])",
                method.name
            )
        } else if !opt.is_empty() {
            // required + optional
            format!(
                "let mut params = vec![{}];\n{}\n    self.call(\"{}\", &params)",
                req, opt, method.name
            )
        } else {
            // only required
            format!("self.call(\"{}\", &[{}])", method.name, req)
        }
    };

    if !method.arguments.is_empty() && method.arguments.iter().all(|arg| arg.optional) {
        // 1) default‐params variant
        let default_doc = format!("{description} with default parameters.");
        let default_fn = format!(
            "/// {doc}\npub fn {name}_default(&self) -> Result<{ret}> {{
    {call}
}}",
            doc = default_doc,
            name = method_name,
            ret = return_ty,
            call = make_call("", "")
        );

        // 2) specified‐params variant
        let param_doc = format!("{description} with specified parameters.");
        let method_args = generate_method_args(method);
        let (required_args, optional_body_raw) = generate_args(method);
        let optional_body = optional_body_raw
            .lines()
            .map(|line| format!("    {line}"))
            .collect::<Vec<_>>()
            .join("\n");
        let param_call = make_call(&required_args, &optional_body);
        let param_fn = format!(
            "/// {param_doc}\npub fn {method_name}(&self{method_args}) -> Result<{return_ty}> {{
    {param_call}
}}"
        );

        function_defs.push(default_fn);
        function_defs.push(param_fn);
    } else {
        // Single‐method variant
        let method_args = generate_method_args(method);
        let (required_args, optional_body_raw) = generate_args(method);
        let optional_body = optional_body_raw
            .lines()
            .map(|line| format!("    {line}"))
            .collect::<Vec<_>>()
            .join("\n");
        let raw_call = make_call(&required_args, &optional_body);

        // If this RPC returns `null`, wrap it in a match
        let call_body = if method
            .results
            .iter()
            .any(|r| r.type_.eq_ignore_ascii_case("none"))
        {
            format!(
                "match {call} {{
    Ok(serde_json::Value::Null) => Ok(()),
    Ok(ref val) if val.is_null() => Ok(()),
    Ok(other) => Err(crate::client_sync::Error::Returned(format!(\"{method} expected null, got: {{}}\", other))),
    Err(e) => Err(e.into()),
}}",
                call = raw_call,
                method = method.name
            )
        } else {
            raw_call
        };

        let fn_def = format!(
            "pub fn {method_name}(&self{method_args}) -> Result<{return_ty}> {{
    {call_body}
}}"
        );

        function_defs.push(fn_def);
    }

    let impl_block = function_defs.join("\n\n");
    format!(
        "/// Implements Bitcoin Core JSON-RPC API method `{}` for version {}\n{}\n#[macro_export]\n\
         macro_rules! {} {{\n    () => {{\n        impl Client {{\n{}\n        }}\n    }};\n}}",
        method.name,
        version,
        description,
        macro_name,
        indent(&impl_block, 12)
    )
}

fn generate_method_args(method: &ApiMethod) -> String {
    let mut args = String::new();
    for arg in &method.arguments {
        let arg_name = &arg.names[0];
        let arg_type = {
            let (ty, _) = TypeRegistry.map_argument_type(arg);
            ty.to_string()
        };
        // Escape reserved keywords
        let escaped_name = if arg_name == "type" {
            format!("r#{arg_name}")
        } else {
            arg_name.clone()
        };
        if arg.optional {
            args.push_str(&format!(", {escaped_name}: Option<{arg_type}>"));
        } else {
            args.push_str(&format!(", {escaped_name}: {arg_type}"));
        }
    }
    args
}

fn generate_args(method: &ApiMethod) -> (String, String) {
    let mut required_args = Vec::new();
    let mut optional_args = Vec::new();
    for arg in &method.arguments {
        let arg_name = &arg.names[0];
        // Escape reserved keywords
        let escaped_name = if arg_name == "type" {
            format!("r#{arg_name}")
        } else {
            arg_name.clone()
        };
        let arg_expr = if method.name == "addnode" && arg_name == "command" {
            "serde_json::to_value(command)?".to_string()
        } else if arg.type_ == "object-named-parameters" {
            format!("into_json({escaped_name}.unwrap_or_default())?")
        } else {
            format!("into_json({escaped_name})?")
        };
        if arg.optional {
            optional_args.push(format!(
                "if let Some({escaped_name}) = {escaped_name} {{\n    params.push(into_json({escaped_name})?);\n}}"
            ));
        } else {
            required_args.push(arg_expr);
        }
    }
    (required_args.join(", "), optional_args.join("\n"))
}

fn get_return_type_from_results(results: &[rpc_api::ApiResult]) -> String {
    if results.is_empty() {
        "()".to_string()
    } else {
        get_return_type(&results[0])
    }
}

fn get_return_type(result: &rpc_api::ApiResult) -> String {
    if result.type_.as_str() == "object" && !result.inner.is_empty() {
        generate_object_type(result)
    } else {
        map_type_to_rust(result.type_.as_str())
    }
}

/// Converts a JSON RPC type identifier into its corresponding Rust type name.
///
/// This centralized mapping ensures that schema types like `"string"`, `"number"`,
/// and `"boolean"` are consistently translated to their Rust equivalents (`String`,
/// `f64`, `bool`, etc.). Any unrecognized or dynamic object types default to
/// `serde_json::Value`, preserving flexibility for unknown or evolving schemas.
///
/// # Arguments
///
/// * `type_str` – A string slice representing the JSON RPC type label.
///
/// # Returns
///
/// A `String` containing the Rust type name to use in generated code or runtime
/// conversions.
pub fn map_type_to_rust(type_str: &str) -> String {
    match type_str {
        "string" => "String".to_string(),
        "number" => "f64".to_string(),
        "boolean" => "bool".to_string(),
        "hex" => "String".to_string(),
        "object" => "serde_json::Value".to_string(),
        "object_dynamic" => "serde_json::Value".to_string(),
        _ => "serde_json::Value".to_string(),
    }
}

fn generate_object_type(result: &rpc_api::ApiResult) -> String {
    if result.inner.is_empty() || result.key_name == "object_dynamic" {
        "serde_json::Value".to_string()
    } else {
        let base_name = if result.key_name.is_empty() {
            "Value".to_string()
        } else {
            capitalize(&result.key_name)
        };
        format!("serde_json::{base_name}")
    }
}

/// Indents each line of a string by the specified number of spaces
fn indent(s: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    s.lines()
        .map(|line| format!("{pad}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}
