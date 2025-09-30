[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Docs.rs](https://img.shields.io/docsrs/bitcoin-rpc-midas)](https://docs.rs/bitcoin-rpc-midas)
[![crates.io](https://img.shields.io/crates/v/bitcoin-rpc-midas)](https://crates.io/crates/bitcoin-rpc-midas)

# bitcoin-rpc-codegen

Generate [bitcoin-rpc-midas](https://github.com/nervana21/bitcoin-rpc-midas), a Bitcoin Core client that supercharges Bitcoin testing and development.

## Why Use This?

Compared to hand-written RPC clients, [midas](https://github.com/nervana21/bitcoin-rpc-midas) offers:

- Reduced repetition
- Fewer versioning issues
- Increased compile-time checks
- Improved isolation from environment glitches

Together, these features translate Bitcoin Core into idiomatic Rust.

## Semantic Compression

This project applies semantic compression. It models the RPC surface as a structured schema and generates type-safe Rust clients directly from that schema. This unites Rust consumers of the RPC interface layer. All generated code is derived from a **single source of truth**: [api_v29_1.json](api_v29_1.json). By adopting this schema, consistency is guaranteed not only within this codebase, but also across any Rust project that consumes Core [RPCs](https://github.com/nervana21/bitcoin/tree/2025-07-schema-generation).

Deep Dive: [docs/semantic-compression.md](docs/semantic-compression.md)

## Architecture

See [docs/architecture.mmd](docs/architecture.mmd) for a full system diagram.

### Project Structure

The project is organized into several focused crates:

- [types](https://crates.io/crates/bitcoin-rpc-types): Type definitions for Bitcoin Core's JSON-RPC interface
- [conversions](https://crates.io/crates/bitcoin-rpc-conversions): For converting Bitcoin RPC types to Rust types
- [codegen](./codegen/): Emits idiomatic Rust modules and clients
- [pipeline](./pipeline/): Coordinates end-to-end code generation
- [transport](./transport/): RPC transport and error handling
- [node](./node/): Multi-node management and test client support
- [config](./config/): Configuration utilities

All components are modular and reusable.

## Example

This asynchronous example uses [Tokio](https://tokio.rs) and enables some
optional features, so your `Cargo.toml` could look like this:

```toml
[dependencies]
bitcoin-rpc-midas = "29.1.2"
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

MIT OR Apache-2.0

## Related Projects

Part of the bitcoin-rpc crate ecosystem, providing type-safe Rust primitives for testing and development at the Bitcoin Core JSON-RPC interface.

## Security

This library communicates directly with `bitcoind`. For mainnet use, restrict RPC access to trusted hosts and avoid exposing RPC endpoints to untrusted networks.


