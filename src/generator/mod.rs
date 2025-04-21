use crate::parser::{ApiMethod, ApiResult};

use std::fs;
use std::io;
use std::path::Path;

/// Hardcoded list of supported client versions (corresponds to generated files).
pub const SUPPORTED_VERSIONS: &[&str] = &["v27", "v28"];

/// Generates a struct definition string from a type name, a description, and its fields.
pub fn generate_struct(type_name: &str, description: &str, fields: &str) -> String {
    format!(
        r#"/// Response for the {} RPC call.
{}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct {} {{{}}}"#,
        type_name, description, type_name, fields
    )
}

pub fn generate_client_macro(method: &ApiMethod, version: &str) -> String {
    let method_name = sanitize_method_name(&method.name);
    let macro_name = format!("impl_client_{}__{}", version, method_name);
    let description = format_doc_comment(&method.description);
    let mut function_defs = Vec::new();

    // Compute return type once
    let return_ty = get_return_type_from_results(&method.results);

    // Helper to format the call expression with correct empty‑slice cast
    let make_call = |req: &str, opt: &str| {
        if req.is_empty() && opt.is_empty() {
            // no required, no optional
            format!(
                "self.call(\"{}\", &[] as &[serde_json::Value])",
                method.name
            )
        } else if !opt.is_empty() {
            // required + optional
            format!(
                "let mut params = vec![{}];\n{}\n    self.call(\"{}\", &params)",
                req, opt, method.name
            )
        } else {
            // only required
            format!("self.call(\"{}\", &[{}])", method.name, req)
        }
    };

    if !method.arguments.is_empty() && method.arguments.iter().all(|arg| arg.optional) {
        // 1) default‐params variant
        let default_doc = format!("{} with default parameters.", description);
        let default_fn = format!(
            "/// {doc}\npub fn {name}_default(&self) -> Result<{ret}> {{
    {call}
}}",
            doc = default_doc,
            name = method_name,
            ret = return_ty,
            call = make_call("", "")
        );

        // 2) specified‐params variant
        let param_doc = format!("{} with specified parameters.", description);
        let method_args = generate_method_args(method);
        let (required_args, optional_body_raw) = generate_args(method);
        let optional_body = optional_body_raw
            .lines()
            .map(|line| format!("    {}", line))
            .collect::<Vec<_>>()
            .join("\n");
        let param_call = make_call(&required_args, &optional_body);
        let param_fn = format!(
            "/// {doc}\npub fn {name}(&self{args}) -> Result<{ret}> {{
    {call}
}}",
            doc = param_doc,
            name = method_name,
            args = method_args,
            ret = return_ty,
            call = param_call
        );

        function_defs.push(default_fn);
        function_defs.push(param_fn);
    } else {
        // Single‐method variant
        let method_args = generate_method_args(method);
        let (required_args, optional_body_raw) = generate_args(method);
        let optional_body = optional_body_raw
            .lines()
            .map(|line| format!("    {}", line))
            .collect::<Vec<_>>()
            .join("\n");
        let raw_call = make_call(&required_args, &optional_body);

        // If this RPC returns `null`, wrap it in a match
        let call_body = if method
            .results
            .iter()
            .any(|r| r.type_.eq_ignore_ascii_case("none"))
        {
            format!(
                "match {call} {{
    Ok(serde_json::Value::Null) => Ok(()),
    Ok(ref val) if val.is_null() => Ok(()),
    Ok(other) => Err(crate::client_sync::Error::Returned(format!(\"{method} expected null, got: {{}}\", other))),
    Err(e) => Err(e.into()),
}}",
                call = raw_call,
                method = method.name
            )
        } else {
            raw_call
        };

        let fn_def = format!(
            "pub fn {name}(&self{args}) -> Result<{ret}> {{
    {body}
}}",
            name = method_name,
            args = method_args,
            ret = return_ty,
            body = call_body
        );

        function_defs.push(fn_def);
    }

    let impl_block = function_defs.join("\n\n");
    format!(
        "/// Implements Bitcoin Core JSON-RPC API method `{}` for version {}\n{}\n#[macro_export]\n\
         macro_rules! {} {{\n    () => {{\n        impl Client {{\n{}\n        }}\n    }};\n}}",
        method.name,
        version,
        description,
        macro_name,
        indent(&impl_block, 12)
    )
}

pub fn generate_return_type(method: &ApiMethod) -> Option<String> {
    if method.results.is_empty() || method.results[0].type_.to_lowercase() == "none" {
        return None;
    }
    let result = &method.results[0];
    let type_name = format!("{}Response", capitalize(&method.name));
    let formatted_description = format_doc_comment(&method.description);
    let fields = if result.inner.is_empty() {
        format!("    pub result: {},", get_return_type(result))
    } else {
        generate_struct_fields(result)
    };
    Some(generate_struct(&type_name, &formatted_description, &fields))
}

