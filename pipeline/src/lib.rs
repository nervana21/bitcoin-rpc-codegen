//! High-level pipeline that ties together discovery / parsing,
//! schema normalisation, and code generation.

use anyhow::Result;
use codegen::{
    namespace_scaffolder::ModuleGenerator, rpc_method_discovery,
    test_node_generator::TestNodeGenerator, write_generated, CodeGenerator, TransportCodeGenerator,
    TypesCodeGenerator,
};
use parser::{DefaultHelpParser, HelpParser, MethodHelp};
use rpc_api::parse_api_json;
use schema::{DefaultSchemaNormalizer, DefaultSchemaValidator, SchemaNormalizer, SchemaValidator};

use std::fs;
use std::path::{Path, PathBuf};

/* --------------------------------------------------------------------- */
/*  Generated code lives under `src/generated`                           */
/* --------------------------------------------------------------------- */
#[cfg(feature = "generated")]
pub mod generated;

#[cfg(feature = "generated")]
pub use generated::transport::RpcClient; // optional shorthand

/* --------------------------------------------------------------------- */
/*  Public entry: run() – file-driven                                    */
/* --------------------------------------------------------------------- */

/// Generate code from either a `help.txt` dump or a structured `api.json`.
pub fn run(input_path: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    println!("ENTERED RUN FUNCTION");
    println!("Running file-driven pipeline");
    println!(
        "Step 1: Creating subdirectories and mod.rs files in {:?}",
        out_dir
    );
    let subdirs = ["transport", "types", "test_node"];
    for subdir in &subdirs {
        let dir_path = out_dir.join(subdir);
        fs::create_dir_all(&dir_path)?;
        let mod_rs = dir_path.join("mod.rs");
        if !mod_rs.exists() {
            fs::write(
                &mod_rs,
                format!("// Auto-generated mod.rs for {}\n", subdir),
            )?;
        }
    }

    /* ----------------------------------------------------------------- */
    /*  Parse → normalise → validate                                     */
    /* ----------------------------------------------------------------- */
    let (norm, src_description) = if input_path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    {
        let json = fs::read_to_string(input_path)?;
        (parse_api_json(&json)?, "structured JSON")
    } else {
        let help = fs::read_to_string(input_path)?;
        let raw = DefaultHelpParser.parse(&help)?;
        (DefaultSchemaNormalizer.normalize(&raw)?, "help dump")
    };
    DefaultSchemaValidator.validate(&norm)?;
    println!("Loaded {} methods from {}", norm.len(), src_description);

    /* ----------------------------------------------------------------- */
    /*  Generate transport                                               */
    /* ----------------------------------------------------------------- */
    let transport_dir = out_dir.join("transport");
    fs::create_dir_all(&transport_dir)?;
    let transport_files = TransportCodeGenerator.generate(&norm);
    write_generated(&transport_dir, &transport_files)?;

    // Ensure `rpc_client.rs` exists until the generator writes it itself.
    ensure_rpc_client(&transport_dir)?;

    write_mod_rs(&transport_dir, &transport_files)?;

    /* ----------------------------------------------------------------- */
    /*  Generate types (latest)                                          */
    /* ----------------------------------------------------------------- */
    let types_dir = out_dir.join("types");
    fs::create_dir_all(&types_dir)?;
    let latest_types_dir = types_dir.join("latest_types");
    fs::create_dir_all(&latest_types_dir)?;

    let type_files = TypesCodeGenerator.generate(&norm);
    write_generated(&latest_types_dir, &type_files)?;
    write_mod_rs(&latest_types_dir, &type_files)?;

    fs::write(
        types_dir.join("mod.rs"),
        "// Auto-generated types module declarations.\n\
         pub mod latest_types;\n\
         pub use latest_types::*;\n",
    )?;

    /* ----------------------------------------------------------------- */
    /*  Generate test-node helper                                        */
    /* ----------------------------------------------------------------- */
    let test_node_dir = out_dir.join("test_node");
    fs::create_dir_all(&test_node_dir)?;
    let test_node_files = TestNodeGenerator.generate(&norm);
    write_generated(&test_node_dir, &test_node_files)?;
    write_mod_rs(&test_node_dir, &test_node_files)?;

    /* ----------------------------------------------------------------- */
    /*  Root mod.rs + scaffolding                                        */
    /* ----------------------------------------------------------------- */
    write_mod_rs(
        out_dir,
        &[
            ("transport".into(), String::new()),
            ("types".into(), String::new()),
            ("test_node".into(), String::new()),
        ],
    )?;

    ModuleGenerator::new(vec!["latest".into()], out_dir.clone()).generate_all()?;

    println!(
        "✅ Wrote {} transport, {} types, and {} test-node files to {:?}",
        transport_files.len(),
        type_files.len(),
        test_node_files.len(),
        out_dir
    );
    Ok(())
}

