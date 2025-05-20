use codegen::{
    write_generated, BasicCodeGenerator, CodeGenerator, TransportCodeGenerator, TypesCodeGenerator,
};

use rpc_api::ApiMethod;
use std::{fs, process::Command};
use tempfile::TempDir;

#[test]
fn generated_code_compiles() {
    // 1. Set up a temp Cargo project
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

    // 2. Generate two dummy methods (foo & bar) and emit them into src/
    let methods = vec![
        ApiMethod {
            name: "foo".into(),
            description: "".into(),
            arguments: vec![],
            results: vec![],
        },
        ApiMethod {
            name: "bar".into(),
            description: "".into(),
            arguments: vec![],
            results: vec![],
        },
    ];

    let files = BasicCodeGenerator.generate(&methods);
    write_generated(&src, &files).expect("write_generated failed");

    // 3. Write lib.rs that imports both modules
    let mut lib_rs = String::new();
    for m in &methods {
        lib_rs.push_str(&format!("mod {0}; pub use {0}::{0};\n", m.name));
    }
    fs::write(src.join("lib.rs"), lib_rs).expect("write lib.rs");

    // 4. Run `cargo check` in that temp project
    let status = Command::new("cargo")
        .arg("check")
        .current_dir(project)
        .status()
        .expect("failed to run cargo check");

    assert!(status.success(), "Generated code failed to compile");
}

#[test]
fn json_rpc_generated_code_compiles() {
    // 1. Set up a temp Cargo project
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

    // Create generated/types module
    let generated_dir = src.join("generated");
    fs::create_dir(&generated_dir).expect("create generated dir");
    let types_dir = generated_dir.join("types");
    fs::create_dir(&types_dir).expect("create types dir");

    // Generate two methods via JsonRpcCodeGenerator
    let methods = vec![
        ApiMethod {
            name: "foo".into(),
            description: "".into(),
            arguments: vec![],
            results: vec![],
        },
        ApiMethod {
            name: "bar".into(),
            description: "".into(),
            arguments: vec![],
            results: vec![],
        },
    ];

    // Generate types first
    let type_gen = TypesCodeGenerator;
    let type_files = type_gen.generate(&methods);
    write_generated(&types_dir, &type_files).expect("write_generated types");

    // Generate transport code
    let gen = TransportCodeGenerator;
    let files = gen.generate(&methods);
    write_generated(&src, &files).expect("write_generated");

    // Write lib.rs importing both
    let mut lib_rs = String::new();
    lib_rs.push_str("pub mod generated;\n");
    for m in &methods {
        lib_rs.push_str(&format!("mod {0}; pub use {0}::{0};\n", m.name));
    }
    fs::write(src.join("lib.rs"), lib_rs).unwrap();

    // Write generated/mod.rs
    fs::write(generated_dir.join("mod.rs"), "pub mod types;\n").unwrap();

    // Write generated/types/mod.rs
    let mut types_mod_rs = String::new();
    for (name, _) in &type_files {
        types_mod_rs.push_str(&format!("pub mod {};\n", name));
    }
    fs::write(types_dir.join("mod.rs"), types_mod_rs).unwrap();

    // cargo check
    let status = Command::new("cargo")
        .arg("check")
        .current_dir(project)
        .status()
        .unwrap();
    assert!(
        status.success(),
        "JSON-RPC generated code failed to compile"
    );
}

#[test]
fn response_type_generation_with_special_fields() {
    // 1. Set up a temp Cargo project
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
bitcoin = "0.31"

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

    // Create generated/types module
    let generated_dir = src.join("generated");
    fs::create_dir(&generated_dir).expect("create generated dir");
    let types_dir = generated_dir.join("types");
    fs::create_dir(&types_dir).expect("create types dir");

    // Generate a method with special field names
    let methods = vec![ApiMethod {
        name: "test_method".into(),
        description: "Test method with special fields".into(),
        arguments: vec![],
        results: vec![
            rpc_api::ApiResult {
                key_name: "bip125-replaceable".into(),
                type_: "boolean".into(),
                description: "Test field with hyphen".into(),
                optional: false,
                inner: vec![],
            },
            rpc_api::ApiResult {
                key_name: "type".into(),
                type_: "string".into(),
                description: "Test field with keyword".into(),
                optional: false,
                inner: vec![],
            },
            rpc_api::ApiResult {
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
        .current_dir(project)
        .status()
        .unwrap();
    assert!(
        status.success(),
        "Response type generation with special fields failed to compile"
    );
}

// TODO: Add tests for generate_types_from_schema
// #[tokio::test]
// async fn test_generate_types_from_schema() {
//     // 1. Read API schema
//     let schema_json = fs::read_to_string("api.json").unwrap();
//     let methods = parse_api_json(&schema_json).unwrap();

//     // 2. Generate types
//     let generator = TransportCodeGenerator;
//     let generated_files = generator.generate(&methods);

//     // 3. Write to temp directory
//     let tmp = TempDir::new().unwrap();
//     write_generated(tmp.path(), &generated_files).unwrap();

//     // 4. Verify files were generated
//     for (name, _) in &generated_files {
//         let file_path = tmp.path().join(format!("{}.rs", name));
//         assert!(
//             file_path.exists(),
//             "Expected {} to exist",
//             file_path.display()
//         );
//     }

//     // 5. Optionally validate types
//     if let Ok(node_manager) = BitcoinNodeManager::new() {
//         let validator = TypeValidator::new(Box::new(node_manager), "rpcuser", "rpcpass")
//             .await
//             .unwrap();
//         validator
//             .validate_method_response(&methods[0])
//             .await
//             .unwrap();
//     }
// }
