use codegen::{CodeGenerator, TransportCodeGenerator};
use rpc_api::version::DEFAULT_VERSION;
use rpc_api::{ApiMethod, ApiResult};

#[test]
fn transport_codegen_basic_functionality() {
    // Test basic code generation with a simple method (no arguments)
    let methods = vec![ApiMethod {
        name: "getblockchaininfo".into(),
        description:
            "Returns an object containing various state info regarding blockchain processing."
                .into(),
        arguments: vec![],
        results: vec![ApiResult {
            key_name: "result".into(),
            type_: "string".into(),
            description: "".into(),
            inner: vec![],
            required: true,
        }],
    }];

    let gen = TransportCodeGenerator::new(&DEFAULT_VERSION.as_str_lowercase());
    let files = gen.generate(&methods);

    // Verify basic code generation
    assert_eq!(files.len(), 1);
    let (mod_name, src) = &files[0];
    assert_eq!(mod_name, "getblockchaininfo");

    // Verify the generated code contains expected elements
    assert!(src.contains("pub async fn getblockchaininfo"));
    assert!(src.contains("transport: &dyn TransportTrait"));
    assert!(src.contains("TransportError"));
}

#[test]
fn transport_codegen_with_arguments() {
    // Test code generation with a method that has arguments
    let methods = vec![ApiMethod {
        name: "getblock".into(),
        description: "Returns information about the block with the given hash.".into(),
        arguments: vec![rpc_api::ApiArgument {
            names: vec!["blockhash".into()],
            type_: "string".into(),
            required: true,
            description: "The block hash".into(),
        }],
        results: vec![],
    }];

    let gen = TransportCodeGenerator::new(&DEFAULT_VERSION.as_str_lowercase());
    let files = gen.generate(&methods);

    // Verify argument handling
    assert_eq!(files.len(), 1);
    let (mod_name, src) = &files[0];
    assert_eq!(mod_name, "getblock");

    // Verify the generated code handles arguments correctly
    assert!(src.contains("pub async fn getblock"));
    assert!(src.contains("transport: &dyn TransportTrait"));
    assert!(src.contains("blockhash: serde_json::Value"));
    assert!(src.contains("vec![json!(blockhash)]"));
}
