use crate::parser::{ApiMethod, ApiResult};
use std::fs;
use std::path::Path;

pub fn generate_client_macro(method: &ApiMethod, version: &str) -> String {
    let method_name = sanitize_method_name(&method.name);
    let macro_name = format!("impl_client_{}__{}", version, method_name);
    let return_type = if method
        .results
        .iter()
        .any(|r| r.type_.to_lowercase() == "none")
    {
        "()".to_string()
    } else {
        get_return_type_from_results(&method.results)
    };

    let description = format_doc_comment(&method.description);

    let (required_args, optional_args) = generate_args(method);
    let call_args = if !optional_args.is_empty() {
        format!(
            r#"let mut params = vec![{}];
{}
                self.call("{}", &params)"#,
            required_args, optional_args, method.name
        )
    } else {
        format!(r#"self.call("{}", &[{}])"#, method.name, required_args)
    };

    generate_macro(
        &method.name,
        version,
        &description,
        &macro_name,
        &generate_method_args(method),
        &return_type,
        &call_args,
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

pub fn generate_type_conversion(method: &ApiMethod, version: &str) -> Option<String> {
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
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    /// This conversion is specific to version {}.
    pub fn into_model(self) -> Result<{}, {}Error> {{
        Ok(())
    }}
}}"#,
        type_name, version, model_type, type_name
    ))
}

fn map_type_to_rust(type_str: &str) -> String {
    match type_str {
        "boolean" => "bool".to_string(),
        "string" => "String".to_string(),
        "number" => "f64".to_string(),
        "hex" => "Hex".to_string(),
        "amount" => "Amount".to_string(),
        "time" => "Time".to_string(),
        "object" | "elision" => "serde_json::Value".to_string(),
        _ => type_str.replace("-", "_").to_string(),
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
            "object" => "serde_json::Value".to_string(),
            "object-named-parameters" => "serde_json::Value".to_string(),
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
        let arg_type = match arg.type_.as_str() {
            "hex" => "String",
            "string" => "String",
            "number" => "i64",
            "boolean" => "bool",
            "array" => {
                if arg_name == "inputs" {
                    "Vec<Input>"
                } else if arg_name == "outputs" {
                    "Vec<Output>"
                } else {
                    "Vec<String>"
                }
            }
            "object" => "serde_json::Value",
            "object-named-parameters" => "serde_json::Value",
            _ => &arg.type_,
        };

        if arg.optional {
            optional_args.push(format!(
                "                if let Some({}) = {} {{
                    params.push(into_json({})?);
                }}",
                arg_name, arg_name, arg_name
            ));
        } else {
            required_args.push(format!("{}: {}", arg_name, arg_type));
        }
    }

    (required_args.join(", "), optional_args.join("\n"))
}

fn format_doc_comment(description: &str) -> String {
    description
        .split('\n')
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
        generate_object_type(field)
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
    if result.inner.is_empty() {
        "serde_json::Value".to_string()
    } else {
        // Use a more descriptive name for the object type
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
            "boolean" => "bool".to_string(),
            "object" => generate_object_type(inner),
            "hex" => "Hex".to_string(),
            "amount" => "Amount".to_string(),
            "time" => "Time".to_string(),
            "elision" => "serde_json::Value".to_string(),
            _ => inner.type_.clone(),
        }
    } else {
        "serde_json::Value".to_string()
    }
}

fn generate_macro(
    method_name: &str,
    version: &str,
    description: &str,
    macro_name: &str,
    method_args: &str,
    return_type: &str,
    call_args: &str,
) -> String {
    format!(
        r#"/// Implements Bitcoin Core JSON-RPC API method `{}` for version {}
///
{}
#[macro_export]
macro_rules! {} {{
    () => {{
        impl Client {{
            pub fn {}(&self{}) -> Result<{}> {{
                {}
            }}
        }}
    }};
}}"#,
        method_name,
        version,
        description,
        macro_name,
        method_name,
        method_args,
        return_type,
        call_args
    )
}

fn generate_struct(type_name: &str, description: &str, fields: &str) -> String {
    format!(
        r#"/// Response for the {} RPC call.
///
{}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct {} {{{}}}"#,
        type_name, description, type_name, fields
    )
}

pub fn generate_mod_rs(output_dir: &str, versions: &[&str]) -> std::io::Result<()> {
    // Create mod.rs for the main directory
    let mod_rs_content = "pub mod client;\npub mod types;\n";
    let mod_rs_path = Path::new(output_dir).join("mod.rs");
    fs::write(mod_rs_path, mod_rs_content)?;

    // Create mod.rs for the client directory
    let client_mod_rs_content: String = versions
        .iter()
        .map(|version| format!("pub use self::{}::*;\n", version))
        .collect();
    let client_mod_rs_path = Path::new(output_dir).join("client").join("mod.rs");
    fs::create_dir_all(client_mod_rs_path.parent().unwrap())?;
    fs::write(client_mod_rs_path, client_mod_rs_content)?;

    // Create mod.rs for the types directory
    let types_mod_rs_content: String = versions
        .iter()
        .map(|version| format!("pub use self::{}_types::*;\n", version))
        .collect();
    let types_mod_rs_path = Path::new(output_dir).join("types").join("mod.rs");
    fs::create_dir_all(types_mod_rs_path.parent().unwrap())?;
    fs::write(types_mod_rs_path, types_mod_rs_content)?;

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
            category: "test-category".to_string(),
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
