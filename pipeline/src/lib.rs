//! High-level pipeline that generates a self-contained `midas` crate
//! by tying together discovery/parsing, schema normalization, and code generation.

use anyhow::{Context, Result};
use codegen::{
    namespace_scaffolder::ModuleGenerator, test_node_generator::TestNodeGenerator, write_generated,
    CodeGenerator, TransportCodeGenerator, TransportCoreGenerator, TypesCodeGenerator,
};
use parser::{DefaultHelpParser, HelpParser};
use rpc_api::parse_api_json;
use schema::{DefaultSchemaNormalizer, DefaultSchemaValidator, SchemaNormalizer, SchemaValidator};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::{env, fs};

/* --------------------------------------------------------------------- */
/*  Public entry: run() – build `midas` crate                            */
/* --------------------------------------------------------------------- */
/// Generates a fully self-contained `midas` crate under the workspace root.
/// Always emits into `<workspace-root>/midas` and prints verbose diagnostics.
pub fn run(input_path: Option<&PathBuf>) -> Result<()> {
    // Find project root by looking for Cargo.toml
    let project_root = find_project_root()?;
    println!("[diagnostic] project root directory: {:?}", project_root);

    // Use default api.json in project root if no input path provided
    let input_path = match input_path {
        Some(path) => {
            if path.is_absolute() {
                path.clone()
            } else {
                project_root.join(path)
            }
        }
        None => project_root.join("api.json"),
    };
    println!("[diagnostic] resolved input path: {:?}", input_path);

    // Verify input file exists before proceeding
    if !input_path.exists() {
        return Err(anyhow::anyhow!(
            "Input file not found: {:?}. Please either:\n\
             1. Place an api.json file in the project root, or\n\
             2. Specify the path to your API JSON file as an argument",
            input_path
        ));
    }

    let crate_root = project_root.join("midas");
    println!("[diagnostic] target crate path: {:?}", crate_root);

    // Remove existing midas directory if it exists
    if crate_root.exists() {
        println!("[diagnostic] removing existing midas directory");
        fs::remove_dir_all(&crate_root).with_context(|| {
            format!(
                "Failed to remove existing midas directory: {:?}",
                crate_root
            )
        })?;
    }

    // Prepare crate structure: Cargo.toml + src/
    let src_dir = crate_root.join("src");
    println!("[diagnostic] creating directory: {:?}", src_dir);
    fs::create_dir_all(&src_dir)
        .with_context(|| format!("Failed to create src directory: {:?}", src_dir))?;

    // Write Cargo.toml
    write_cargo_toml(&crate_root)
        .with_context(|| format!("Failed to write Cargo.toml in: {:?}", crate_root))?;

    println!("[diagnostic] starting code generation into: {:?}", src_dir);
    generate_into(&src_dir, &input_path)
        .with_context(|| format!("generate_into failed for src_dir {:?}", src_dir))?;

    // List resulting crate contents for verification
    println!("[diagnostic] contents of midas/src:");
    for entry in fs::read_dir(&src_dir)
        .with_context(|| format!("Failed to read midas/src directory: {:?}", src_dir))?
    {
        let entry = entry?;
        println!("  - {:?}", entry.path());
    }

    println!(
        "✅ Completed generation of `midas` crate at {:?}",
        crate_root
    );
    Ok(())
}

/// Find the workspace root by looking for the root Cargo.toml
fn find_project_root() -> Result<PathBuf> {
    let mut current = env::current_dir()?;
    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            // Read the Cargo.toml to check if it's the workspace root
            let contents = fs::read_to_string(&cargo_toml)?;
            if contents.contains("[workspace]") {
                return Ok(current);
            }
        }
        if !current.pop() {
            return Err(anyhow::anyhow!(
                "Could not find workspace root (no workspace Cargo.toml found)"
            ));
        }
    }
}

