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
- Support for all Bitcoin p2p networks (mainnet, regtest, signet, testnet, and testnet4)
- Improved isolation from environment and port conflicts

These features are intended to make Bitcoin Core RPCs easier to integrate, test, and maintain in Rust projects. The result is a client that remains aligned with upstream changes and is suitable for production use.

## Semantic Compression

This project uses a **semantic compression** approach: rather than hand-coding interfaces for a changing protocol, it models the RPC surface as structured data and generates type-safe Rust clients from that schema. This reduces duplication while maintaining fidelity to upstream behavior.

A key advantage is that **all generated code is derived from a single source of truth**: [`api_v29.json`](api_v29.json). By using this unified schema, consistency is ensured not only across this codebase, but also across any project or tool that adopts the same description. This approach makes it easy to reason about the full RPC surface in one place and simplifies updates as upstream changes.

The architecture aims to reduce complexity and treat code duplication, version mismatches, and inconsistencies as issues to be addressed in the generator.

Read more: [`docs/semantic-compression.md`](docs/semantic-compression.md)

## Architecture

See [`docs/architecture.mmd`](docs/architecture.mmd) for a full system diagram.

### Project Structure

The project is organized into several focused crates:

- [`codegen/`](./codegen/): Emits Rust modules and client implementations
- [`pipeline/`](./pipeline/): Coordinates the end-to-end code generation workflow
- [`bitcoin-rpc-midas/`](https://github.com/nervana21/bitcoin-rpc-midas): The final generated Rust client library (output of the codegen pipeline).  
  _Note: This directory is generated and published as a separate repository._

- [`transport/`](./transport/): Async RPC transport + error handling with batching support
- [`node/`](./node/): Multi-network node management and test client support
- [`config/`](./config/): Node and network configuration utilities

All components are modular and reusable. You can build overlays, language targets, or devtools by composing with this core.

> **Note:** This repository houses the code generator. The generated client library is published separately as [`bitcoin-rpc-midas`](https://crates.io/crates/bitcoin-rpc-midas).

## Example

This asynchronous example uses [Tokio](https://tokio.rs) and enables some
optional features, so your `Cargo.toml` could look like this:

```toml
[dependencies]
bitcoin-rpc-midas = "29.1.0"
tokio = { version = "1.0", features = ["full"] }
```

And then the code:

```rust
use bitcoin_rpc_midas::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BitcoinTestClient::new_with_network(Network::Regtest).await?;

    let blockchain_info = client.getblockchaininfo().await?;
    println!("Blockchain info:\n{blockchain_info:#?}");

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
