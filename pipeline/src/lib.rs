//! High-level pipeline that generates a self-contained `bitcoin-rpc-midas` crate
//! by tying together discovery/parsing, schema normalization, and code generation.

use anyhow::{Context, Result};
use codegen::generators::TypesCodeGenerator;
use codegen::{
    namespace_scaffolder::ModuleGenerator, test_node_generator::TestNodeGenerator, write_generated,
    CodeGenerator, TransportCodeGenerator, TransportCoreGenerator,
};
use parser::{DefaultHelpParser, HelpParser};
use rpc_api::parse_api_json;
use schema::{DefaultSchemaNormalizer, DefaultSchemaValidator, SchemaNormalizer, SchemaValidator};
use std::fmt::Write as _;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

/* --------------------------------------------------------------------- */
/*  Constants                                                            */
/* --------------------------------------------------------------------- */
/// The version of the generated bitcoin-rpc-midas crate
pub const CRATE_VERSION: &str = "0.1.1";

/* --------------------------------------------------------------------- */
/*  Public entry: run() – build `bitcoin-rpc-midas` crate                            */
/* --------------------------------------------------------------------- */
/// Generates a fully self-contained `bitcoin-rpc-midas` crate under the workspace root.
/// Always emits into `<workspace-root>/bitcoin-rpc-midas` and prints verbose diagnostics.
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

    let crate_root = project_root.join("bitcoin-rpc-midas");
    println!("[diagnostic] target crate path: {:?}", crate_root);

    // Remove existing bitcoin-rpc-midas directory if it exists
    if crate_root.exists() {
        println!("[diagnostic] removing existing bitcoin-rpc-midas directory");
        fs::remove_dir_all(&crate_root).with_context(|| {
            format!(
                "Failed to remove existing bitcoin-rpc-midas directory: {:?}",
                crate_root
            )
        })?;
    }

    // Prepare crate structure: Cargo.toml + src/
    let src_dir = crate_root.join("src");
    println!("[diagnostic] creating directory: {:?}", src_dir);
    fs::create_dir_all(&src_dir)
        .with_context(|| format!("Failed to create src directory: {:?}", src_dir))?;

    // Copy template files to src directory
    println!("[diagnostic] copying template files to src directory");
    copy_templates_to(&src_dir)
        .with_context(|| format!("Failed to copy template files to {:?}", src_dir))?;

    // Write Cargo.toml
    write_cargo_toml(&crate_root)
        .with_context(|| format!("Failed to write Cargo.toml in: {:?}", crate_root))?;

    // Write .gitignore with minimal exclusions
    let gitignore_path = crate_root.join(".gitignore");
    println!("[diagnostic] writing .gitignore at {:?}", gitignore_path);
    fs::write(&gitignore_path, "/target\n/Cargo.lock\n")
        .with_context(|| format!("Failed to write .gitignore at {:?}", gitignore_path))?;

    // Write README.md
    write_readme(&crate_root)
        .with_context(|| format!("Failed to write README.md in: {:?}", crate_root))?;

    println!("[diagnostic] starting code generation into: {:?}", src_dir);
    generate_into(&src_dir, &input_path)
        .with_context(|| format!("generate_into failed for src_dir {:?}", src_dir))?;

    // List resulting crate contents for verification
    println!("[diagnostic] contents of bitcoin-rpc-midas/src:");
    for entry in fs::read_dir(&src_dir).with_context(|| {
        format!(
            "Failed to read bitcoin-rpc-midas/src directory: {:?}",
            src_dir
        )
    })? {
        let entry = entry?;
        println!("  - {:?}", entry.path());
    }

    println!(
        "✅ Completed generation of `bitcoin-rpc-midas` crate at {:?}",
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
    let subdirs = ["transport", "types", "node"];
    for sub in &subdirs {
        let module_dir = out_dir.join(sub);
        println!("[diagnostic] creating module directory: {:?}", module_dir);
        fs::create_dir_all(&module_dir)
            .with_context(|| format!("Failed to create module directory: {:?}", module_dir))?;

        // Skip creating mod.rs for node directory since we'll handle it separately
        if *sub != "node" {
            let mod_rs = module_dir.join("mod.rs");
            if !mod_rs.exists() {
                println!("[diagnostic] writing mod.rs for module: {}", sub);
                fs::write(&mod_rs, format!("// Auto-generated `{}` module\n", sub))
                    .with_context(|| format!("Failed to write mod.rs at {:?}", mod_rs))?;
            }
        }
    }

    // Copy template files
    println!("[diagnostic] copying template files");
    copy_templates_to(out_dir).with_context(|| "Failed to copy template files")?;

    // After copying template files, ensure node/mod.rs exists
    let node_dir = out_dir.join("node");
    let node_mod_rs = node_dir.join("mod.rs");

    if !node_mod_rs.exists() {
        println!("[diagnostic] writing node/mod.rs manually");
        fs::write(
            &node_mod_rs,
            r#"
// Auto-generated `node` module
pub mod bitcoin_node_manager;
pub mod test_config;

pub use bitcoin_node_manager::BitcoinNodeManager;
pub use test_config::TestConfig;
"#,
        )
        .with_context(|| format!("Failed to write node/mod.rs at {:?}", node_mod_rs))?;
    }

    // Create node implementation files
    println!("[diagnostic] creating node implementation files");

    // Create bitcoin_node_manager.rs
    let bitcoin_node_manager_rs = node_dir.join("bitcoin_node_manager.rs");
    println!("[diagnostic] writing bitcoin_node_manager.rs");
    fs::write(
        &bitcoin_node_manager_rs,
        r#"use anyhow::Result;
use async_trait::async_trait;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tempfile::TempDir;
use tokio::io::AsyncBufReadExt;
use tokio::process::{Child, Command};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, error, info, warn};
use std::process::Stdio;

use super::test_config::TestConfig;

/// Represents the state of a Bitcoin node
#[derive(Debug, Clone)]
pub struct NodeState {
    pub is_running: bool,
    pub version: String,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            is_running: false,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Trait defining the interface for a Bitcoin node manager
#[async_trait]
pub trait NodeManager: Send + Sync + std::any::Any + std::fmt::Debug {
    async fn start(&self) -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
    async fn get_state(&self) -> Result<NodeState>;
    /// Return the RPC port this manager was configured with
    fn rpc_port(&self) -> u16;
}

/// Implementation of the Bitcoin node manager
#[derive(Debug)]
pub struct BitcoinNodeManager {
    state: Arc<RwLock<NodeState>>,
    child: Arc<Mutex<Option<Child>>>,
    pub(crate) rpc_port: u16,
    rpc_username: String,
    rpc_password: String,
    _datadir: Option<TempDir>,
}

impl BitcoinNodeManager {
    pub fn new() -> Result<Self> {
        Self::new_with_config(&TestConfig::default())
    }

    pub fn new_with_config(config: &TestConfig) -> Result<Self> {
        let datadir = TempDir::new()?;

        // Handle automatic port selection:
        // When rpc_port is 0, we need to find an available port dynamically.
        let rpc_port = if config.rpc_port == 0 {
            // Bind to port 0 to let the OS assign an available port
            let listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
            listener.local_addr()?.port()
        } else {
            config.rpc_port
        };

        Ok(Self {
            state: Arc::new(RwLock::new(NodeState::default())),
            child: Arc::new(Mutex::new(None)),
            rpc_port,
            rpc_username: config.rpc_username.clone(),
            rpc_password: config.rpc_password.clone(),
            _datadir: Some(datadir),
        })
    }

    pub(crate) async fn start_internal(&self) -> Result<()> {
        let mut state = self.state.write().await;
        if state.is_running {
            info!("[DEBUG] Node is already running, skipping start");
            return Ok(());
        }

        info!(
            "[DEBUG] Starting Bitcoin node with datadir: {:?}",
            self._datadir.as_ref().unwrap().path()
        );
        let datadir = self._datadir.as_ref().unwrap().path();
        let mut cmd = Command::new("bitcoind");
        cmd.args([
            "-regtest",
            "-listen=0",
            &format!("-datadir={}", datadir.display()),
            &format!("-rpcport={}", self.rpc_port),
            &format!("-rpcbind=127.0.0.1:{}", self.rpc_port),
            "-rpcallowip=127.0.0.1",
            "-fallbackfee=0.0002",
            "-server=1",
            "-prune=1",
            &format!("-rpcuser={}", self.rpc_username),
            &format!("-rpcpassword={}", self.rpc_password),
        ]);

        // Capture both stdout and stderr for better error reporting
        cmd.stderr(Stdio::piped());
        cmd.stdout(Stdio::piped());

        info!("[DEBUG] Spawning bitcoind process");
        let mut child = cmd.spawn()?;
        info!("[DEBUG] bitcoind process spawned successfully");

        // Read stderr in a separate task
        let stderr = child.stderr.take().unwrap();
        let stderr_reader = tokio::io::BufReader::new(stderr);
        tokio::spawn(async move {
            let mut lines = stderr_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                error!("[DEBUG] bitcoind stderr: {}", line);
            }
        });

        // Read stdout in a separate task
        let stdout = child.stdout.take().unwrap();
        let stdout_reader = tokio::io::BufReader::new(stdout);
        tokio::spawn(async move {
            let mut lines = stdout_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                info!("[DEBUG] bitcoind stdout: {}", line);
            }
        });

        // Store the child process
        info!("[DEBUG] Storing child process");
        let mut child_guard = self.child.lock().await;
        *child_guard = Some(child);
        info!("[DEBUG] Child process stored successfully");

        // Wait for node to be ready
        info!("[DEBUG] Waiting for node to be ready");
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut attempts = 0;
        while Instant::now() < deadline {
            if let Some(child) = child_guard.as_mut() {
                if let Ok(Some(status)) = child.try_wait() {
                    let error = format!("Bitcoin node exited early with status: {}", status);
                    error!("[DEBUG] {}", error);
                    anyhow::bail!(error);
                }
            }

            // Try to connect to RPC
            info!(
                "[DEBUG] Attempt {}: Trying to connect to RPC at port {}",
                attempts + 1,
                self.rpc_port
            );
            let client = reqwest::Client::new();
            match client
                .post(format!("http://127.0.0.1:{}/", self.rpc_port))
                .basic_auth(&self.rpc_username, Some(&self.rpc_password))
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "getnetworkinfo",
                    "params": [],
                    "id": 1
                }))
                .send()
                .await
            {
                Ok(response) => {
                    info!("[DEBUG] RPC response status: {}", response.status());
                    if response.status().is_success() {
                        state.is_running = true;
                        info!(
                            "[DEBUG] Bitcoin node started successfully on port {}",
                            self.rpc_port
                        );
                        return Ok(());
                    } else {
                        warn!(
                            "[DEBUG] RPC request failed with status {} (attempt {})",
                            response.status(),
                            attempts
                        );
                    }
                }
                Err(e) => {
                    debug!(
                        "[DEBUG] Failed to connect to RPC (attempt {}): {}",
                        attempts, e
                    );
                }
            }

            attempts += 1;
            info!("[DEBUG] Waiting 200ms before next attempt");
            tokio::time::sleep(Duration::from_millis(200)).await;
        }

        let error = format!(
            "[DEBUG] Timed out waiting for Bitcoin node to start on port {} after {} attempts",
            self.rpc_port, attempts
        );
        error!("{}", error);
        anyhow::bail!(error);
    }

    pub(crate) async fn stop_internal(&mut self) -> Result<()> {
        let mut state = self.state.write().await;
        if !state.is_running {
            return Ok(());
        }

        let child = self.child.lock().await.take();
        if let Some(mut child) = child {
            std::mem::drop(child.kill());
            std::mem::drop(child.wait());
        }

        state.is_running = false;
        Ok(())
    }

    async fn get_state(&self) -> Result<NodeState> {
        Ok(self.state.read().await.clone())
    }

    fn rpc_port(&self) -> u16 {
        self.rpc_port
    }
}

