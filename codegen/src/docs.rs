// codegen/src/docs.rs

use rpc_api::ApiMethod;

/// Format documentation comments
pub fn format_doc_comment(description: &str) -> String {
    description
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| format!("/// {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Format a struct field with documentation
pub fn format_struct_field(field_name: &str, field_type: &str, description: &str) -> String {
    let desc = format_doc_comment(description);
    if desc.is_empty() {
        format!("    pub {}: {},\n", field_name, field_type)
    } else {
        format!("{}\n    pub {}: {},\n", desc, field_name, field_type)
    }
}

/// Generate example usage documentation for an RPC method
pub fn generate_example_docs(method: &ApiMethod, version: &str) -> String {
    let mut docs = String::new();
    docs.push_str("//! This file is auto-generated. Do not edit manually.\n");
    docs.push_str("//! Generated for Bitcoin Core version: ");
    docs.push_str(version);
    docs.push_str("\n\n");

    if !method.description.trim().is_empty() {
        for line in method.description.lines().filter(|l| !l.trim().is_empty()) {
            docs.push_str("/// ");
            docs.push_str(line.trim());
            docs.push('\n');
        }
    }

    // Add example usage
    docs.push_str("\n/// # Example\n");
    docs.push_str("/// ```rust\n");
    docs.push_str("/// use bitcoin_rpc_codegen::client::");
    docs.push_str(version);
    docs.push_str("::");
    docs.push_str(&method.name);
    docs.push_str(";\n///\n");
    docs.push_str("/// let client = Client::new(\"http://127.0.0.1:8332\", auth);\n");
    docs.push_str("/// let result = client.");
    docs.push_str(&method.name);
    docs.push('(');
    if !method.arguments.is_empty() {
        docs.push_str("/* params */");
    }
    docs.push_str(").await?;\n");
    docs.push_str("/// ```\n");

    docs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_doc_comment() {
        let input = "This is a test\nwith multiple lines\n\nand empty lines";
        let expected = "/// This is a test\n/// with multiple lines\n/// and empty lines";
        assert_eq!(format_doc_comment(input), expected);
    }

    #[test]
    fn test_format_struct_field() {
        let field = format_struct_field("block_count", "u32", "The number of blocks");
        assert!(field.contains("pub block_count: u32"));
        assert!(field.contains("/// The number of blocks"));
    }

    #[test]
    fn test_generate_example_docs() {
        let method = ApiMethod {
            name: "getblockcount".to_string(),
            description: "Returns the number of blocks".to_string(),
            arguments: vec![],
            results: vec![rpc_api::ApiResult {
                key_name: "count".to_string(),
                type_: "number".to_string(),
                description: "The current block count".to_string(),
            }],
        };

        let docs = generate_example_docs(&method, "v29");
        assert!(docs.contains("/// Returns the number of blocks"));
        assert!(docs.contains("/// # Example"));
        assert!(docs.contains("use bitcoin_rpc_codegen::client::v29::getblockcount"));
        assert!(docs.contains("let result = client.getblockcount().await?"));
    }
}
