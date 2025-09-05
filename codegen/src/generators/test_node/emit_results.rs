//! Generate result structs for RPC method returns

use rpc_api::{ApiMethod, ApiResult};
use std::fmt::Write as _;

use super::utils::camel;
use type_conversion::TypeRegistry;

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

/// Determines the appropriate Rust type for a given API result.
///
/// This function takes an `ApiResult` reference, uses the type registry to map the API result type
/// to a corresponding Rust type, and wraps the type in `Option<>` if the result is considered optional
/// according to the registry's mapping rules.
///
/// # Arguments
/// * `result` - A reference to the API result metadata.
///
/// # Returns
/// A `String` representing the Rust type for the result, possibly wrapped in `Option<>`.
fn rust_type_for_result(result: &ApiResult) -> String {
    let (base_ty, is_option) = TypeRegistry::new().map_result_type(result);
    if is_option {
        format!("Option<{base_ty}>")
    } else {
        base_ty.to_string()
    }
}
