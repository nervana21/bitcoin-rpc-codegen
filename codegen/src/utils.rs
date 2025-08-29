// codegen/src/utils.rs

use rpc_api::ApiArgument;
use type_registry::TypeRegistry;

/// Converts a camelCase string to snake_case
pub fn camel_to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

/// Capitalizes the first character of a string and converts snake_case/kebab-case to PascalCase
pub fn capitalize(s: &str) -> String {
    s.split(['_', '-'])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
}

/// Determines the appropriate Rust type for a given API argument.
///
/// This function takes the parameter name and its API type as input,
/// consults the type registry to map the API type to a Rust type,
/// and wraps the type in `Option<>` if the argument is considered optional
/// according to the registry's mapping rules.
///
/// # Arguments
/// * `param_name` - The name of the parameter.
/// * `api_ty` - The type of the parameter as specified in the API.
///
/// # Returns
/// A `String` representing the Rust type for the argument, possibly wrapped in `Option<>`.
pub fn rust_type_for_argument(param_name: &str, api_ty: &str) -> String {
    let (base_ty, is_option) = TypeRegistry::new().map_argument_type(&ApiArgument {
        type_: api_ty.to_string(),
        names: vec![param_name.to_string()],
        type_str: None,
        required: true,
        description: String::new(),
    });
    if is_option {
        format!("Option<{base_ty}>")
    } else {
        base_ty.to_string()
    }
}