#[async_trait]
impl NodeManager for BitcoinNodeManager {
    async fn start(&self) -> Result<()> {
        self.start_internal().await
    }

    async fn stop(&mut self) -> Result<()> {
        self.stop_internal().await
    }

    async fn get_state(&self) -> Result<NodeState> {
        self.get_state().await
    }

    fn rpc_port(&self) -> u16 {
        self.rpc_port()
    }
}

impl Drop for BitcoinNodeManager {
    fn drop(&mut self) {
        if let Some(mut child) = self
            .child
            .try_lock()
            .ok()
            .and_then(|mut guard| guard.take())
        {
            std::mem::drop(child.kill());
            std::mem::drop(child.wait());
        }
    }
}

impl Default for BitcoinNodeManager {
    fn default() -> Self {
        Self::new_with_config(&TestConfig::default())
            .expect("Failed to create default BitcoinNodeManager")
    }
}"#,
    )
    .with_context(|| {
        format!(
            "Failed to write bitcoin_node_manager.rs at {:?}",
            bitcoin_node_manager_rs
        )
    })?;

    // Create test_config.rs
    let test_config_rs = node_dir.join("test_config.rs");
    println!("[diagnostic] writing test_config.rs");
    fs::write(
        &test_config_rs,
        r#"use std::env;

/// TestConfig represents the configuration needed to run a Bitcoin node in a test environment.
/// This struct is the single source of truth for test‑node settings: RPC port, username, and password.
/// Defaults are:
/// - `rpc_port = 0` (auto‑select a free port)
/// - `rpc_username = "rpcuser"`
/// - `rpc_password = "rpcpassword"`
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// The port number for RPC communication with the Bitcoin node.
    /// A value of 0 indicates that an available port should be automatically selected.
    pub rpc_port: u16,
    /// The username for RPC authentication.
    /// Can be customized to match your `bitcoin.conf` `rpcuser` setting.
    pub rpc_username: String,
    /// The password for RPC authentication.
    /// Can be customized to match your `bitcoin.conf` `rpcpassword` setting.
    pub rpc_password: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            rpc_port: 0,
            rpc_username: "rpcuser".to_string(),
            rpc_password: "rpcpassword".to_string(),
        }
    }
}

