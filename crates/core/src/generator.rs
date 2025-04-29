// crates/core/src/generator.rs

use crate::schema::{ApiMethod, ApiResult};
use anyhow::Result;
use std::fmt::Write as FmtWrite;
use std::{fs, io::Write as _, path::Path};

pub const SUPPORTED_MAJORS: &[u32] = &[17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];

/// Map JSON schema types to Rust types
pub fn map_type_to_rust(type_str: &str) -> String {
    match type_str {
        "string" => "String".to_string(),
        "number" => "f64".to_string(),
        "boolean" => "bool".to_string(),
        "hex" => "String".to_string(),
        "object" => "serde_json::Value".to_string(),
        "object_dynamic" => "serde_json::Value".to_string(),
        _ => "String".to_string(),
    }
}

/// Sanitize an RPC method name into a valid Rust identifier
pub fn sanitize_method_name(name: &str) -> String {
    name.replace("-", "_").to_lowercase()
}

/// Capitalize a string, handling hyphenated words
pub fn capitalize(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

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

/// Sanitize a field name into a valid Rust identifier
pub fn sanitize_field_name(name: &str) -> String {
    let filtered: String = name
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c == '_' {
                Some(c)
            } else {
                None
            }
        })
        .collect();
    if filtered
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        format!("_{}", filtered)
    } else {
        filtered
    }
}

/// Get the Rust type for a field
pub fn get_field_type(field: &ApiResult) -> String {
    match field.type_.as_str() {
        "array" | "array-fixed" => "Vec<serde_json::Value>".to_string(),
        "object" if !field.inner.is_empty() => "serde_json::Value".to_string(),
        _ => map_type_to_rust(&field.type_),
    }
}

/// Generate the client-side macro for a single RPC method
pub fn generate_client_macro(method: &ApiMethod, version: &str) -> String {
    let name = &method.name;
    let func_name = sanitize_method_name(name);
    let macro_name = format!("impl_client_{}__{}", version, func_name);

    // 1) Typed params list: `foo: String, bar: bool`
    let params_decl = method
        .arguments
        .iter()
        .map(|arg| format!("{}: {}", arg.names[0], map_type_to_rust(&arg.type_)))
        .collect::<Vec<_>>()
        .join(", ");

    // 2) JSON array: `json!([foo, bar])`
    let params_json = if method.arguments.is_empty() {
        "json!([])".to_string()
    } else {
        let args = method
            .arguments
            .iter()
            .map(|arg| arg.names[0].clone())
            .collect::<Vec<_>>()
            .join(", ");
        format!("json!([{}])", args)
    };

    // 3) Documentation with examples
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

    // Add example usage if available
    if let Some(_example) = method.examples.as_ref() {
        docs.push_str("\n/// # Example\n");
        docs.push_str("/// ```rust\n");
        docs.push_str("/// use bitcoin_rpc_codegen::client::");
        docs.push_str(version);
        docs.push_str("::");
        docs.push_str(&func_name);
        docs.push_str(";\n///\n");
        docs.push_str("/// let client = Client::new(\"http://127.0.0.1:8332\", auth);\n");
        docs.push_str("/// let result = client.");
        docs.push_str(&func_name);
        docs.push('(');
        if !method.arguments.is_empty() {
            docs.push_str("/* params */");
        }
        docs.push_str(").await?;\n");
        docs.push_str("/// ```\n");
    }

    // 4) Assemble the macro
    let mut out = String::new();
    out.push_str(&docs);
    writeln!(out, "/// client impl for `{}` RPC ({})", name, version).unwrap();
    writeln!(out, "macro_rules! {} {{", macro_name).unwrap();
    writeln!(out, "    () => {{").unwrap();
    if params_decl.is_empty() {
        writeln!(
            out,
            "        pub fn {}(&self) -> RpcResult<{}Response> {{",
            func_name,
            capitalize(name)
        )
        .unwrap();
    } else {
        writeln!(
            out,
            "        pub fn {}(&self, {}) -> RpcResult<{}Response> {{",
            func_name,
            params_decl,
            capitalize(name)
        )
        .unwrap();
    }
    writeln!(out, "            self.call(\"{}\", {})", name, params_json).unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }};").unwrap();
    writeln!(out, "}}").unwrap();

    out
}

