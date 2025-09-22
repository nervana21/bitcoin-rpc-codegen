// codegen/src/doc_comment_generator.rs

use bitcoin_rpc_types::BtcMethod;

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
pub fn generate_example_docs(method: &BtcMethod, version: &str) -> String {
    let mut docs = String::new();
    docs.push_str("//! This file is auto-generated. Do not edit manually.\n");
    docs.push_str(&format!("//! Generated from Bitcoin Core {}\n\n", version));

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
    docs.push_str(&version.replace(".", "_"));
    docs.push_str("::");
    docs.push_str(&method.name);
    docs.push_str(";\n///\n");
    docs.push_str("/// let client = Client::new(\"http://127.0.0.1:18443\", auth);\n");
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