/* --------------------------------------------------------------------- */
/*  Shared logic: generate code modules into an arbitrary directory      */
/* --------------------------------------------------------------------- */
fn generate_into(out_dir: &Path, input_path: &Path) -> Result<()> {
    println!(
        "[diagnostic] generate_into received out_dir: {:?}, input_path: {:?}",
        out_dir, input_path
    );

    // 1) Prepare module directories
    let subdirs = ["transport", "types"];
    for sub in &subdirs {
        let module_dir = out_dir.join(sub);
        println!("[diagnostic] creating module directory: {:?}", module_dir);
        fs::create_dir_all(&module_dir)
            .with_context(|| format!("Failed to create module directory: {:?}", module_dir))?;

        let mod_rs = module_dir.join("mod.rs");
        if !mod_rs.exists() {
            println!("[diagnostic] writing mod.rs for module: {}", sub);
            fs::write(&mod_rs, format!("// Auto-generated `{}` module\n", sub))
                .with_context(|| format!("Failed to write mod.rs at {:?}", mod_rs))?;
        }
    }

    // Create test_node directory without writing mod.rs
    let test_node_dir = out_dir.join("test_node");
    println!(
        "[diagnostic] creating test_node directory: {:?}",
        test_node_dir
    );
    fs::create_dir_all(&test_node_dir)
        .with_context(|| format!("Failed to create test_node directory: {:?}", test_node_dir))?;

    // 2) Parse & normalize schema
    println!(
        "[diagnostic] detecting input file type for {:?}",
        input_path
    );
    let (norm, src_desc) = if input_path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("json"))
    {
        println!("[diagnostic] parsing JSON at {:?}", input_path);
        let json = fs::read_to_string(input_path)
            .with_context(|| format!("Failed to read JSON file: {:?}", input_path))?;
        (
            parse_api_json(&json).context("Failed to parse API JSON")?,
            "structured JSON",
        )
    } else {
        println!("[diagnostic] parsing help text at {:?}", input_path);
        let help = fs::read_to_string(input_path)
            .with_context(|| format!("Failed to read help dump file: {:?}", input_path))?;
        let raw = DefaultHelpParser
            .parse(&help)
            .context("HelpParser failed to parse help text")?;
        (
            DefaultSchemaNormalizer
                .normalize(&raw)
                .context("Schema normalization failed")?,
            "help dump",
        )
    };
    DefaultSchemaValidator
        .validate(&norm)
        .context("Schema validation failed")?;
    println!(
        "[diagnostic] loaded {} methods from {}",
        norm.len(),
        src_desc
    );

    // 3) Transport layer
    println!("[diagnostic] generating transport code");
    let tx_files = TransportCodeGenerator.generate(&norm);
    write_generated(out_dir.join("transport"), &tx_files)
        .context("Failed to write transport files")?;

    // Generate core transport types
    println!("[diagnostic] generating core transport types");
    let core_files = TransportCoreGenerator.generate(&norm);
    write_generated(out_dir.join("transport"), &core_files)
        .context("Failed to write core transport files")?;

    ensure_rpc_client(&out_dir.join("transport")).context("Failed to ensure rpc_client stub")?;
    write_mod_rs(&out_dir.join("transport"), &tx_files)
        .context("Failed to write transport mod.rs")?;

    // 4) Types
    println!("[diagnostic] generating types code");
    let ty_files = TypesCodeGenerator.generate(&norm);
    write_generated(out_dir.join("types"), &ty_files).context("Failed to write types files")?;
    write_mod_rs(&out_dir.join("types"), &ty_files).context("Failed to write types mod.rs")?;

    // 5) Test-node helpers
    println!("[diagnostic] generating test_node code");
    let tn_files = TestNodeGenerator.generate(&norm);
    write_generated(out_dir.join("test_node"), &tn_files)
        .context("Failed to write test_node files")?;
    write_mod_rs(&out_dir.join("test_node"), &tn_files)
        .context("Failed to write test_node mod.rs")?;

    // 6) Root `lib.rs`
    let lib_rs = out_dir.join("lib.rs");
    println!("[diagnostic] writing root lib.rs at {:?}", lib_rs);
    let lib_content = "//! `midas` — generated Bitcoin RPC test-node toolkit\n\n".to_string()
        + "pub mod transport;\n"
        + "pub mod types;\n"
        + "pub mod test_node;\n\n"
        + "pub use node::{NodeManager, TestConfig, BitcoinNodeManager};\n\n"
        + "pub use transport::{Transport, DefaultTransport, TransportError};\n"
        + "pub use test_node::test_node::{TestNode, BitcoinTestClient};\n"
        + "pub use types::*;\n";
    fs::write(&lib_rs, lib_content)
        .with_context(|| format!("Failed to write lib.rs at {:?}", lib_rs))?;

    ModuleGenerator::new(vec!["latest".into()], out_dir.to_path_buf())
        .generate_all()
        .context("ModuleGenerator failed")?;

    println!("✅ Generated modules in {:?}", out_dir);
    Ok(())
}

