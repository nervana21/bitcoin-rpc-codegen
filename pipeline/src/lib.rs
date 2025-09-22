//! High-level pipeline that generates a self-contained `bitcoin-rpc-midas` crate
//! by orchestrating code generation.
//!
//! This module provides the core functionality for generating a complete Bitcoin RPC client
//! library, including transport layer, type definitions, and test node helpers.

use anyhow::{Context, Result};
use codegen::generators::{BatchBuilderGenerator, ClientTraitGenerator, ResponseTypeCodeGenerator};
use codegen::{
    generators::test_node::TestNodeGenerator, namespace_scaffolder::ModuleGenerator,
    write_generated, CodeGenerator, TransportCodeGenerator, TransportCoreGenerator,
    load_api_methods_from_file,
};
use regex::Regex;
use std::fmt::Write as _;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};
use bitcoin_rpc_types::Version;

/// Extract version from filename
///
/// This function extracts a Bitcoin Core version from the filename.
/// It requires the filename to follow the pattern "api_vXX.json" or "api_vXX_X.json" where XX is a version number and X is a single digit.
///
/// # Arguments
///
/// * `filename` - The filename string (e.g., "api_v28.json", "api_v29_1.json")
///
/// # Returns
///
/// Returns `Result<String>` containing the extracted version string (e.g., "v28", "v29.1")
///
/// # Errors
///
/// Returns an error if the filename doesn't match the expected pattern.
fn extract_version_from_filename(filename: &str) -> Result<String> {
    let re = Regex::new(r"^api_v(\d+)(?:_(\d))?\.json$")?;
    let caps = re
        .captures(filename)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Input filename '{}' does not match expected pattern 'api_vXX.json' or 'api_vXX_X.json' where XX is a version number and X is a single digit. \
                Examples: api_v28.json, api_v29.json, api_v29_1.json",
                filename
            )
        })?;

    let major = &caps[1];
    let minor = caps.get(2).map(|m| m.as_str()).unwrap_or("0");

    let version = format!("v{}.{}", major, minor);

    Ok(version)
}

