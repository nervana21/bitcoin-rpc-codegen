//! Generate parameter structs for RPC method calls

use crate::generators::doc_comment;
use crate::utils::{camel_to_snake_case, rust_type_for_argument};
use types::ApiMethod;
use std::fmt::Write as _;

use super::utils::camel;

/// Generates Rust parameter structs for Bitcoin RPC methods that require arguments.
///
/// This function takes a collection of API methods and produces Rust code that defines
/// serializable parameter structs for each method that has arguments. Each struct is
/// named `{MethodName}Params` and contains fields corresponding to the method's parameters.
///
/// The generated code includes:
/// - Proper Rust naming conventions (snake_case for fields, camelCase for struct names)
/// - Serde serialization support for JSON-RPC communication
/// - Documentation comments from the original API specification
/// - Type mapping from Bitcoin RPC types to appropriate Rust types
/// - Special handling for reserved keywords (e.g., "type" becomes "_type")
///
/// Methods without arguments are skipped, as they don't require parameter structs.
///
/// # Arguments
///
/// * `methods` - A slice of API methods from the Bitcoin RPC specification
///
/// # Returns
///
/// A `String` containing the complete Rust code for all parameter structs
pub fn generate_params_code(methods: &[ApiMethod]) -> String {
    let mut code =
        String::from("//! Parameter structs for RPC method calls\nuse serde::Serialize;\n\n");
    for m in methods {
        if m.arguments.is_empty() {
            continue;
        }
        writeln!(code, "{}", doc_comment::format_doc_comment(&m.description)).unwrap();
        writeln!(
            code,
            "#[derive(Debug, Serialize)]\npub struct {}Params {{",
            camel(&m.name)
        )
        .unwrap();
        for p in &m.arguments {
            let field = if p.names[0] == "type" {
                "_type"
            } else {
                &camel_to_snake_case(&p.names[0])
            };
            let ty = rust_type_for_argument(&p.names[0], &p.type_);
            writeln!(code, "    pub {field}: {ty},").unwrap();
        }
        writeln!(code, "}}\n").unwrap();
    }
    code
}
