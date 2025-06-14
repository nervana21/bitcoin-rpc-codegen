//! RPC Method Discovery module for bitcoin-rpc-codegen.
//!
//! Provides functionality to discover RPC methods and convert them into ApiMethod structures
//! for code generation.

use parser::BitcoinCli;
use rpc_api::{ApiArgument, ApiMethod, ApiResult};
use std::path::Path;

/// Discover available RPC methods and convert them into ApiMethod structures.
/// This is the main entry point for the discovery pipeline.
pub fn discover_methods(bitcoind_bin: &Path) -> Result<Vec<ApiMethod>, String> {
    let cli = BitcoinCli::new(bitcoind_bin);

    let methods = cli
        .get_methods()
        .map_err(|e| format!("Failed to get RPC methods: {}", e))?;

    // Convert each method into an ApiMethod structure
    let mut api_methods = Vec::new();
    for method_name in methods {
        let help_text = cli
            .get_help_text(&method_name)
            .map_err(|e| format!("Failed to get help for {}: {}", method_name, e))?;

        api_methods.push(parse_help_text(&method_name, &help_text));
    }

    Ok(api_methods)
}

/// Parse a help text into an ApiMethod structure
fn parse_help_text(method_name: &str, help_text: &str) -> ApiMethod {
    let mut sections = help_text.split("\n\n");

    // Parse description (everything before Arguments:)
    let description = sections
        .next()
        .unwrap_or("")
        .lines()
        .collect::<Vec<_>>()
        .join("\n");

    // Parse arguments
    let mut arguments = Vec::new();
    if let Some(args_section) = sections.find(|s| s.starts_with("Arguments:")) {
        for line in args_section.lines().skip(1) {
            if line.trim().is_empty() {
                continue;
            }
            if let Some((_num, rest)) = line.split_once('.') {
                if let Some((name_type, desc)) = rest.split_once("(") {
                    let name = name_type
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_string();
                    let type_ = name_type
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or("string")
                        .trim_matches(|c| c == '(' || c == ')')
                        .to_string();
                    let optional = desc.contains("optional");
                    arguments.push(ApiArgument {
                        names: vec![name],
                        type_,
                        optional,
                        description: desc.trim_matches(|c| c == '(' || c == ')').to_string(),
                    });
                }
            }
        }
    }

    // Parse results
    let mut results = Vec::new();
    if let Some(results_section) = sections.find(|s| s.starts_with("Result:")) {
        for line in results_section.lines().skip(1) {
            if line.trim().is_empty() {
                continue;
            }
            if let Some((key, rest)) = line.split_once("(") {
                let type_ = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or("string")
                    .trim_matches(|c| c == '(' || c == ')')
                    .to_string();
                let description = rest
                    .split_once(")")
                    .map(|(_, desc)| desc.trim().to_string())
                    .unwrap_or_default();
                results.push(ApiResult {
                    key_name: key.trim().to_string(),
                    type_,
                    description,
                    inner: Vec::new(),
                    optional: false,
                });
            }
        }
    }

    ApiMethod {
        name: method_name.to_string(),
        description,
        arguments,
        results,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_help_text() {
        let help_text = r#"Get information about the blockchain.

Arguments:
1. verbose    (boolean, optional) True for a json object, false for array of transaction ids
2. blockhash  (string, required) The block hash

Result:
"hash"    (string) The block hash
"confirmations"    (numeric) The number of confirmations
"size"    (numeric) The block size
"height"    (numeric) The block height"#;

        let method = parse_help_text("getblock", help_text);
        assert_eq!(method.name, "getblock");
        assert_eq!(method.arguments.len(), 2);
        assert_eq!(method.results.len(), 4);
        assert!(method.arguments[0].optional);
        assert!(!method.arguments[1].optional);
    }
}