/// Generates a complete Bitcoin RPC client library structure and code.
///
/// This function orchestrates the entire code generation pipeline by:
/// 1. Creating the necessary module directory structure
/// 2. Parsing and normalizing the input schema
/// 3. Generating the transport layer code for RPC communication
/// 4. Creating type definitions for RPC responses
/// 5. Generating test node helpers and client trait implementations
/// 6. Setting up the library's root structure with proper module organization
///
/// The generated code provides a type-safe, async-ready client for interacting
/// with Bitcoin Core's JSON-RPC interface, complete with all necessary
/// dependencies and documentation.
///
/// # Arguments
///
/// * `input_path` - Optional path to the input file containing JSON API spec.
///   If None, defaults to "api_v29.json" in the project root, or "api_v29_1.json" if available.
///   The filename must follow the pattern "api_vXX.json" or "api_vXX_X.json" where XX is the Bitcoin Core version and X is a single digit.
///
/// # Returns
///
/// Returns `Result<()>` indicating success or failure of the generation process
pub fn run(input_path: Option<&PathBuf>) -> Result<()> {
    let project_root = find_project_root()?;
    println!("[diagnostic] project root directory: {project_root:?}");

    let input_path = match input_path {
        Some(path) =>
            if path.is_absolute() {
                path.clone()
            } else {
                project_root.join(path)
            },
        None => {
            let version = Version::default().as_number();
            let base_path = project_root.join(format!("api_v{}.json", version));
            let alt_path = project_root.join(format!("api_v{}_1.json", version));

            // Try the _1 suffix first, then fall back to the base version
            if alt_path.exists() {
                alt_path
            } else {
                base_path
            }
        }
    };
    println!("[diagnostic] resolved input path: {input_path:?}");

    if !input_path.exists() {
        return Err(anyhow::anyhow!(
            "Input file not found: {:?}. Please provide a path to an API JSON file.\n\
             The filename must follow the pattern 'api_vXX.json' or 'api_vXX_X.json' where XX is the Bitcoin Core version and X is a single digit.\n\
             Examples: api_v28.json, api_v29.json, api_v29_1.json",
            input_path
        ));
    }

    let crate_root = project_root.join("bitcoin-rpc-midas");
    println!("[diagnostic] target crate path: {crate_root:?}");

    if crate_root.exists() {
        println!("[diagnostic] removing existing bitcoin-rpc-midas directory");
        fs::remove_dir_all(&crate_root).with_context(|| {
            format!("Failed to remove existing bitcoin-rpc-midas directory: {crate_root:?}")
        })?;
    }

    let src_dir = crate_root.join("src");
    println!("[diagnostic] creating directory: {src_dir:?}");
    fs::create_dir_all(&src_dir)
        .with_context(|| format!("Failed to create src directory: {src_dir:?}"))?;

    println!("[diagnostic] copying template files to src directory");
    copy_templates_to(&src_dir)
        .with_context(|| format!("Failed to copy template files to {src_dir:?}"))?;

    // Extract version early to pass to functions that need it
    let filename = input_path.file_name().and_then(|f| f.to_str()).ok_or_else(|| {
        anyhow::anyhow!("Could not extract filename from input path: {:?}", input_path)
    })?;

    let version_str = extract_version_from_filename(filename)?;
    let target_version = Version::from_string(&version_str)?;

    write_cargo_toml(&crate_root)
        .with_context(|| format!("Failed to write Cargo.toml in: {crate_root:?}"))?;

    let gitignore_path = crate_root.join(".gitignore");
    println!("[diagnostic] writing .gitignore at {gitignore_path:?}");
    fs::write(&gitignore_path, "/target\n/Cargo.lock\n")
        .with_context(|| format!("Failed to write .gitignore at {gitignore_path:?}"))?;

    write_readme(&crate_root, &target_version)
        .with_context(|| format!("Failed to write README.md in: {crate_root:?}"))?;

    write_contributing(&crate_root)
        .with_context(|| format!("Failed to write CONTRIBUTING.md in: {crate_root:?}"))?;

    write_license(&crate_root)
        .with_context(|| format!("Failed to write LICENSE.md in: {crate_root:?}"))?;

    println!("[diagnostic] starting code generation into: {src_dir:?}");
    generate_into(&src_dir, &input_path, &target_version)
        .with_context(|| format!("generate_into failed for src_dir {src_dir:?}"))?;

    println!("[diagnostic] contents of bitcoin-rpc-midas/src:");
    for entry in fs::read_dir(&src_dir)
        .with_context(|| format!("Failed to read bitcoin-rpc-midas/src directory: {src_dir:?}"))?
    {
        let entry = entry?;
        println!("  - {:?}", entry.path());
    }

    println!("Completed generation of `bitcoin-rpc-midas` crate at {crate_root:?}");
    Ok(())
}

/// Find the workspace root by looking for the root Cargo.toml
///
///
/// Returns `Result<PathBuf>` containing the path to the workspace root directory
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

