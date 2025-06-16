// codegen/src/doc_comment_generator.rs

use rpc_api::ApiMethod;

/// Format documentation comments
pub fn format_doc_comment(description: &str) -> String {
    let mut doc = String::new();
    let mut current_section = String::new();
    let mut in_section = false;
    let mut first_section = true;
    let mut in_code_block = false;

    for line in description.lines() {
        let line = line.trim();

        // Handle code block markers
        if line.starts_with("```") {
            if !in_code_block {
                // Start of code block
                if !current_section.is_empty() {
                    // Process any pending section content
                    process_section(&mut doc, &current_section, in_section, &mut first_section);
                    current_section.clear();
                }
                doc.push_str("///\n");
            }
            doc.push_str(&format!("/// {line}\n"));
            in_code_block = !in_code_block;
            continue;
        }

        // Process the line
        let processed_line = if !in_code_block {
            // Replace backticks with double backticks
            let line = line.replace('`', "``");
            // Replace single quotes with double quotes when they look like string literals
            let line = line.replace("'", "\"");
            // Add space before possessive apostrophes
            line.replace("'s ", " 's ")
        } else {
            line.to_string()
        };

        if processed_line.is_empty() {
            if !current_section.is_empty() {
                process_section(&mut doc, &current_section, in_section, &mut first_section);
                current_section.clear();
            }
            in_section = false;
        } else {
            if processed_line.starts_with("Arguments:")
                || processed_line.starts_with("Result:")
                || processed_line.starts_with("Examples:")
            {
                in_section = true;
                current_section.clear();
            }
            current_section.push_str(&processed_line);
            current_section.push('\n');
        }
    }

    // Process the last section
    if !current_section.is_empty() {
        process_section(&mut doc, &current_section, in_section, &mut first_section);
    }

    doc.trim_end().to_string()
}

fn process_section(doc: &mut String, section: &str, in_section: bool, first_section: &mut bool) {
    if !*first_section {
        doc.push_str("///\n");
    }
    *first_section = false;

    if section.starts_with("Arguments:") {
        doc.push_str("/// # Arguments\n");
        for section_line in section.lines().skip(1) {
            let section_line = section_line.trim();
            if !section_line.is_empty() {
                doc.push_str(&format!("/// {section_line}\n"));
            }
        }
    } else if section.starts_with("Result:") {
        doc.push_str("/// # Returns\n");
        for section_line in section.lines().skip(1) {
            let section_line = section_line.trim();
            if !section_line.is_empty() {
                doc.push_str(&format!("/// {section_line}\n"));
            }
        }
    } else if section.starts_with("Examples:") {
        doc.push_str("/// # Examples\n");
        for section_line in section.lines().skip(1) {
            let section_line = section_line.trim();
            if !section_line.is_empty() {
                doc.push_str(&format!("/// {section_line}\n"));
            }
        }
    } else if !in_section {
        // This is the description section
        for desc_line in section.lines() {
            let desc_line = desc_line.trim();
            if !desc_line.is_empty() {
                doc.push_str(&format!("/// {desc_line}\n"));
            }
        }
    }
}

/// Format a struct field with documentation
pub fn format_struct_field(field_name: &str, field_type: &str, description: &str) -> String {
    let desc = format_doc_comment(description);
    if desc.is_empty() {
        format!("    pub {field_name}: {field_type},\n")
    } else {
        format!("{desc}\n    pub {field_name}: {field_type},\n")
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
        let expected = "/// This is a test\n/// with multiple lines\n///\n/// and empty lines";
        assert_eq!(format_doc_comment(input), expected);
    }

    #[test]
    fn test_format_doc_comment_with_sections() {
        let input = r#"This is a description
with multiple lines

Arguments:
1. arg1    (string, required) First argument
2. arg2    (number, optional) Second argument

Result:
"result"    (string) The result value

Examples:
> bitcoin-cli command arg1 arg2
> curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "command", "params": ["arg1", "arg2"]}' -H 'content-type: application/json' http://127.0.0.1:8332/"#;

        let expected = r#"/// This is a description
/// with multiple lines
///
/// # Arguments
/// 1. arg1    (string, required) First argument
/// 2. arg2    (number, optional) Second argument
///
/// # Returns
/// "result"    (string) The result value
///
/// # Examples
/// > bitcoin-cli command arg1 arg2
/// > curl --user myusername --data-binary "{"jsonrpc": "2.0", "id": "curltest", "method": "command", "params": ["arg1", "arg2"]}" -H "content-type: application/json" http://127.0.0.1:8332/"#;

        assert_eq!(format_doc_comment(input), expected);
    }

    #[test]
    fn test_format_doc_comment_with_missing_sections() {
        let input = r#"This is a description
with multiple lines

Result:
"result"    (string) The result value"#;

        let expected = r#"/// This is a description
/// with multiple lines
///
/// # Returns
/// "result"    (string) The result value"#;

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
                inner: vec![],
                optional: false,
            }],
        };

        let docs = generate_example_docs(&method, "v29");
        assert!(docs.contains("/// Returns the number of blocks"));
        assert!(docs.contains("/// # Example"));
        assert!(docs.contains("use bitcoin_rpc_codegen::client::v29::getblockcount"));
        assert!(docs.contains("let result = client.getblockcount().await?"));
    }
}
