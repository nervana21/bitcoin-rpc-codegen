//! Generate result structs for RPC method returns

use crate::utils::rust_type_for_result;
use rpc_api::ApiMethod;
use std::fmt::Write as _;

use super::utils::camel;

/// Generates Rust struct definitions for RPC method response types.
///
/// This function creates transparent wrapper structs for methods that return a single result.
/// Each generated struct is named `{MethodName}Response` and wraps the actual return type
/// with serde deserialization support.
pub fn generate_result_code(methods: &[ApiMethod]) -> String {
    let mut code =
        String::from("//! Result structs for RPC method returns\nuse serde::Deserialize;\n\n");
    for m in methods {
        if m.results.len() != 1 {
            continue;
        }
        let r = &m.results[0];
        let ty = rust_type_for_result(r);
        writeln!(
            code,
            "#[derive(Debug, Deserialize)]\n#[serde(transparent)]\npub struct {}Response(pub {});\n",
            camel(&m.name),
            ty
        )
        .unwrap();
    }
    code
}
