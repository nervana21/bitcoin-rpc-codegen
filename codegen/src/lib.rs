// codegen/src/lib.rs

use rpc_api::ApiMethod;
use std::{fs, io::Write, path::Path};

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

impl CodeGenerator for JsonRpcCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        methods
            .iter()
            .map(|m| {
                let func = &m.name;
                let rpc = &m.name;
                let src = format!(
                    r#"// Auto‑generated JSON-RPC client for `{rpc}`

use serde_json::json;
use serde_json::Value;
use reqwest::Client;

/// Calls the `{rpc}` method on the configured node.
pub async fn {func}(client: &Client) -> Result<Value, reqwest::Error> {{
    let req = json!({{
        "jsonrpc": "2.0",
        "method": "{rpc}",
        "params": [],
        "id": 1,
    }});
    let resp = client
        .post("{url}")
        .json(&req)
        .send()
        .await?
        .json::<Value>()
        .await?;
    Ok(resp["result"].clone())
}}
"#,
                    rpc = rpc,
                    func = func,
                    url = self.url,
                );
                (func.clone(), src)
            })
            .collect()
    }
}
