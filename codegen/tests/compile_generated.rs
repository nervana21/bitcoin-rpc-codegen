use codegen::{CodeGenerator, TransportCodeGenerator};
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
            key_name: "chain".into(),
            type_: "string".into(),
            description: "Current network name".into(),
            inner: vec![],
            required: true,
        }],
    }];

    let gen = TransportCodeGenerator;
    let files = gen.generate(&methods);

    // Verify basic code generation
    assert_eq!(files.len(), 1);
    let (mod_name, src) = &files[0];
    assert_eq!(mod_name, "getblockchaininfo");
    assert!(src.contains("pub async fn getblockchaininfo"));
    assert!(src.contains("Transport"));
    assert!(src.contains("chain"));
}

#[test]
fn transport_codegen_with_arguments() {
    // Test code generation with method arguments
    let methods = vec![ApiMethod {
        name: "getblock".into(),
        description: "Returns block information.".into(),
        arguments: vec![rpc_api::ApiArgument {
            names: vec!["blockhash".into()],
            type_: "string".into(),
            description: "The block hash".into(),
            required: true,
        }],
        results: vec![ApiResult {
            key_name: "hash".into(),
            type_: "string".into(),
            description: "The block hash".into(),
            inner: vec![],
            required: true,
        }],
    }];

    let gen = TransportCodeGenerator;
    let files = gen.generate(&methods);

    // Verify argument handling
    assert_eq!(files.len(), 1);
    let (mod_name, src) = &files[0];
    assert_eq!(mod_name, "getblock");
    assert!(src.contains("pub async fn getblock"));
    assert!(src.contains("blockhash: serde_json::Value"));
    assert!(src.contains("hash"));
}
