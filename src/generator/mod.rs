//src/generator/mod.rs

use crate::parser::{ApiMethod, ApiResult};

pub mod codegen;
pub use codegen::SUPPORTED_VERSIONS;

use std::fmt::Write;
use std::fs;
use std::io;
use std::path::Path;

/// Generates a struct definition string from a type name, a description, and its fields.
pub fn generate_struct(type_name: &str, description: &str, fields: &str) -> String {
    let mut s = String::new();
    writeln!(s, "/// Response for the {} RPC call.", type_name).unwrap();
    writeln!(s, "{}", description).unwrap();
    writeln!(
        s,
        "#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]"
    )
    .unwrap();
    writeln!(s, "pub struct {} {{", type_name).unwrap();
    writeln!(s, "{}", fields).unwrap();
    write!(s, "}}").unwrap();
    s
}

/// Implements the client macro for a given RPC method and version.
pub fn generate_client_macro(method: &ApiMethod, version: &str) -> String {
    codegen::generate_client_macro(method, version)
}

/// Generates the return type struct for a given RPC method.
pub fn generate_return_type(method: &ApiMethod) -> Option<String> {
    codegen::generate_return_type(method)
}

/// Retrieves the Rust type for a nested ApiResult, with strict fallback.
fn get_return_type(result: &ApiResult) -> String {
    if result.type_.eq_ignore_ascii_case("object") && !result.inner.is_empty() {
        "serde_json::Value".to_string()
    } else {
        codegen::map_type_to_rust(&result.type_)
    }
}

/// Fallback to a default return type if no results.
fn get_return_type_from_results(results: &[ApiResult]) -> String {
    results.first().map_or("()".into(), get_return_type)
}

/// Determine each field's Rust type, now with strict fallback for arrays/objects.
fn get_field_type(field: &ApiResult) -> String {
    match field.type_.as_str() {
        "array" | "array-fixed" => "Vec<serde_json::Value>".to_string(),
        "object" if !field.inner.is_empty() => "serde_json::Value".to_string(),
        _ => codegen::map_type_to_rust(&field.type_),
    }
}

/// Generate struct fields for nested JSON results (delegated/fallback).
fn generate_struct_fields(_result: &ApiResult) -> String {
    codegen::generate_struct_fields(_result)
}

/// Generate inline object types, now always fallback.
fn generate_object_type(_result: &ApiResult) -> String {
    "serde_json::Value".to_string()
}

fn sanitize_method_name(name: &str) -> String {
    codegen::sanitize_method_name(name)
}

/// Convert an arbitrary JSON key into a valid Rust field name by
/// keeping only ASCII alphanumeric characters and underscores.
fn sanitize_field_name(name: &str) -> String {
    codegen::sanitize_field_name(name)
}

fn capitalize(s: &str) -> String {
    codegen::capitalize(s)
}

/// Generates the top-level `mod.rs` files (unchanged).
pub fn generate_mod_rs(output_dir: &str, versions: &[&str]) -> std::io::Result<()> {
    codegen::generate_mod_rs(output_dir, versions)
}

pub fn generate_type_conversion(method: &ApiMethod, _version: &str) -> Option<String> {
    if method
        .results
        .iter()
        .any(|r| r.type_.eq_ignore_ascii_case("none"))
    {
        return None;
    }
    let type_name = format!("{}Response", capitalize(&method.name));
    let model_type = format!("model::{}", type_name);
    Some(format!(
        r#"impl {} {{
    pub fn into_model(self) -> Result<{}, {}Error> {{
        Ok(())
    }}
}}"#,
        type_name, model_type, type_name
    ))
}

fn generate_method_args(method: &ApiMethod) -> String {
    let mut args = String::new();
    for arg in &method.arguments {
        let arg_name = &arg.names[0];
        let arg_type = match arg.type_.as_str() {
            "hex" => "String".to_string(),
            "string" => "String".to_string(),
            "number" => "i64".to_string(),
            "boolean" => "bool".to_string(),
            "array" => {
                if arg_name == "inputs" {
                    "Vec<Input>".to_string()
                } else if arg_name == "outputs" {
                    "Vec<Output>".to_string()
                } else {
                    "Vec<String>".to_string()
                }
            }
            "object" | "object-named-parameters" => "serde_json::Value".to_string(),
            _ => arg.type_.clone(),
        };
        if arg.optional {
            args.push_str(&format!(", {}: Option<{}>", arg_name, arg_type));
        } else {
            args.push_str(&format!(", {}: {}", arg_name, arg_type));
        }
    }
    args
}

