[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Docs.rs](https://img.shields.io/docsrs/bitcoin-rpc-midas)](https://docs.rs/bitcoin-rpc-midas)
[![crates.io](https://img.shields.io/crates/v/bitcoin-rpc-midas)](https://crates.io/crates/bitcoin-rpc-midas)

# Bitcoin RPC Codegen

Generate [bitcoin-rpc-midas](https://github.com/nervana21/bitcoin-rpc-midas), a Bitcoin Core client designed to simplify Bitcoin testing and development.

## Why Use This?

Compared to hand-written RPC clients, this toolchain offers:

- Reduced repetition
- Fewer versioning issues
- Increased compile-time checks
- Simplified local testing with embedded regtest
- Improved isolation from environment and port conflicts

These features are intended to make Bitcoin Core RPCs easier to integrate, test, and maintain in Rust projects. The intended result is a client that remains aligned with upstream changes and is suitable for production use.

## Semantic Compression

This project uses a **semantic compression** approach: rather than hand-coding interfaces for a changing protocol, it models the RPC surface as structured data and generates type-safe Rust clients from that schema. This reduces duplication while maintaining fidelity to upstream behavior.

A key advantage is that **all generated code is derived from a single source of truth**: [`api.json`](api.json). By using this unified schema, consistency is ensured not only across this codebase, but also across any project or tool that adopts the same description. This approach makes it easy to reason about the full RPC surface in one place and simplifies updates as upstream changes.

The architecture aims to reduce complexity and treat code duplication, version mismatches, and inconsistencies as issues to be addressed in the generator.

Read more: [`docs/semantic-compression.md`](docs/semantic-compression.md)

## Architecture

See [`docs/architecture.mmd`](docs/architecture.mmd) for a full system diagram.

### Project Structure

The project is organized into several focused crates:

- [`rpc_api/`](./rpc-api/): JSON model of RPC methods and parameters
- [`codegen/`](./codegen/): Emits Rust modules and client implementations
- [`pipeline/`](./pipeline/): Coordinates the end-to-end code generation workflow
- [`bitcoin-rpc-midas/`](https://github.com/nervana21/bitcoin-rpc-midas): The final generated Rust client library (output of the codegen pipeline).  
  _Note: This directory is generated and published as a separate repository._

- [`transport/`](./transport/): Async RPC transport + error handling
- [`node/`](./node/): Regtest node management and test client support
- [`config/`](./config/): Node and test configuration utilities

All components are modular and reusable. You can build overlays, language targets, or devtools by composing with this core.

## Quick Start

> **Note:** This repository provides the code generator. The generated client library is published separately as [`bitcoin-rpc-midas`](https://crates.io/crates/bitcoin-rpc-midas).

### Install the Client

```bash
cargo add bitcoin-rpc-midas
```

Or add it manually:

```toml
[dependencies]
bitcoin-rpc-midas = "0.1.2"
tokio = { version = "1.0", features = ["full"] }
```

### Minimal Example

```rust
use bitcoin_rpc_midas::*; // Re-exports BitcoinTestClient and other helpers

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BitcoinTestClient::new().await?;
    let blockchain_info = client.getblockchaininfo().await?;
    println!("Blockchain info:\n{:#?}", blockchain_info);
    Ok(())
}
```

## Contributing

Contributors are warmly welcome, see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Bitcoin RPC Code Generator is released under the terms of the MIT license. See [LICENSE](LICENSE) for more information or see https://opensource.org/license/MIT.

## Security

This library communicates directly with `bitcoind`.
**For mainnet use,** audit the code carefully, restrict RPC access to trusted hosts, and avoid exposing RPC endpoints to untrusted networks.