/// Generates all the code into the specified output directory
///
/// # Arguments
///
/// * `out_dir` - The output directory to write generated code to
/// * `input_path` - Path to the input JSON file
/// * `target_version` - The Bitcoin Core version being targeted
pub fn generate_into(out_dir: &Path, input_path: &Path, target_version: &Version) -> Result<()> {
    println!(
        "[diagnostic] generate_into received out_dir: {out_dir:?}, input_path: {input_path:?}, target_version: {target_version:?}"
    );

    // 1) Prepare module directories
    let subdirs = ["transport", "types", "node", "client_trait"];
    for sub in &subdirs {
        let module_dir = out_dir.join(sub);
        println!("[diagnostic] creating module directory: {module_dir:?}");
        fs::create_dir_all(&module_dir)
            .with_context(|| format!("Failed to create module directory: {module_dir:?}"))?;

        // Skip creating mod.rs for node directory since we'll handle it separately
        if *sub != "node" {
            let mod_rs = module_dir.join("mod.rs");
            if !mod_rs.exists() {
                println!("[diagnostic] writing mod.rs for module: {sub}");
                fs::write(&mod_rs, format!("// Auto-generated `{sub}` module\n"))
                    .with_context(|| format!("Failed to write mod.rs at {mod_rs:?}"))?;
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
        .with_context(|| format!("Failed to write node/mod.rs at {node_mod_rs:?}"))?;
    }

    // Create node module that re-exports from the node crate
    println!("[diagnostic] creating node module that re-exports from node crate");

    let node_mod_content = r#"//! Node module - re-exports from the node crate
//! 
//! This module re-exports types from the node crate.

pub use node::{BitcoinNodeManager, NodeManager, NodeState};
pub use node::test_config::TestConfig;
"#;

    fs::write(&node_mod_rs, node_mod_content)
        .with_context(|| format!("Failed to write node/mod.rs at {node_mod_rs:?}"))?;

    // Create test_node directory without writing mod.rs
    let test_node_dir = out_dir.join("test_node");
    println!("[diagnostic] creating test_node directory: {test_node_dir:?}");
    fs::create_dir_all(&test_node_dir)
        .with_context(|| format!("Failed to create test_node directory: {test_node_dir:?}"))?;

    println!("[diagnostic] detecting input file type for {input_path:?}");
    let (norm, src_desc) = if input_path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("json"))
    {
        println!("[diagnostic] parsing JSON at {input_path:?}");
        (
            load_api_methods_from_file(input_path).context("Failed to parse API JSON")?,
            "structured JSON",
        )
    } else {
        return Err(anyhow::anyhow!("Only JSON files are supported. Please provide a .json file."));
    };
    println!("[diagnostic] loaded {} methods from {}", norm.len(), src_desc);

    println!("[pipeline] generating code for version: {}", target_version.as_str());
    println!("[pipeline] target_version.major(): {}", target_version.major());
    println!("[pipeline] target_version.minor(): {}", target_version.minor());

    // 3) Transport layer
    println!("[diagnostic] generating transport code");
    let tx_files = TransportCodeGenerator::new(target_version.clone()).generate(&norm);
    write_generated(out_dir.join("transport"), &tx_files)
        .context("Failed to write transport files")?;

    let core_files = TransportCoreGenerator.generate(&norm);
    write_generated(out_dir.join("transport"), &core_files)
        .context("Failed to write core transport files")?;

    let batch_files = BatchBuilderGenerator.generate(&norm);
    write_generated(out_dir.join("transport"), &batch_files)
        .context("Failed to write batch builder files")?;

    ensure_rpc_client(&out_dir.join("transport")).context("Failed to ensure rpc_client stub")?;

    let all_transport_files = tx_files
        .iter()
        .chain(core_files.iter())
        .chain(batch_files.iter())
        .cloned()
        .collect::<Vec<_>>();
    write_mod_rs(&out_dir.join("transport"), &all_transport_files)
        .context("Failed to write transport mod.rs")?;

    // After the transport layer generation:
    println!("[diagnostic] generating client trait");
    let client_trait_files = ClientTraitGenerator::new(target_version.as_str()).generate(&norm);
    write_generated(out_dir.join("client_trait"), &client_trait_files)
        .context("Failed to write client trait files")?;

    write_mod_rs(&out_dir.join("client_trait"), &client_trait_files)
        .context("Failed to write client_trait mod.rs")?;

    // 4) Types
    println!("[diagnostic] generating types code");
    let ty_files = ResponseTypeCodeGenerator::new(target_version.as_str()).generate(&norm);
    write_generated(out_dir.join("types"), &ty_files).context("Failed to write types files")?;
    write_mod_rs(&out_dir.join("types"), &ty_files).context("Failed to write types mod.rs")?;

    // 5) Test-node helpers
    println!("[diagnostic] generating test_node code");
    let tn_files = TestNodeGenerator::new(target_version.clone()).generate(&norm);

    // Write all generated files directly to test_node_dir
    write_generated(&test_node_dir, &tn_files).context("Failed to write test_node files")?;
    write_mod_rs(&test_node_dir, &tn_files).context("Failed to write test_node mod.rs")?;

    // Update lib.rs to include the client trait module
    let lib_rs = out_dir.join("lib.rs");
    println!("[diagnostic] writing root lib.rs at {lib_rs:?}");
    let mut file =
        File::create(&lib_rs).with_context(|| format!("Failed to create lib.rs at {lib_rs:?}"))?;

    let version_nodots = target_version.as_str().replace('.', "_");
    let version_capitalized = if let Some(stripped) = version_nodots.strip_prefix('v') {
        format!("V{}", stripped)
    } else {
        version_nodots.to_uppercase()
    };

    writeln!(
        file,
        "//! Generated Bitcoin RPC client library.\n\
     //!\n\
     //! This library provides a strongly-typed interface to the Bitcoin RPC API.\n\
     //! It is generated from the Bitcoin Core RPC API documentation.\n\n\
     // Core modules\n\
     pub mod config;\n\
     pub mod client_trait;\n\
     pub mod node;\n\
     pub mod test_node;\n\
     pub mod transport;\n\
     pub mod types;\n\n\
     // Re-exports for ergonomic access\n\
     pub use config::Config;\n\
     pub use client_trait::client::BitcoinClient{version_capitalized};\n\
     pub use node::BitcoinNodeManager;\n\
     pub use bitcoin::Network;\n\
     pub use node::TestConfig;\n\
     pub use test_node::client::BitcoinTestClient;\n\
     pub use bitcoin_rpc_types::*;\n\
     pub use transport::{{\n    DefaultTransport,\n    TransportError,\n    RpcClient,\n    BatchBuilder,\n}};\n"
    )?;

    ModuleGenerator::new(vec![target_version.clone()], out_dir.to_path_buf())
        .generate_all()
        .context("ModuleGenerator failed")?;

    println!("Generated modules in {out_dir:?}");

    let project_root = find_project_root()?;
    let batch_transport_src =
        std::fs::read_to_string(project_root.join("transport/src/batch_transport.rs"))
            .with_context(|| {
                format!(
                    "Failed to read batch_transport.rs at {:?}",
                    project_root.join("transport/src/batch_transport.rs")
                )
            })?;
    let dest_path = out_dir.join("transport").join("batch_transport.rs");

    std::fs::create_dir_all(dest_path.parent().unwrap())
        .with_context(|| format!("Failed to create directory for {dest_path:?}"))?;

    std::fs::write(&dest_path, batch_transport_src)
        .with_context(|| format!("Failed to write batch_transport.rs at {dest_path:?}"))?;

    Ok(())
}

/// Write the Cargo.toml file for the generated crate
///
/// # Arguments
///
/// * `root` - The root directory of the generated crate
/// * `target_version` - The Bitcoin Core version being targeted
///
/// # Returns
///
/// Returns `Result<()>` indicating success or failure of writing the Cargo.toml file
fn write_cargo_toml(root: &Path) -> Result<()> {
    println!("[diagnostic] writing Cargo.toml at {:?}", root.join("Cargo.toml"));

    let version = Version::default().crate_version();
    let toml = format!(
        r#"[package]
publish = true

name = "bitcoin-rpc-midas"
version = "{}"
edition = "2021"
authors = ["Bitcoin RPC Codegen Core Developers"]
license = "MIT OR Apache-2.0"
description = "Generated Bitcoin Core RPC client v{}."
readme = "README.md"
keywords = ["bitcoin", "rpc", "codegen", "integration-testing"]
categories = ["cryptography::cryptocurrencies", "development-tools::testing"]
repository = "https://github.com/nervana21/bitcoin-rpc-midas"
homepage = "https://github.com/nervana21/bitcoin-rpc-midas"
documentation = "https://docs.rs/bitcoin-rpc-midas"

[dependencies]
anyhow = "1.0"
async-trait = "0.1"
bitcoin = {{ version = "0.32.6", features = ["rand", "serde"] }}
bitcoin-rpc-types = {{ path = "../bitcoin-rpc-types" }}
node = {{ path = "../node" }}
reqwest = {{ version = "0.12.15", default-features = false, features = [
    "json",
    "rustls-tls",
] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
tempfile = "3.10"
thiserror = "2.0.12"
tokio = {{ version = "1.0", features = ["time", "process", "io-util"] }}
tracing = "0.1"

[workspace]
"#,
        version, version
    );

    fs::write(root.join("Cargo.toml"), toml)
        .with_context(|| format!("Failed to write bitcoin-rpc-midas Cargo.toml at {root:?}"))?;
    Ok(())
}

/// Write the README.md file for the generated crate
///
/// # Arguments
///
/// * `root` - The root directory of the generated crate
/// * `target_version` - The Bitcoin Core version being targeted
///
/// # Returns
///
/// Returns `Result<()>` indicating success or failure of writing the README.md file
fn write_readme(root: &Path, target_version: &Version) -> Result<()> {
    println!("[diagnostic] writing README.md at {:?}", root.join("README.md"));

    let version = Version::default().crate_version();
    let readme = format!(
        r#"# Bitcoin-RPC-Midas

[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Docs.rs](https://img.shields.io/docsrs/bitcoin-rpc-midas)](https://docs.rs/bitcoin-rpc-midas)
[![crates.io](https://img.shields.io/crates/v/bitcoin-rpc-midas)](https://crates.io/crates/bitcoin-rpc-midas)

Type-safe Rust client for Bitcoin Core v{} RPCs, with test node support. Generated from a version-flexible toolchain.

## Why Use This?

Compared to hand-written RPC clients, this toolchain offers:

- Reduced repetition
- Fewer versioning issues
- Increased compile-time checks
- Support for all Bitcoin p2p networks (mainnet, regtest, signet, testnet, and testnet4)
- Improved isolation from environment and port conflicts

## Architecture

The crate is organized into focused modules:

- `client_trait/`: Trait definitions for type-safe RPC method calls
- `node/`: Multi-network node management and test client support
- `test_node/`: Integration testing helpers with embedded Bitcoin nodes
- `transport/`: Async RPC transport with error handling and batching
- `types/`: Generated type definitions for all RPC responses

## Example

This asynchronous example uses [Tokio](https://tokio.rs) and enables some
optional features, so your `Cargo.toml` could look like this:

```toml
[dependencies]
bitcoin-rpc-midas = "{version}"
tokio = {{ version = "1.0", features = ["full"] }}  
```

And then the code:

```rust
use bitcoin_rpc_midas::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let client = BitcoinTestClient::new_with_network(Network::Regtest).await?;

    let blockchain_info = client.getblockchaininfo().await?;
    println!("Blockchain info:\n{{:#?}}", blockchain_info);

    Ok(())
}}
```
## Requirements

Requires a working `bitcoind` executable.

## About

This crate is generated by [bitcoin-rpc-codegen](https://github.com/nervana21/bitcoin-rpc-codegen), which systematically derives type-safe clients from Bitcoin Core's RPC specification. The generator ensures consistency, reduces duplication, and maintains alignment with upstream changes.

## Contributing

Contributors are warmly welcome, see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Bitcoin RPC Code Generator is released under the terms of the MIT license. See [LICENSE](LICENSE) for more information or see https://opensource.org/license/MIT.

## Security

This library communicates directly with `bitcoind`.
**For mainnet use,** audit the code carefully, restrict RPC access to trusted hosts, and avoid exposing RPC endpoints to untrusted networks.
"#,
        target_version.major()
    );

    fs::write(root.join("README.md"), readme)
        .with_context(|| format!("Failed to write README.md at {root:?}"))?;
    Ok(())
}

/// Write the CONTRIBUTING.md file for the generated crate
fn write_contributing(root: &Path) -> Result<()> {
    println!("[diagnostic] writing CONTRIBUTING.md at {:?}", root.join("CONTRIBUTING.md"));
    let contributing = r#"# Contributing to Bitcoin RPC Code Generator

We love your input! We want to make contributing to Bitcoin RPC Code Generator as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## We Develop with GitHub

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.

## We Use [Github Flow](https://guides.github.com/introduction/flow/index.html)

Pull requests are the best way to propose changes to the codebase. We actively welcome your pull requests:

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs or the generation process, update the documentation.
4. Ensure the test suite passes using `cargo test`.
5. Make sure your code adheres to the standard Rust style (`cargo fmt`) and passes linter checks (`cargo clippy`).
6. Issue that pull request!

## Any contributions you make will be under the MIT Software License

In short, when you submit code changes, your submissions are understood to be under the same [MIT License](http://choosealicense.com/licenses/mit/) that covers the project. Feel free to contact the maintainers if that's a concern.

## Report bugs using GitHub's [issue tracker](https://github.com/nervana21/bitcoin-rpc-codegen/issues)

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/nervana21/bitcoin-rpc-codegen/issues/new); it's that easy! **Please replace `yourusername` with the actual GitHub organization or username if different.**

## Write bug reports with detail, background, and sample code

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can.
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

## Use a Consistent Coding Style

- We follow standard Rust formatting conventions. Run `cargo fmt` to format your code.
- We use Clippy for linting. Run `cargo clippy -- -D warnings` to check for issues.

## License

By contributing, you agree that your contributions will be licensed under its MIT License.

## Development Setup

1. **Install Rust**: Make sure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).

2. **Clone the repository**:

   ```bash
   git clone https://github.com/nervana21/bitcoin-rpc-codegen.git
   cd bitcoin-rpc-codegen
   ```

3. **Build the project**:

   ```bash
   cargo build
   ```

4. **Run the tests**:

   ```bash
   cargo test
   ```

## Project Structure

The project is organized into several focused crates:

- `rpc_api/`: JSON model of RPC methods and parameters
- `codegen/`: Emits Rust modules and client implementations
- `transport/`: Async RPC transport + error handling with batching support
- `node/`: Multi-network node management and test client support
- `config/`: Node and network configuration utilities

## Guidelines for Pull Requests

1. **Keep it focused**: Each pull request should address a single issue or feature.
2. **Write tests**: Include tests for any new functionality or bug fixes.
3. **Update documentation**: Update relevant documentation as needed.
4. **Follow the code style**: Run `cargo fmt` and `cargo clippy`.
5. **Meaningful commits**: Use conventional commit messages (e.g., `feat(types): Add support for new type`).

## Questions and Discussions

If you have questions or want to discuss ideas, please open an issue on GitHub.

Thank you for contributing to the Bitcoin RPC Code Generator!
"#;

    fs::write(root.join("CONTRIBUTING.md"), contributing)
        .with_context(|| format!("Failed to write CONTRIBUTING.md at {root:?}"))?;
    Ok(())
}

/// Write the LICENSE.md file for the generated crate
fn write_license(root: &Path) -> Result<()> {
    println!("[diagnostic] writing LICENSE.md at {:?}", root.join("LICENSE.md"));
    let license = r#"MIT License

Copyright (c) 2025 Bitcoin RPC Code Generator

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."#;

    fs::write(root.join("LICENSE.md"), license)
        .with_context(|| format!("Failed to write LICENSE.md at {root:?}"))?;
    Ok(())
}

/// Ensure the RPC client stub exists in the transport directory
///
/// # Arguments
///
/// * `transport_dir` - The transport module directory
///
/// # Returns
///
/// Returns `Result<()>` indicating success or failure of ensuring the RPC client stub
fn ensure_rpc_client(transport_dir: &Path) -> Result<()> {
    let stub_path = transport_dir.join("rpc_client.rs");
    println!("[diagnostic] ensuring rpc_client stub at {stub_path:?}");
    if stub_path.exists() {
        println!("[diagnostic] rpc_client stub already exists, skipping");
        return Ok(());
    }
    let stub = r#"use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use std::fmt;
use crate::transport::{TransportTrait, TransportError, DefaultTransport, BatchBuilder};

/// Thin wrapper around a transport for making RPC calls
pub struct RpcClient {
    transport: Arc<dyn TransportTrait>,
}

impl fmt::Debug for RpcClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RpcClient")
            .field("transport", &"<dyn TransportTrait>")
            .finish()
    }
}

impl RpcClient {
    /// Wrap an existing transport (no URL+auth dance)
    pub fn from_transport(inner: Arc<dyn TransportTrait>) -> Self {
        Self { transport: inner }
    }

    /// Create a new RPC client with URL and auth
    pub fn new(url: &str, user: &str, pass: &str) -> Self {
        let transport = DefaultTransport::new(
            url.to_string(),
            Some((user.to_string(), pass.to_string())),
        );
        Self { transport: Arc::new(transport) }
    }

    /// Call a JSON-RPC method
    pub async fn call_method(&self, method: &str, params: &[Value]) -> Result<Value, TransportError> {
        self.transport.send_request(method, params).await
    }

    /// Start building a batch of RPC calls
    pub fn batch(&self) -> BatchBuilder {
        BatchBuilder::new(self.transport.clone())
    }
}"#;
    fs::write(&stub_path, stub)
        .with_context(|| format!("Failed to write rpc_client stub at {stub_path:?}"))?;
    Ok(())
}

/// Write the mod.rs file for a module directory
///
/// # Arguments
///
/// * `dir` - The module directory
/// * `files` - List of (filename, content) pairs to include in the module
///
/// # Returns
///
/// Returns `Result<()>` indicating success or failure of writing the mod.rs file
fn write_mod_rs(dir: &Path, files: &[(String, String)]) -> Result<()> {
    let mod_rs = dir.join("mod.rs");
    let mut content = String::new();

    // Special-case re-exports for transport core types, batch_transport, batch_builder & rpc_client
    if dir.ends_with("transport") {
        writeln!(
            content,
            "pub mod core;\n\
             pub use core::{{TransportTrait, TransportError, DefaultTransport, TransportExt}};\n\
             pub mod batch_transport;\n\
             pub use batch_transport::BatchTransport;\n\
             pub mod batch_builder;\n\
             pub use batch_builder::BatchBuilder;\n\
             pub mod rpc_client;\n\
             pub use rpc_client::RpcClient;\n"
        )?;
    }

    // Add module declarations and re-exports for everything else
    for (name, _) in files {
        if name.ends_with(".rs") {
            let module_name = name.trim_end_matches(".rs");
            // skip files we special-cased, plus `mod.rs` itself
            if module_name != "mod"
                && module_name != "core"
                && module_name != "batch_transport"
                && module_name != "batch_builder"
                && module_name != "rpc_client"
            {
                writeln!(content, "pub mod {module_name};")?;
                writeln!(content, "pub use {module_name}::*;")?;
            }
        }
    }

    fs::write(&mod_rs, content).with_context(|| format!("Failed to write mod.rs at {mod_rs:?}"))?;
    Ok(())
}

/// Copy template files to the destination directory
///
/// # Arguments
///
/// * `dst_dir` - The destination directory for the template files
///
/// # Returns
///
/// Returns `Result<()>` indicating success or failure of copying the template files
fn copy_templates_to(dst_dir: &Path) -> Result<()> {
    let project_root = find_project_root()?;
    let src_dir = project_root.join("templates");

    for filename in TEMPLATE_FILES {
        let src_path = src_dir.join(filename);
        let dst_path = dst_dir.join(filename);
        println!("[diagnostic] copying template: {src_path:?} -> {dst_path:?}");
        fs::copy(&src_path, &dst_path)
            .with_context(|| format!("Failed to copy template file: {filename:?}"))?;
    }

    Ok(())
}

/// Template files to be copied to the generated crate
const TEMPLATE_FILES: &[&str] = &["config.rs"];