pub fn generate_type_conversion(method: &ApiMethod, _version: &str) -> Option<String> {
    if method
        .results
        .iter()
        .any(|r| r.type_.to_lowercase() == "none")
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

fn map_type_to_rust(type_str: &str) -> String {
    match type_str {
        "string" => "String".to_string(),
        "number" => "f64".to_string(),
        "boolean" => "bool".to_string(),
        "hex" => "String".to_string(),
        "object" => "serde_json::Value".to_string(),
        "object_dynamic" => "serde_json::Value".to_string(),
        _ => "serde_json::Value".to_string(),
    }
}

fn get_return_type(result: &ApiResult) -> String {
    if result.type_.as_str() == "object" && !result.inner.is_empty() {
        generate_object_type(result)
    } else {
        map_type_to_rust(result.type_.as_str())
    }
}

fn get_return_type_from_results(results: &[ApiResult]) -> String {
    if results.is_empty() {
        "()".to_string()
    } else {
        get_return_type(&results[0])
    }
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
    description
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| format!("/// {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_struct_field(field_name: &str, field_type: &str, description: &str) -> String {
    let formatted_description = format_doc_comment(description);
    format!(
        "{}\n    pub {}: {},\n",
        formatted_description, field_name, field_type
    )
}

fn generate_struct_fields(result: &ApiResult) -> String {
    let mut fields = String::new();
    for field in &result.inner {
        let field_name = if field.key_name.is_empty() {
            "result".to_string()
        } else {
            sanitize_field_name(&field.key_name)
        };
        let field_type = get_field_type(field);
        fields.push_str(&format_struct_field(
            &field_name,
            &field_type,
            &field.description,
        ));
    }
    fields
}

fn get_field_type(field: &ApiResult) -> String {
    if field.type_.as_str() == "array" || field.type_.as_str() == "array-fixed" {
        format!("Vec<{}>", generate_array_type(field))
    } else if field.type_.as_str() == "object" && !field.inner.is_empty() {
        if field.key_name == "object_dynamic" {
            "serde_json::Value".to_string()
        } else {
            generate_object_type(field)
        }
    } else {
        map_type_to_rust(field.type_.as_str())
    }
}

fn sanitize_method_name(name: &str) -> String {
    name.replace("-", "_").to_lowercase()
}

fn sanitize_field_name(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace(".", "_")
}

fn capitalize(s: &str) -> String {
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

fn generate_object_type(result: &ApiResult) -> String {
    if result.inner.is_empty() || result.key_name == "object_dynamic" {
        "serde_json::Value".to_string()
    } else {
        let base_name = if result.key_name.is_empty() {
            "Value".to_string()
        } else {
            capitalize(&result.key_name)
        };
        format!("serde_json::{}", base_name)
    }
}

fn generate_array_type(result: &ApiResult) -> String {
    if let Some(inner) = result.inner.first() {
        match inner.type_.as_str() {
            "string" => "String".to_string(),
            "number" => "f64".to_string(),
            "hex" => "String".to_string(),
            _ => "serde_json::Value".to_string(),
        }
    } else {
        "serde_json::Value".to_string()
    }
}

fn indent(s: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    s.lines()
        .map(|line| format!("{}{}", pad, line))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn generate_mod_rs(output_dir: &str, versions: &[&str]) -> std::io::Result<()> {
    use std::fmt::Write; // Import the Write trait for writeln!
    let mod_rs_content = "pub mod client;\npub mod types;\n";
    fs::write(Path::new(output_dir).join("mod.rs"), mod_rs_content)?;

    // Client mod.rs
    let mut client_mod_rs = String::new();
    for version in versions {
        writeln!(client_mod_rs, "pub use self::{}::*;", version)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
    let client_mod_path = Path::new(output_dir).join("client").join("mod.rs");
    fs::create_dir_all(client_mod_path.parent().unwrap())?;
    fs::write(client_mod_path, client_mod_rs)?;

    // Types mod.rs
    let mut types_mod_rs = String::new();
    for version in versions {
        writeln!(types_mod_rs, "pub use self::{}_types::*;", version)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
    let types_mod_path = Path::new(output_dir).join("types").join("mod.rs");
    fs::create_dir_all(types_mod_path.parent().unwrap())?;
    fs::write(types_mod_path, types_mod_rs)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{ApiMethod, ApiResult};

    fn create_test_method() -> ApiMethod {
        ApiMethod {
            name: "test-method".to_string(),
            description: "Test method description".to_string(),
            arguments: vec![],
            results: vec![ApiResult {
                type_: "string".to_string(),
                description: "Test result".to_string(),
                key_name: "".to_string(),
                inner: vec![],
            }],
        }
    }

    #[test]
    fn test_generate_client_macro() {
        let method = create_test_method();
        let macro_code = generate_client_macro(&method, "v21");
        assert!(macro_code.contains("impl_client_v21__test_method"));
        assert!(macro_code.contains("Test method description"));
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("test-method"), "TestMethod");
    }

    #[test]
    fn test_generate_return_type() {
        let method = create_test_method();
        let type_code = generate_return_type(&method).unwrap();
        assert!(type_code.contains("TestMethodResponse"));
    }

    #[test]
    fn test_generate_type_conversion() {
        let method = create_test_method();
        let conversion_code = generate_type_conversion(&method, "v21").unwrap();
        assert!(conversion_code.contains("impl TestMethodResponse"));
    }

    #[test]
    fn test_sanitize_method_name() {
        assert_eq!(sanitize_method_name("test-method"), "test_method");
    }
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
        writeln!(client_mod_rs, "pub mod {};", version)
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
