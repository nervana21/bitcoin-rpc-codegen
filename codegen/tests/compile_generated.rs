use codegen::{write_generated, BasicCodeGenerator, CodeGenerator, TransportCodeGenerator};
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
    let gen = TransportCodeGenerator;
    let files = gen.generate(&methods);
    write_generated(&src, &files).expect("write_generated");

    // Write lib.rs importing both
    let mut lib_rs = String::new();
    for m in &methods {
        lib_rs.push_str(&format!("mod {0}; pub use {0}::{0};\n", m.name));
    }
    fs::write(src.join("lib.rs"), lib_rs).unwrap();

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
