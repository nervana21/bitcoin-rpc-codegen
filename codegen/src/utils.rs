// codegen/src/utils.rs

use bitcoin_rpc_types::BtcArgument;
use type_conversion::TypeRegistry;

/// Converts a camelCase string to snake_case
pub fn camel_to_snake_case(s: &str) -> String {
    // This requires special handling because otherwise, it would be converted to "script_pub_key".
    if s == "scriptPubKey" {
        return "script_pubkey".to_string();
    }

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
    let (base_ty, is_option) = TypeRegistry::new().map_argument_type(&BtcArgument {
        type_: api_ty.to_string(),
        names: vec![param_name.to_string()],
        type_str: None,
        required: true,
        description: String::new(),
        oneline_description: String::new(),
        also_positional: false,
        hidden: false,
    });
    if is_option {
        format!("Option<{base_ty}>")
    } else {
        base_ty.to_string()
    }
}

/// Check if a method requires argument reordering for Rust function signatures.
///
/// Some Bitcoin RPC methods have unusual argument ordering where optional parameters appear
/// before required ones (e.g., `prioritisetransaction`, `sendmany`, `walletcreatefundedpsbt`).
/// This function identifies such cases for automatic reordering.
///
/// # Returns
/// `true` if the method requires argument reordering, `false` otherwise.
pub fn needs_parameter_reordering(args: &[BtcArgument]) -> bool {
    if args.len() < 2 {
        return false;
    }

    // Check if any required argument comes after an optional one
    args.windows(2).any(|window| !window[0].required && window[1].required)
}

/// Reorder arguments for Rust function signatures
///
/// Some Bitcoin RPC methods have unusual argument ordering where optional parameters appear
/// before required ones (e.g., `prioritisetransaction`, `sendmany`, `walletcreatefundedpsbt`).
/// This function reorders arguments to create idiomatic Rust function signatures while maintaining
/// compatibility with the Bitcoin Core JSON-RPC argument order.
///
/// Used by `MethodTemplate::generate_param_struct()` to create parameter structs for methods
/// that require argument reordering.
pub fn reorder_arguments_for_rust_signature(
    args: &[BtcArgument],
) -> (Vec<BtcArgument>, Vec<usize>) {
    if args.len() < 2 {
        return (args.to_vec(), (0..args.len()).collect());
    }

    // Check if reordering is needed
    let needs_reordering = args.windows(2).any(|window| !window[0].required && window[1].required);

    if !needs_reordering {
        return (args.to_vec(), (0..args.len()).collect());
    }

    // Split into required and optional arguments, preserving order within each group
    let mut required_args = Vec::new();
    let mut optional_args = Vec::new();
    let mut required_indices = Vec::new();
    let mut optional_indices = Vec::new();

    for (i, arg) in args.iter().enumerate() {
        if arg.required {
            required_args.push(arg.clone());
            required_indices.push(i);
        } else {
            optional_args.push(arg.clone());
            optional_indices.push(i);
        }
    }

    // Combine: required first, then optional
    let mut reordered_args = required_args;
    reordered_args.extend(optional_args);

    // Create mapping from reordered position to original position
    // The mapping tells us: for each position in the reordered args, what was its original position
    let mut mapping = Vec::new();
    mapping.extend(required_indices);
    mapping.extend(optional_indices);

    (reordered_args, mapping)
}
