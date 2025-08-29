use codegen::{CodeGenerator, TransportCodeGenerator};
use rpc_api::version::DEFAULT_VERSION;
use rpc_api::{ApiArgument, ApiMethod};

#[test]
fn test_json_rpc_codegen_transport() {
    let methods = vec![ApiMethod {
        name: "foo".into(),
        description: "".into(),
        arguments: vec![],
        results: vec![],
    }];

    let gen = TransportCodeGenerator::new(DEFAULT_VERSION.as_str_lowercase());
    let files = gen.generate(&methods);

    // should produce exactly one module named "foo"
    assert_eq!(files.len(), 1);
    let (mod_name, src) = &files[0];
    assert_eq!(mod_name, "foo");

    // verify the generated source contains expected elements
    assert!(src.contains("pub async fn foo"));
    assert!(src.contains("transport: &dyn TransportTrait"));
    assert!(src.contains("TransportError"));
}

#[test]
fn transport_codegen_with_args() {
    let methods = vec![ApiMethod {
        name: "foo".into(),
        description: "".into(),
        arguments: vec![
            ApiArgument {
                names: vec!["arg1".into()],
                description: "".into(),
                required: true,
                type_: "string".into(),
                type_str: None,
            },
            ApiArgument {
                names: vec!["arg2".into()],
                description: "".into(),
                required: true,
                type_: "number".into(),
                type_str: None,
            },
        ],
        results: vec![],
    }];

    let gen = TransportCodeGenerator::new(DEFAULT_VERSION.as_str_lowercase());
    let files = gen.generate(&methods);
    assert_eq!(files.len(), 1);

    let (_mod, src) = &files[0];
    // 1) signature includes both args
    assert!(src.contains(
        "pub async fn foo(transport: &dyn TransportTrait, arg1: serde_json::Value, arg2: serde_json::Value)"
    ));
    // 2) params vec serializes them in order
    assert!(src.contains("let params = vec![json!(arg1), json!(arg2)];"));
    // 3) send_request call remains correct
    assert!(src.contains("transport.send_request(\"foo\", &params).await"));
}

#[test]
fn transport_codegen_error_handling() {
    let methods = vec![ApiMethod {
        name: "foo".into(),
        description: "".into(),
        arguments: vec![],
        results: vec![],
    }];

    let gen = TransportCodeGenerator::new(DEFAULT_VERSION.as_str_lowercase());
    let files = gen.generate(&methods);
    assert_eq!(files.len(), 1);

    let (_mod, src) = &files[0];
    // Verify error handling is properly generated
    assert!(src.contains("Result<"));
    assert!(src.contains("TransportError"));
    assert!(src.contains(".await?"));
}

#[test]
fn transport_codegen_imports() {
    let methods = vec![ApiMethod {
        name: "foo".into(),
        description: "".into(),
        arguments: vec![],
        results: vec![],
    }];

    let gen = TransportCodeGenerator::new(DEFAULT_VERSION.as_str_lowercase());
    let files = gen.generate(&methods);
    assert_eq!(files.len(), 1);

    let (_mod, src) = &files[0];
    // Verify required imports are present
    assert!(
        src.contains("use serde_json::{Value, json};"),
        "Missing serde_json imports"
    );
    assert!(
        src.contains("use transport::{TransportTrait, TransportError};"),
        "Missing transport imports"
    );
}
