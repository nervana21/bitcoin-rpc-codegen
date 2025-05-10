// codegen/src/lib.rs

use rpc_api::ApiMethod;
use std::{fs, io::Write, path::Path};

pub mod docs;
pub mod types;

/// A code generator that turns a list of `ApiMethod` into Rust source files.
///
/// Returns a `Vec` of `(module_name, source_code)` tuples.
pub trait CodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)>;
}

/// A minimal generator that creates one file per RPC method,
/// each containing an empty `pub fn <method_name>() { unimplemented!() }` stub.
pub struct BasicCodeGenerator;

impl CodeGenerator for BasicCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        methods
            .iter()
            .map(|m| {
                let name = m.name.clone();
                let src = format!(
                    r#"// Auto‑generated stub for RPC method `{name}`

pub fn {name}() {{
    unimplemented!();
}}
"#,
                    name = name,
                );
                (name, src)
            })
            .collect()
    }
}

/// Write generated modules to disk under `out_dir`.
///
/// Creates `out_dir` if needed, and writes each `(module_name, src)`
/// pair to a `<module_name>.rs` file.
pub fn write_generated<P: AsRef<Path>>(
    out_dir: P,
    files: &[(String, String)],
) -> std::io::Result<()> {
    let out_dir = out_dir.as_ref();
    fs::create_dir_all(out_dir)?;
    for (name, src) in files {
        let path = out_dir.join(format!("{}.rs", name));
        let mut file = fs::File::create(&path)?;
        file.write_all(src.as_bytes())?;
    }
    Ok(())
}

/// A generator that emits fully‑templated JSON‑RPC async functions
/// using `reqwest` and `serde_json`.
pub struct JsonRpcCodeGenerator {
    /// The URL of the Bitcoin node RPC endpoint, e.g. "http://127.0.0.1:18443"
    pub url: String,
}

/// A code generator that creates type-safe Rust functions for Bitcoin Core RPC methods.
///
/// This generator takes a list of RPC methods and generates corresponding async Rust functions
/// that communicate with a Bitcoin Core node using JSON-RPC over HTTP. Each generated function:
///
/// 1. Takes a `Transport` instance that handles the low-level HTTP communication
/// 2. Returns a `Result<Value, TransportError>` where:
///    - `Value` is the JSON-RPC response data
///    - `TransportError` captures both HTTP and RPC-level errors
///
/// The generated code provides a thin wrapper around the `Transport` layer, making it easy to
/// call Bitcoin Core RPC methods while handling authentication, HTTP communication, and error
/// handling in a consistent way.
///
/// # Example
///
/// For an RPC method like `getblockcount`, this generator will create:
///
/// ```rust,ignore
/// pub async fn getblockcount(transport: &Transport) -> Result<Value, TransportError> {
///     transport.send_request("getblockcount", &[] as &[Value]).await
/// }
/// ```
///
/// This allows users to make RPC calls like:
///
/// ```rust,ignore
/// let transport = Transport::new("http://127.0.0.1:18443");
/// let block_count = getblockcount(&transport).await?;
/// ```
pub struct TransportCodeGenerator;

impl CodeGenerator for TransportCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        methods
            .iter()
            .map(|m| {
                let name = &m.name;
                let sanitized_name = types::sanitize_method_name(name);

                // 1) Build the fn signature args: always `transport: &Transport`,
                //    plus one `arg_name: serde_json::Value` per ApiArgument.
                let mut fn_args = vec!["transport: &Transport".to_string()];
                for arg in &m.arguments {
                    let arg_name = &arg.names[0];
                    let arg_type = types::map_type_to_rust(&arg.type_);
                    fn_args.push(format!("{}: {}", arg_name, arg_type));
                }
                let fn_args = fn_args.join(", ");

                // 2) Build the params vector literal: either `vec![]` or
                //    `vec![ json!(arg1), json!(arg2), … ]`.
                let params_expr = if m.arguments.is_empty() {
                    // explicit empty Vec<Value> so `P = Value` can be inferred
                    "Vec::<Value>::new()".to_string()
                } else {
                    let elems: Vec<_> = m
                        .arguments
                        .iter()
                        .map(|arg| format!("json!({})", &arg.names[0]))
                        .collect();
                    format!("vec![{}]", elems.join(", "))
                };

                // 3) Generate documentation
                let docs = docs::generate_example_docs(m, "latest");

                // 4) Generate return type
                let return_type = types::generate_return_type(m)
                    .map(|t| format!(" -> Result<{}, TransportError>", t))
                    .unwrap_or_else(|| " -> Result<Value, TransportError>".to_string());

                // 5) Emit the module source with conditional imports
                let imports = if m.arguments.is_empty() {
                    "use serde_json::Value;"
                } else {
                    "use serde_json::{json, Value};"
                };

                let src = format!(
                    r#"{docs}

use transport::Transport;
{imports}
use transport::TransportError;

/// Calls the `{name}` RPC method.
pub async fn {sanitized_name}({fn_args}){return_type} {{
    let params = {params_expr};
    transport.send_request("{name}", &params).await
}}
"#,
                    docs = docs,
                    name = name,
                    sanitized_name = sanitized_name,
                    fn_args = fn_args,
                    params_expr = params_expr,
                    imports = imports,
                    return_type = return_type,
                );

                (name.clone(), src)
            })
            .collect()
    }
}