impl TestConfig {
    /// Create a `TestConfig`, overriding defaults with environment variables:
    /// - `RPC_PORT`: overrides `rpc_port`
    /// - `RPC_USER`: overrides `rpc_username`
    /// - `RPC_PASS`: overrides `rpc_password`
    pub fn from_env() -> Self {
        let mut cfg = Self::default();
        if let Ok(port_str) = env::var("RPC_PORT") {
            if let Ok(port) = port_str.parse() {
                cfg.rpc_port = port;
            }
        }
        if let Ok(user) = env::var("RPC_USER") {
            cfg.rpc_username = user;
        }
        if let Ok(pass) = env::var("RPC_PASS") {
            cfg.rpc_password = pass;
        }
        cfg
    }
}"#,
    )
    .with_context(|| format!("Failed to write test_config.rs at {:?}", test_config_rs))?;

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
    let mut file = File::create(&lib_rs)
        .with_context(|| format!("Failed to create lib.rs at {:?}", lib_rs))?;

    let version_nodots = CRATE_VERSION.replace('.', "");

    writeln!(
        file,
        "//! Generated Bitcoin RPC client library.\n\
         //!\n\
         //! This library provides a strongly-typed interface to the Bitcoin RPC API.\n\
         //! It is generated from the Bitcoin Core RPC API documentation.\n\n\
         pub mod config;\n\
         pub mod node;\n\
         pub mod transport;\n\
         pub mod types;\n\
         pub mod test_node;\n\n\
         pub use config::Config;\n\
         pub use node::BitcoinNodeManager;\n\
         pub use transport::{{DefaultTransport, TransportError}};\n\
         pub use crate::test_node::test_node::BitcoinTestClient;"
    )?;

    ModuleGenerator::new(vec!["latest".into()], out_dir.to_path_buf())
        .generate_all()
        .context("ModuleGenerator failed")?;

    println!("✅ Generated modules in {:?}", out_dir);
    Ok(())
}

