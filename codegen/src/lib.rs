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
/// 2. Returns a `Result<T, TransportError>` where:
///    - `T` is the generated response type for the RPC method
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
/// pub async fn getblockcount(transport: &Transport) -> Result<GetblockcountResponse, TransportError> {
///     let response = transport.send_request("getblockcount", &[] as &[Value]).await?;
///     Ok(serde_json::from_value(response)?)
/// }
/// ```
///
/// This allows users to make RPC calls like:
///
/// ```rust,ignore
/// let transport = Transport::new("http://127.0.0.1:18443");
/// let block_count = getblockcount(&transport).await?;
/// println!("Current block height: {}", block_count.count);
/// ```
pub struct TransportCodeGenerator;

impl CodeGenerator for TransportCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        methods
            .iter()
            .map(|m| {
                let name = &m.name;
                let sanitized_name = types::sanitize_method_name(name);

                // 1) Build the fn signature args
                let mut fn_args = vec!["transport: &Transport".to_string()];
                for arg in &m.arguments {
                    let arg_name = &arg.names[0];
                    let arg_type = types::map_type_to_rust(&arg.type_);
                    fn_args.push(format!("{}: {}", arg_name, arg_type));
                }
                let fn_args = fn_args.join(", ");

                // 2) Build the params vector literal
                let params_expr = if m.arguments.is_empty() {
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

                // 4) Generate response type and return type
                let response_type = types::generate_return_type(m);
                let return_type = response_type
                    .as_ref()
                    .map(|_| {
                        format!(
                            " -> Result<{}Response, TransportError>",
                            types::capitalize(name)
                        )
                    })
                    .unwrap_or_else(|| " -> Result<Value, TransportError>".to_string());

                // 5) Emit the module source with conditional imports
                let imports = if m.arguments.is_empty() {
                    "use serde_json::Value;"
                } else {
                    "use serde_json::{json, Value};"
                };

                let response_handler = if response_type.is_some() {
                    "Ok(serde_json::from_value(response)?)"
                } else {
                    "Ok(response)"
                };
                let response_type_str = response_type.unwrap_or_default();
                let src = format!(
                    r#"{docs}

use serde::{{Deserialize, Serialize}};
use transport::Transport;
{imports}
use transport::TransportError;

{response_type}

/// Calls the `{name}` RPC method.
pub async fn {sanitized_name}({fn_args}){return_type} {{
    let params = {params_expr};
    let response = transport.send_request("{name}", &params).await?;
    {response_handler}
}}
"#,
                    docs = docs,
                    name = name,
                    sanitized_name = sanitized_name,
                    fn_args = fn_args,
                    return_type = return_type,
                    params_expr = params_expr,
                    response_type = response_type_str,
                    imports = imports,
                    response_handler = response_handler
                );

                (name.clone(), src)
            })
            .collect()
    }
}
