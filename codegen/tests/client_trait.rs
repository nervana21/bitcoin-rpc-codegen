use codegen::generators::client_trait::{render_client_trait, ClientTraitGenerator};
use codegen::CodeGenerator;
use types::ApiArgument;
use types::ApiMethod;
use types::ApiResult;

fn make_method(
    name: &str,
    description: &str,
    arguments: Vec<ApiArgument>,
    results: Vec<ApiResult>,
) -> ApiMethod {
    ApiMethod {
        name: name.to_string(),
        description: description.to_string(),
        arguments,
        results,
    }
}

#[test]
fn generate_no_methods() {
    let gen = ClientTraitGenerator::new("v29");
    let files = gen.generate(&[]);
    assert!(files.iter().any(|(name, _)| name == "client_trait.rs"));
    let trait_file = files
        .iter()
        .find(|(name, _)| name == "client_trait.rs")
        .unwrap();
    assert!(trait_file.1.contains("pub trait BitcoinClientV29"));
}

#[test]
fn generate_single_method() {
    let gen = ClientTraitGenerator::new("v29");
    let method = make_method(
        "getblock",
        "Get a block.",
        vec![ApiArgument {
            names: vec!["blockhash".into()],
            type_: "string".into(),
            type_str: None,
            required: true,
            description: "Block hash".into(),
        }],
        vec![ApiResult {
            key_name: "block".into(),
            type_: "object".into(),
            description: "Block object".into(),
            inner: vec![],
            required: true,
        }],
    );
    let files = gen.generate(&[method]);
    let trait_file = files
        .iter()
        .find(|(name, _)| name == "client_trait.rs")
        .unwrap();
    assert!(trait_file.1.contains("async fn getblock"));
    assert!(trait_file.1.contains("/// Get a block."));
    assert!(trait_file.1.contains("_blockhash: bitcoin::BlockHash"));
    assert!(trait_file
        .1
        .contains("-> Result<GetblockResponse, TransportError>"));
}

#[test]
fn generate_optional_and_reserved_arg() {
    let gen = ClientTraitGenerator::new("v29");
    let method = make_method(
        "getinfo",
        "Get info.",
        vec![
            ApiArgument {
                names: vec!["type".into()],
                type_: "string".into(),
                type_str: None,
                required: false,
                description: "Type arg".into(),
            },
            ApiArgument {
                names: vec!["foo".into()],
                type_: "number".into(),
                type_str: None,
                required: false,
                description: "Foo arg".into(),
            },
        ],
        vec![],
    );
    let files = gen.generate(&[method]);
    let trait_file = files
        .iter()
        .find(|(name, _)| name == "client_trait.rs")
        .unwrap();
    assert!(trait_file.1.contains("r#_type: Option<String>"));
    assert!(trait_file.1.contains("_foo: Option<u64>"));
}

#[test]
fn render_client_trait_substitutes_all() {
    let template = r#"
        // {{IMPORTS}}
        pub trait BitcoinClient{{VERSION_NODOTS}} {
        {{TRAIT_METHODS}}
        }
    "#;
    let method = make_method(
        "getblock",
        "Get a block.",
        vec![ApiArgument {
            names: vec!["blockhash".into()],
            type_: "string".into(),
            type_str: None,
            required: true,
            description: "Block hash".into(),
        }],
        vec![ApiResult {
            key_name: "block".into(),
            type_: "object".into(),
            description: "Block object".into(),
            inner: vec![],
            required: true,
        }],
    );
    let out = render_client_trait(template, &[method], "v29");
    assert!(out.contains("use crate::types::*"));
    assert!(out.contains("pub trait BitcoinClientV29"));
    assert!(out.contains("async fn getblock"));
}

#[test]
fn json_params_generates_correct_parameter_strings() {
    use codegen::generators::client_trait::MethodTemplate;

    let method = make_method(
        "testmethod",
        "Test method",
        vec![
            ApiArgument {
                names: vec!["param1".into()],
                type_: "string".into(),
                type_str: None,
                required: true,
                description: "First param".into(),
            },
            ApiArgument {
                names: vec!["type".into()],
                type_: "string".into(),
                type_str: None,
                required: true,
                description: "Type param".into(),
            },
        ],
        vec![],
    );

    let template = MethodTemplate::new(&method);
    let json_params = template.json_params();

    // Should contain the correct parameter names
    assert!(json_params.contains("_param1"));
    assert!(json_params.contains("r#_type"));
    assert!(json_params.contains("serde_json::json!"));
}

#[test]
fn json_params_handles_empty_arguments() {
    use codegen::generators::client_trait::MethodTemplate;

    let method = make_method("nomethod", "No args method", vec![], vec![]);

    let template = MethodTemplate::new(&method);
    let json_params = template.json_params();

    // Should be empty string when no arguments
    assert_eq!(json_params, "");
}

#[test]
fn json_params_handles_reserved_keyword_type() {
    use codegen::generators::client_trait::MethodTemplate;

    let method = make_method(
        "testmethod",
        "Test method",
        vec![ApiArgument {
            names: vec!["type".into()],
            type_: "string".into(),
            type_str: None,
            required: true,
            description: "Type param".into(),
        }],
        vec![],
    );

    let template = MethodTemplate::new(&method);
    let json_params = template.json_params();

    // Should use r#_type for reserved keyword
    assert!(json_params.contains("r#_type"));
    // The parameter should be properly escaped, not just contain the substring
    assert!(json_params.trim().starts_with("serde_json::json!(r#_type"));
}