/* --------------------------------------------------------------------- */
/*  Helper: ensure minimal rpc_client.rs                                 */
/* --------------------------------------------------------------------- */

fn ensure_rpc_client(transport_dir: &Path) -> Result<()> {
    let rpc_client_path = transport_dir.join("rpc_client.rs");

    if rpc_client_path.exists() {
        return Ok(());
    }

    let content = r#"use anyhow::Result;
use serde_json::Value;

/// The main RPC client struct. Use this to make calls to Bitcoin Core.
#[derive(Debug, Clone)]
pub struct RpcClient {
    transport: transport::Transport,
}

impl RpcClient {
    /// Create a new RPC client with the given URL and authentication.
    pub fn new_with_auth(url: impl Into<String>, username: &str, password: &str) -> Self {
        Self {
            transport: transport::Transport::new_with_auth(url, username, password),
        }
    }

    /// Make a raw RPC call to the Bitcoin Core node.
    pub async fn call_method(&self, method: &str, params: &[Value]) -> Result<Value> {
        self.transport
            .send_request(method, params)
            .await
            .map_err(anyhow::Error::from)
    }
}
"#;

    fs::write(&rpc_client_path, content)?;
    Ok(())
}

/* --------------------------------------------------------------------- */
/*  Optional: discovery variant                                          */
/* --------------------------------------------------------------------- */

/// Same as [`run`] but discovers RPCs from a live `bitcoin-cli`.
pub fn run_discovery(bitcoind_path: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    let discovered = rpc_method_discovery::discover_methods(bitcoind_path)
        .map_err(|e| anyhow::anyhow!("Discovery failed: {e}"))?;

    let helps: Vec<MethodHelp> = discovered
        .into_iter()
        .map(|m| MethodHelp {
            name: m.name,
            raw: m.description,
        })
        .collect();

    let norm = DefaultSchemaNormalizer.normalize(&helps)?;
    DefaultSchemaValidator.validate(&norm)?;

    fs::create_dir_all(out_dir)?;
    let transport_dir = out_dir.join("transport");
    let types_dir = out_dir.join("types");
    let test_node_dir = out_dir.join("test_node");
    fs::create_dir_all(&transport_dir)?;
    fs::create_dir_all(&types_dir)?;
    fs::create_dir_all(&test_node_dir)?;

    write_generated(&transport_dir, &TransportCodeGenerator.generate(&norm))?;
    write_generated(&types_dir, &TypesCodeGenerator.generate(&norm))?;
    write_generated(&test_node_dir, &TestNodeGenerator.generate(&norm))?;

    Ok(())
}

/* --------------------------------------------------------------------- */
/*  Utility: simple mod.rs writer                                        */
/* --------------------------------------------------------------------- */

fn write_mod_rs(dir: &Path, files: &[(String, String)]) -> anyhow::Result<()> {
    let mut body = String::new();

    // Add RpcClient if this is transport/
    if dir.file_name().map_or(false, |n| n == "transport") {
        body.push_str("pub mod rpc_client;\n");
        body.push_str("pub use rpc_client::RpcClient;\n\n");
    }

    for (path, _) in files {
        let stem = Path::new(path).file_stem().unwrap().to_str().unwrap();
        if stem != "rpc_client" {
            body.push_str(&format!("pub mod {};\n", stem));
        }
    }

    // If this is the generated root, expose RpcClient as a convenience
    if dir.file_name().map_or(false, |n| n == "generated") {
        body.push_str("\npub use transport::RpcClient;\n");
    }

    fs::write(dir.join("mod.rs"), body)?;
    Ok(())
}