/// Generate the return type for a method
pub fn generate_return_type(method: &ApiMethod) -> Option<String> {
    if method.results.is_empty() {
        return None;
    }
    let result = &method.results[0];
    if result.type_.eq_ignore_ascii_case("none") {
        return None;
    }
    let type_name = format!("{}Response", capitalize(&method.name));
    let formatted_description = format_doc_comment(&method.description);
    let fields = if result.inner.is_empty() {
        format!("    pub result: {},", get_return_type(result))
    } else {
        generate_struct_fields(result)
    };

    let mut s = String::new();
    writeln!(
        s,
        "//! This file is auto-generated. Do not edit manually.\n"
    )
    .unwrap();
    writeln!(s, "/// Response type for the {} RPC call.", type_name).unwrap();
    if !formatted_description.trim().is_empty() {
        writeln!(s, "{}", formatted_description).unwrap();
    }
    writeln!(
        s,
        "#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]"
    )
    .unwrap();
    writeln!(s, "pub struct {} {{", type_name).unwrap();
    writeln!(s, "{}", fields).unwrap();
    writeln!(s, "}}\n").unwrap();

    Some(s)
}

fn get_return_type(result: &ApiResult) -> String {
    if result.type_.eq_ignore_ascii_case("object") && !result.inner.is_empty() {
        "serde_json::Value".to_string()
    } else {
        map_type_to_rust(&result.type_)
    }
}

// fn generate_struct(type_name: &str, description: &str, fields: &str) -> String {
//     let mut s = String::new();
//     writeln!(s, "/// Response for the {} RPC call.", type_name).unwrap();
//     if !description.trim().is_empty() {
//         writeln!(s, "{}", description).unwrap();
//     }
//     writeln!(
//         s,
//         "#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]"
//     )
//     .unwrap();
//     writeln!(s, "pub struct {} {{", type_name).unwrap();
//     writeln!(s, "{}", fields).unwrap();
//     writeln!(s, "}}\n").unwrap();
//     s
// }

fn generate_struct_fields(result: &ApiResult) -> String {
    let mut fields = String::new();
    for field in &result.inner {
        let field_name = sanitize_field_name(&field.key_name);
        let field_type = get_field_type(field);
        fields.push_str(&format_struct_field(
            &field_name,
            &field_type,
            &field.description,
        ));
    }
    fields
}

/// Given a version tag (e.g. "v29"), a list of ApiMethods,
/// and an output directory, emit the minimal file layout so that
/// generator_tests.rs will pass.
pub fn generate_version_code(version: &str, methods: &[ApiMethod], out_dir: &Path) -> Result<()> {
    // Create version directory
    let version_dir = out_dir.join(version);
    fs::create_dir_all(&version_dir)?;

    // Generate client stubs
    for method in methods {
        let fname = sanitize_method_name(&method.name) + ".rs";
        let mut file = fs::File::create(version_dir.join(&fname))?;
        let macro_code = generate_client_macro(method, version);
        write!(file, "{}", macro_code)?;
    }

    // Generate mod.rs
    {
        let mut mod_rs = fs::File::create(version_dir.join("mod.rs"))?;
        writeln!(
            mod_rs,
            "//! This file is auto-generated. Do not edit manually.\n"
        )?;
        for method in methods {
            let modname = sanitize_method_name(&method.name);
            writeln!(mod_rs, "pub mod {};", modname)?;
        }
    }

    // Generate type definitions in the same directory
    for method in methods {
        let fname = sanitize_method_name(&method.name) + "_types.rs";
        let mut file = fs::File::create(version_dir.join(&fname))?;
        if let Some(type_code) = generate_return_type(method) {
            write!(file, "{}", type_code)?;
        }
    }

    Ok(())
}

