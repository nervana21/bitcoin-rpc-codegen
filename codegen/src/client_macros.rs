//! # RPC‑to‑Rust Macro Generator
//!
//! `generate_client_macro` turns **one** `ApiMethod` + a version tag  
//! (e.g. `"v29"`) ➜ a `macro_rules! impl_client_v29__<method>()` string.
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

use crate::types::{capitalize, sanitize_method_name};
use rpc_api::ApiMethod;

/// Generates a client-side macro implementation for an RPC method
pub fn generate_client_macro(method: &ApiMethod, version: &str) -> String {
    let method_name = sanitize_method_name(&method.name);
    let macro_name = format!("impl_client_{}__{}", version, method_name);
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
        let default_doc = format!("{} with default parameters.", description);
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
        let param_doc = format!("{} with specified parameters.", description);
        let method_args = generate_method_args(method);
        let (required_args, optional_body_raw) = generate_args(method);
        let optional_body = optional_body_raw
            .lines()
            .map(|line| format!("    {}", line))
            .collect::<Vec<_>>()
            .join("\n");
        let param_call = make_call(&required_args, &optional_body);
        let param_fn = format!(
            "/// {doc}\npub fn {name}(&self{args}) -> Result<{ret}> {{
    {call}
}}",
            doc = param_doc,
            name = method_name,
            args = method_args,
            ret = return_ty,
            call = param_call
        );

        function_defs.push(default_fn);
        function_defs.push(param_fn);
    } else {
        // Single‐method variant
        let method_args = generate_method_args(method);
        let (required_args, optional_body_raw) = generate_args(method);
        let optional_body = optional_body_raw
            .lines()
            .map(|line| format!("    {}", line))
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
            "pub fn {name}(&self{args}) -> Result<{ret}> {{
    {body}
}}",
            name = method_name,
            args = method_args,
            ret = return_ty,
            body = call_body
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

fn format_doc_comment(description: &str) -> String {
    description
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| format!("/// {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_method_args(method: &ApiMethod) -> String {
    let mut args = String::new();
    for arg in &method.arguments {
        let arg_name = &arg.names[0];
        let arg_type = match arg.type_.as_str() {
            "hex" => "String".to_string(),
            "string" => "String".to_string(),
            "number" => "i64".to_string(),
            "boolean" => "bool".to_string(),
            "array" => {
                if arg_name == "inputs" {
                    "Vec<Input>".to_string()
                } else if arg_name == "outputs" {
                    "Vec<Output>".to_string()
                } else {
                    "Vec<String>".to_string()
                }
            }
            "object" | "object-named-parameters" => "serde_json::Value".to_string(),
            _ => arg.type_.clone(),
        };
        if arg.optional {
            args.push_str(&format!(", {}: Option<{}>", arg_name, arg_type));
        } else {
            args.push_str(&format!(", {}: {}", arg_name, arg_type));
        }
    }
    args
}

fn generate_args(method: &ApiMethod) -> (String, String) {
    let mut required_args = Vec::new();
    let mut optional_args = Vec::new();
    for arg in &method.arguments {
        let arg_name = &arg.names[0];
        let arg_expr = if method.name == "addnode" && arg_name == "command" {
            "serde_json::to_value(command)?".to_string()
        } else {
            format!("into_json({})?", arg_name)
        };
        if arg.optional {
            optional_args.push(format!(
                "if let Some({}) = {} {{\n    params.push(into_json({})?);\n}}",
                arg_name, arg_name, arg_name
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

fn map_type_to_rust(type_str: &str) -> String {
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
        format!("serde_json::{}", base_name)
    }
}

fn indent(s: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    s.lines()
        .map(|line| format!("{}{}", pad, line))
        .collect::<Vec<_>>()
        .join("\n")
}