fn generate_args(method: &ApiMethod) -> (String, String) {
    let mut required_args = Vec::new();
    let mut optional_args = Vec::new();
    for arg in &method.arguments {
        let arg_name = &arg.names[0];
        let arg_expr = if method.name == "addnode" && arg_name == "command" {
            "serde_json::to_value(command)?".to_string()
        } else {
            format!("into_json({})?", arg_name)
        };
        if arg.optional {
            optional_args.push(format!(
                "if let Some({}) = {} {{\n    params.push(into_json({})?);\n}}",
                arg_name, arg_name, arg_name
            ));
        } else {
            required_args.push(arg_expr);
        }
    }
    (required_args.join(", "), optional_args.join("\n"))
}

fn format_doc_comment(description: &str) -> String {
    codegen::format_doc_comment(description)
}

fn format_struct_field(field_name: &str, field_type: &str, description: &str) -> String {
    codegen::format_struct_field(field_name, field_type, description)
}

fn indent(s: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    s.lines()
        .map(|line| format!("{}{}", pad, line))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn generate_client_mod_rs(versions: &[String], out_dir: &Path) -> io::Result<()> {
    generate_versioned_mod_rs(versions, out_dir, "client", "pub mod {};")
}

pub fn generate_types_mod_rs(versions: &[String], out_dir: &Path) -> io::Result<()> {
    generate_versioned_mod_rs(versions, out_dir, "types", "pub mod {}_types;")
}

/// Helper function to generate a mod.rs file declaring versioned modules.
fn generate_versioned_mod_rs(
    versions: &[String],
    out_dir: &Path,
    subdir: &str,
    mod_template: &str,
) -> io::Result<()> {
    use std::fmt::Write; // Import the Write trait for writeln!

    // Content for the mod.rs
    let mod_rs_content = versions.iter().fold(String::new(), |mut output, version| {
        // Use the provided template to format the module declaration
        let _ = writeln!(output, "{}", mod_template.replace("{}", version));
        output
    });

    let mod_path = out_dir.join(subdir).join("mod.rs");
    fs::create_dir_all(mod_path.parent().unwrap())?;
    fs::write(&mod_path, mod_rs_content)?;
    Ok(())
}

/// Generates the top-level `src/client/mod.rs` file that declares all versioned client modules.
pub fn generate_top_level_client_mod(versions: &[String], src_dir: &Path) -> io::Result<()> {
    use std::fmt::Write; // Import the Write trait for writeln!

    let mut client_mod_rs = String::new();
    writeln!(
        client_mod_rs,
        "// Auto-generated client module declarations."
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    writeln!(client_mod_rs, "// Do not edit this file manually.")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    writeln!(client_mod_rs).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?; // Add a blank line

    for version in versions {
        writeln!(client_mod_rs, "pub mod {}", version)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
    writeln!(client_mod_rs).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?; // Add a blank line

    for version in versions {
        writeln!(client_mod_rs, "pub use self::{}::*;", version)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    let client_mod_path = src_dir.join("client").join("mod.rs");
    fs::create_dir_all(client_mod_path.parent().unwrap())?;
    fs::write(&client_mod_path, client_mod_rs)?;
    Ok(())
}

/// Generates the top-level `src/types/mod.rs` file that declares all versioned types modules.
pub fn generate_top_level_types_mod(versions: &[String], src_dir: &Path) -> io::Result<()> {
    use std::fmt::Write; // Import the Write trait for writeln!

    let mut types_mod_rs = String::new();
    writeln!(types_mod_rs, "// Auto-generated types module declarations.")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    writeln!(types_mod_rs, "// Do not edit this file manually.")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    writeln!(types_mod_rs).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?; // Add a blank line

    for version in versions {
        writeln!(types_mod_rs, "pub mod {}_types;", version)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
    writeln!(types_mod_rs).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?; // Add a blank line

    for version in versions {
        writeln!(types_mod_rs, "pub use self::{}_types::*;", version)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    let types_mod_path = src_dir.join("types").join("mod.rs");
    fs::create_dir_all(types_mod_path.parent().unwrap())?;
    fs::write(&types_mod_path, types_mod_rs)?;
    Ok(())
}

pub use crate::generator::codegen::generate_version_code;