/// Generate the top-level mod.rs files for client and types directories.
pub fn generate_mod_rs(output_dir: &str, versions: &[&str]) -> std::io::Result<()> {
    let mod_rs = "pub mod client;\npub mod types;\n";
    fs::write(Path::new(output_dir).join("mod.rs"), mod_rs)?;

    let mut client_mod = String::new();
    for v in versions {
        writeln!(client_mod, "pub use self::{}::*;", v).unwrap();
    }
    let client_path = Path::new(output_dir).join("client").join("mod.rs");
    fs::create_dir_all(client_path.parent().unwrap())?;
    fs::write(client_path, client_mod)?;

    let mut types_mod = String::new();
    for v in versions {
        writeln!(types_mod, "pub use self::{}_types::*;", v).unwrap();
    }
    let types_path = Path::new(output_dir).join("types").join("mod.rs");
    fs::create_dir_all(types_path.parent().unwrap())?;
    fs::write(types_path, types_mod)?;

    Ok(())
}

/// Generate the top-level client mod.rs file.
pub fn generate_client_mod_rs(versions: &[String], out_dir: &Path) -> std::io::Result<()> {
    generate_versioned_mod_rs(versions, out_dir, "client", "pub mod {};")
}

/// Generate the top-level types mod.rs file.
pub fn generate_types_mod_rs(versions: &[String], out_dir: &Path) -> std::io::Result<()> {
    generate_versioned_mod_rs(versions, out_dir, "types", "pub mod {}_types;")
}

/// Helper function to generate a mod.rs file declaring versioned modules.
fn generate_versioned_mod_rs(
    versions: &[String],
    out_dir: &Path,
    subdir: &str,
    mod_template: &str,
) -> std::io::Result<()> {
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

/// Generate the top-level `src/client/mod.rs` file that declares all versioned client modules.
pub fn generate_top_level_client_mod(versions: &[String], src_dir: &Path) -> std::io::Result<()> {
    use std::fmt::Write; // Import the Write trait for writeln!

    let mut client_mod_rs = String::new();
    writeln!(
        client_mod_rs,
        "// Auto-generated client module declarations."
    )
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    writeln!(client_mod_rs, "// Do not edit this file manually.")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    writeln!(client_mod_rs).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?; // Add a blank line

    for version in versions {
        writeln!(client_mod_rs, "pub mod {}", version)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }
    writeln!(client_mod_rs).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?; // Add a blank line

    for version in versions {
        writeln!(client_mod_rs, "pub use self::{}::*;", version)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }

    let client_mod_path = src_dir.join("client").join("mod.rs");
    fs::create_dir_all(client_mod_path.parent().unwrap())?;
    fs::write(&client_mod_path, client_mod_rs)?;
    Ok(())
}

/// Generate the top-level `src/types/mod.rs` file that declares all versioned types modules.
pub fn generate_top_level_types_mod(versions: &[String], src_dir: &Path) -> std::io::Result<()> {
    use std::fmt::Write; // Import the Write trait for writeln!

    let mut types_mod_rs = String::new();
    writeln!(types_mod_rs, "// Auto-generated types module declarations.")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    writeln!(types_mod_rs, "// Do not edit this file manually.")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    writeln!(types_mod_rs).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?; // Add a blank line

    for version in versions {
        writeln!(types_mod_rs, "pub mod {}_types;", version)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }
    writeln!(types_mod_rs).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?; // Add a blank line

    for version in versions {
        writeln!(types_mod_rs, "pub use self::{}_types::*;", version)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }

    let types_mod_path = src_dir.join("types").join("mod.rs");
    fs::create_dir_all(types_mod_path.parent().unwrap())?;
    fs::write(&types_mod_path, types_mod_rs)?;
    Ok(())
}

/// Generate a type conversion implementation for a method.
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
