use codegen::{write_generated, CodeGenerator, TransportCodeGenerator, TypesCodeGenerator};
use rpc_api::{parse_api_json, ApiMethod, ApiResult};
use std::{fs, process::Command};
use tempfile::TempDir;

/// Helper function to create a temporary Cargo project for testing
fn setup_test_project() -> (TempDir, std::path::PathBuf) {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let project = tmp.path();

    // Write Cargo.toml
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    fs::write(
        project.join("Cargo.toml"),
        format!(
            r#"[package]
name = "gen_test"
version = "0.1.0"
edition = "2021"

[dependencies]
transport = {{ path = "{}/../transport" }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1"
tokio = {{ version = "1", features = ["full"] }}
bitcoin = "0.32.6"

[lib]
path = "src/lib.rs"
"#,
            manifest_dir
        ),
    )
    .expect("write Cargo.toml");

    // Prepare src/
    let src = project.join("src");
    fs::create_dir(&src).expect("create src dir");

    (tmp, src)
}

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

#[test]
fn response_type_generation_with_special_fields() {
    // Test handling of special field names in response types
    let (tmp, src) = setup_test_project();

    // Create generated/types module
    let generated_dir = src.join("generated");
    fs::create_dir(&generated_dir).expect("create generated dir");
    let types_dir = generated_dir.join("types");
    fs::create_dir(&types_dir).expect("create types dir");

    let methods = vec![ApiMethod {
        name: "test_method".into(),
        description: "Test method with special fields".into(),
        arguments: vec![],
        results: vec![
            ApiResult {
                key_name: "bip125-replaceable".into(),
                type_: "boolean".into(),
                description: "Test field with hyphen".into(),
                optional: false,
                inner: vec![],
            },
            ApiResult {
                key_name: "type".into(),
                type_: "string".into(),
                description: "Test field with keyword".into(),
                optional: false,
                inner: vec![],
            },
            ApiResult {
                key_name: "normal-field".into(),
                type_: "string".into(),
                description: "Another test field".into(),
                optional: true,
                inner: vec![],
            },
        ],
    }];

    // Generate types
    let type_gen = TypesCodeGenerator;
    let type_files = type_gen.generate(&methods);
    write_generated(&types_dir, &type_files).expect("write_generated types");

    // Write lib.rs - only include the generated types
    let mut lib_rs = String::new();
    lib_rs.push_str("pub mod generated;\n");
    fs::write(src.join("lib.rs"), lib_rs).unwrap();

    // Write generated/mod.rs
    fs::write(generated_dir.join("mod.rs"), "pub mod types;\n").unwrap();

    // Write generated/types/mod.rs
    let mut types_mod_rs = String::new();
    for (name, _) in &type_files {
        types_mod_rs.push_str(&format!("pub mod {};\n", name));
    }
    fs::write(types_dir.join("mod.rs"), types_mod_rs).unwrap();

    // Verify the generated response type file
    let response_file = types_dir.join("test_method_response.rs");
    let contents = fs::read_to_string(&response_file).expect("read response file");

    // Verify field names are correctly transformed
    assert!(contents.contains("pub bip125_replaceable: bool"));
    assert!(contents.contains("pub r#type: String"));
    assert!(contents.contains("pub normal_field: Option<String>"));

    // cargo check
    let status = Command::new("cargo")
        .arg("check")
        .current_dir(tmp.path())
        .status()
        .unwrap();
    assert!(
        status.success(),
        "Response type generation with special fields failed to compile"
    );
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
