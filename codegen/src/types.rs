use rpc_api::ApiMethod;
use std::fmt::Write;

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

/// Sanitize a field name into a valid Rust identifier
pub fn sanitize_field_name(name: &str) -> String {
    let filtered: String = name
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c == '_' || c == '-' {
                Some('_')
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
pub fn get_field_type(field: &rpc_api::ApiResult) -> String {
    match field.type_.as_str() {
        "array" | "array-fixed" => "Vec<serde_json::Value>".to_string(),
        "object" => "serde_json::Value".to_string(),
        _ => map_type_to_rust(&field.type_),
    }
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
    let fields = format!("    pub {}: {},", result.key_name, get_field_type(result));

    let mut s = String::new();
    writeln!(
        s,
        "//! This file is auto-generated. Do not edit manually.\n"
    )
    .unwrap();
    writeln!(s, "/// Response type for the {} RPC call.", type_name).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use rpc_api::ApiMethod;

    #[test]
    fn test_map_type_to_rust() {
        assert_eq!(map_type_to_rust("string"), "String");
        assert_eq!(map_type_to_rust("number"), "f64");
        assert_eq!(map_type_to_rust("boolean"), "bool");
        assert_eq!(map_type_to_rust("hex"), "String");
        assert_eq!(map_type_to_rust("object"), "serde_json::Value");
        assert_eq!(map_type_to_rust("unknown"), "String");
    }

    #[test]
    fn test_sanitize_method_name() {
        assert_eq!(sanitize_method_name("get-block-count"), "get_block_count");
        assert_eq!(sanitize_method_name("getBlockCount"), "getblockcount");
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("get-block-count"), "GetBlockCount");
        assert_eq!(capitalize("getblockcount"), "Getblockcount");
    }

    #[test]
    fn test_sanitize_field_name() {
        assert_eq!(sanitize_field_name("block-count"), "block_count");
        assert_eq!(sanitize_field_name("123count"), "_123count");
        assert_eq!(sanitize_field_name("block@count"), "blockcount");
    }

    #[test]
    fn test_generate_return_type() {
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

        let result = generate_return_type(&method).unwrap();
        assert!(result.contains("pub struct GetblockcountResponse"));
        assert!(result.contains("pub count: f64"));
    }
}
