# Bitcoin RPC Code Generator

A code generator that produces **bitcoin-rpc-midas** - a type-safe Rust library for Bitcoin RPC communication. Like King Midas turning everything to gold, bitcoin-rpc-midas transforms Bitcoin Core's C++ RPC interface into pure Rust gold, ensuring your application's Bitcoin node communication is guaranteed to work.

## The Problem

Bitcoin Core's RPC interface evolves across versions, and manually maintaining RPC client code is error-prone and time-consuming. One wrong parameter type or missing field can lead to runtime failures that are hard to catch during development.

## The Solution

This generator creates **bitcoin-rpc-midas** - a **guaranteed-compatible** Rust interface to your Bitcoin node:

- **The Midas Touch:** Transforms Bitcoin Core's C++ RPC interface into pure Rust gold
- **Compile-Time Guarantees:** bitcoin-rpc-midas is type-safe and matches your Bitcoin node's exact RPC interface. If it compiles, it works.
- **Version-Aware:** Automatically detects your Bitcoin node version and generates the correct interface.
- **Zero Runtime Surprises:** All RPC methods and response types are generated from Bitcoin Core's source, ensuring perfect compatibility.

## Key Benefits

- **Golden Type Safety:** Like Midas's touch, every C++ RPC call is transformed into type-safe Rust gold
- **Eliminates RPC Compatibility Issues:** No more runtime errors from mismatched parameter types or response structures
- **Developer Experience:** Full IDE support with autocomplete and type hints
- **Maintenance-Free:** Automatically adapts to your Bitcoin node version
- **Production-Ready:** Generated code is optimized and follows Rust best practices

## Features

- **Bitcoin Core Version Support:** Pre-generated support for Bitcoin Core v28. For other versions, run the generator against your node's RPC interface.
- **Type-Safe API:** All RPC methods and responses are strongly typed
- **Robust Error Handling:** Uses idiomatic Rust error handling (`anyhow`)
- **Test Node Support:** Includes a `BitcoinTestClient` for development and testing

## Project Structure

- **codegen/**: Core code generation logic
- **config/**: Configuration management
- **parser/**: Bitcoin Core RPC API parsing
- **pipeline/**: High-level code generation pipeline
- **rpc-api/**: RPC API definitions and types
- **schema/**: Schema normalization and validation (feeds codegen)
- **transport/**: JSON-RPC transport layer

## Quick Start

### Using the Pre-generated Library

Add the library to your project:

```bash
cargo add bitcoin-rpc-midas
```

Or manually add to your `Cargo.toml`:

```toml
[dependencies]
midas = { package = "bitcoin-rpc-midas", version = "0.1.1" }

anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
bitcoin = "0.32.0"
```

## Example Usage

```rust
use anyhow::Result;
use midas::BitcoinTestClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BitcoinTestClient::new().await?;
    let wallet_info = client.getwalletinfo().await?;
    println!("Wallet state:\n{:#?}\n", wallet_info);
    Ok(())
}
```

## Development and Contributions

We welcome contributions! Please check out our [Contributing Guide](CONTRIBUTING.md) to get started.

## License

MIT License—see [LICENSE](LICENSE) for details.

## Security Disclaimer

Generated code communicates directly with your Bitcoin node. Always audit the generated code before using it on mainnet.
