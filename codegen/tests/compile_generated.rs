use codegen::{write_generated, CodeGenerator, TransportCodeGenerator};
use rpc_api::{parse_api_json, ApiMethod, ApiResult};
use tempfile::TempDir;

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
            optional: false,
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
            optional: false,
        }],
        results: vec![ApiResult {
            key_name: "hash".into(),
            type_: "string".into(),
            description: "The block hash".into(),
            inner: vec![],
            optional: false,
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

#[tokio::test]
async fn test_generate_types_from_schema() {
    // 1. Read API schema
    let schema_json = include_str!("../../api.json");
    let methods = parse_api_json(schema_json).expect("Failed to parse API schema");

    // 2. Generate types
    let generator = TransportCodeGenerator;
    let generated_files = generator.generate(&methods);

    // 3. Write to temp directory
    let tmp = TempDir::new().expect("Failed to create temp dir");
    write_generated(tmp.path(), &generated_files).expect("Failed to write generated files");

    // 4. Verify files were generated
    for (name, _) in &generated_files {
        let file_path = tmp.path().join(format!("{}.rs", name));
        assert!(
            file_path.exists(),
            "Expected {} to exist",
            file_path.display()
        );
    }
}
