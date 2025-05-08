// codegen/src/lib.rs

use rpc_api::ApiMethod;

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