/* --------------------------------------------------------------------- */
/*  Utility: write minimal Cargo.toml for `midas`                      */
/* --------------------------------------------------------------------- */
fn write_cargo_toml(root: &Path) -> Result<()> {
    println!(
        "[diagnostic] writing Cargo.toml at {:?}",
        root.join("Cargo.toml")
    );
    let toml = r#"[package]
name = "midas"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
bitcoin = { version = "0.32.6", features = ["rand", "serde"] }
node = { path = "../node" }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0.12"
tokio = { version = "1.0", features = ["time"] }
transport = { path = "../transport" }
"#;
    fs::write(root.join("Cargo.toml"), toml)
        .with_context(|| format!("Failed to write midas Cargo.toml at {:?}", root))?;
    Ok(())
}

/* --------------------------------------------------------------------- */
/*  RPC-client stub & mod.rs writer (unchanged)                          */
/* --------------------------------------------------------------------- */
fn ensure_rpc_client(transport_dir: &Path) -> Result<()> {
    let stub_path = transport_dir.join("rpc_client.rs");
    println!("[diagnostic] ensuring rpc_client stub at {:?}", stub_path);
    if stub_path.exists() {
        println!("[diagnostic] rpc_client stub already exists, skipping");
        return Ok(());
    }
    let stub = r#"use anyhow::Result;
use serde_json::Value;

#[derive(Debug, Clone)]
/// RPC client stub
pub struct RpcClient { transport: crate::transport::Transport }

impl RpcClient {
    pub fn new_with_auth(url: impl Into<String>, user: &str, pass: &str) -> Self {
        Self { transport: crate::transport::Transport::new_with_auth(url, user, pass) }
    }
    pub async fn call_method(&self, method: &str, params: &[Value]) -> Result<Value> {
        Ok(self.transport.send_request(method, params).await?)
    }
}
"#;
    fs::write(&stub_path, stub)
        .with_context(|| format!("Failed to write rpc_client stub at {:?}", stub_path))?;
    Ok(())
}

fn write_mod_rs(dir: &Path, files: &[(String, String)]) -> Result<()> {
    let mod_rs = dir.join("mod.rs");
    let mut content = String::new();

    // Re-export core transport types
    if dir.ends_with("transport") {
        writeln!(content, "pub mod core;").unwrap();
        writeln!(
            content,
            "pub use core::{{Transport, TransportError, DefaultTransport}};\n"
        )
        .unwrap();
    }

    // Add module declarations
    for (name, _) in files {
        if name.ends_with(".rs") {
            let module_name = name.trim_end_matches(".rs");
            if module_name != "mod" {
                // Skip the mod.rs file itself
                writeln!(content, "pub mod {};", module_name).unwrap();
            }
        }
    }

    fs::write(&mod_rs, content.as_bytes())
        .with_context(|| format!("Failed to write mod.rs at {:?}", mod_rs))?;
    Ok(())
}