/* --------------------------------------------------------------------- */
/*  Utility: write minimal Cargo.toml for `bitcoin-rpc-midas`                      */
/* --------------------------------------------------------------------- */
fn write_cargo_toml(root: &Path) -> Result<()> {
    println!(
        "[diagnostic] writing Cargo.toml at {:?}",
        root.join("Cargo.toml")
    );
    let toml = r#"[package]
publish = true

name = "bitcoin-rpc-midas"
version = "{}"
edition = "2021"
authors = ["Bitcoin RPC Codegen Core Developers"]
license = "MIT OR Apache-2.0"
description = "Generated Bitcoin Core RPC clients with async and strong types."
readme = "README.md"
keywords = ["bitcoin", "rpc", "codegen", "integration-testing"]
categories = ["cryptography::cryptocurrencies", "development-tools::testing"]
repository = "https://github.com/nervana21/bitcoin-rpc-midas"
homepage = "https://github.com/nervana21/bitcoin-rpc-midas"
documentation = "https://docs.rs/bitcoin-rpc-midas"

[dependencies]
anyhow = "1.0"
async-trait = "0.1"
bitcoin = { version = "0.32.6", features = ["rand", "serde"] }
reqwest = { version = "0.12.15", default-features = false, features = [
    "json",
    "rustls-tls",
] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tempfile = "3.10"
thiserror = "2.0.12"
tokio = { version = "1.0", features = ["time", "process", "io-util"] }
tracing = "0.1"

[workspace]
"#,
        CRATE_VERSION
    );

    fs::write(root.join("Cargo.toml"), toml)
        .with_context(|| format!("Failed to write bitcoin-rpc-midas Cargo.toml at {:?}", root))?;
    Ok(())
}

/* --------------------------------------------------------------------- */
/*  Utility: write README.md for `bitcoin-rpc-midas`                      */
/* --------------------------------------------------------------------- */
fn write_readme(root: &Path) -> Result<()> {
    println!(
        "[diagnostic] writing README.md at {:?}",
        root.join("README.md")
    );
    let readme = r#"# Bitcoin-RPC-Midas

Bitcoin-RPC-Midas provides a type-safe, async-ready client for interacting with Bitcoin Core's JSON-RPC interface. It is generated automatically from the bitcoin-cli help output and reflects the actual RPC methods supported by the node.

This crate is designed for developers building tools, tests, or applications that need a reliable interface to bitcoind across versions.

## Features

    ✅ Typed method signatures for each RPC

    🚀 Async support via tokio and reqwest

    🔍 Automatically derived from live node metadata

    🧪 Ideal for integration testing or development tools

    🧱 Supports multiple Bitcoin Core versions

## Example

```rust
use anyhow::{anyhow, Result};
use bitcoin_rpc_midas::BitcoinTestClient;
use bitcoin::Amount;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = BitcoinTestClient::new().await?;
    let info = client.getblockchaininfo().await?;
    println!("{:#?}", info);
    Ok(())
}
```

## Installation

To install:

```bash
cargo add bitcoin-rpc-midas
```

## About

This crate is generated by [bitcoin-rpc-codegen](https://github.com/nervana21/bitcoin-rpc-codegen) and maintained independently as a standalone, publishable client for the Bitcoin RPC interface.

## License

Licensed under either of:

    MIT license

Contributions welcome.
"#;

    fs::write(root.join("README.md"), readme)
        .with_context(|| format!("Failed to write README.md at {:?}", root))?;
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
pub struct RpcClient { 
    transport: Box<dyn Transport> 
}

impl RpcClient {
    pub fn new_with_auth(url: impl Into<String>, user: &str, pass: &str) -> Self {
        Self { 
            transport: Box::new(crate::transport::DefaultTransport::new(
                url,
                Some((user.to_string(), pass.to_string()))
            ))
        }
    }
    
    pub async fn call_method(&self, method: &str, params: &[Value]) -> Result<Value, TransportError> {
        self.transport.send_request(method, params).await
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
        writeln!(
            content,
            "pub mod core;
             pub use core::{{Transport, TransportError, DefaultTransport, TransportExt}};\n"
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

/* --------------------------------------------------------------------- */
/*  Template file copying                                                */
/* --------------------------------------------------------------------- */
const TEMPLATE_FILES: &[&str] = &["config.rs"];

fn copy_templates_to(dst_dir: &Path) -> Result<()> {
    let src_dir = PathBuf::from("templates");

    for filename in TEMPLATE_FILES {
        let src_path = src_dir.join(filename);
        let dst_path = dst_dir.join(filename);
        println!(
            "[diagnostic] copying template: {:?} -> {:?}",
            src_path, dst_path
        );
        fs::copy(&src_path, &dst_path)
            .with_context(|| format!("Failed to copy template file: {:?}", filename))?;
    }

    Ok(())
}
