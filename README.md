# Bitcoin RPC Code Generator

A **Universal Adapter** for Bitcoin RPC communication, seamlessly translating your Rust applications' requests into the native language of your Bitcoin nodes.

No more compatibility headaches—simply tell your application, "Connect to my Bitcoin node," and the adapter intelligently handles the rest.

## What Does "Universal Adapter" Mean?

- **Automatic Version Detection:** It queries your Bitcoin node, asking, "Which language (version) do you speak?"
- **Dynamic Client Generation:** It responds with, "Great, I speak that language too!" and automatically generates type-safe Rust RPC client methods tailored specifically for your node's Bitcoin Core version.
- **Type-safe and Secure:** The generated Rust methods and response types ensure compile-time validation, drastically reducing runtime errors.

## Features

- **Bitcoin Core v28 Support:** Currently supports Bitcoin Core v28 RPC API, with plans to expand to other versions.
- **Strongly Typed API:** Eliminates guesswork and manual parsing, improving maintainability and security.
- **Robust Error Handling:** Uses idiomatic Rust error handling (`anyhow`) to clearly communicate issues during RPC calls.
- **Seamless Integration:** Generated code is easily integrated into your Rust projects, fitting naturally into your build process via Cargo's `OUT_DIR`.
- **Test Node Support:** Includes a `BitcoinTestClient` for testing and development, with support for:
  - Wallet creation and management
  - Address generation (including bech32m)
  - Block generation and mining
  - Blockchain state queries

## Project Structure

The project is organized into several key components:

- **codegen/**: Core code generation logic
  - RPC method macro generation
  - Response type generation
  - Namespace scaffolding
  - Test node generation
- **config/**: Configuration management
- **parser/**: Bitcoin Core RPC API parsing
- **pipeline/**: High-level code generation pipeline
- **rpc-api/**: RPC API definitions and types
- **schema/**: Schema normalization and validation
- **transport/**: JSON-RPC transport layer

## Quick Start

Install the generator with Cargo:

```bash
cargo install bitcoin-rpc-codegen
```

Or clone the repository:

```bash
git clone https://github.com/nervana21/bitcoin-rpc-codegen.git
cd bitcoin-rpc-codegen
cargo build --release
```

## Example Usage

Here's a practical example showing how to use the `BitcoinTestClient` for testing:

```rust
use bitcoin_rpc_codegen::BitcoinTestClient;

async fn test_bitcoin_node() -> anyhow::Result<()> {
    // Initialize test node with sensible defaults
    let client = BitcoinTestClient::new().await?;

    // Get initial blockchain state
    let info = client.getblockchaininfo().await?;
    println!("Initial blockchain state:\n{:#?}\n", info);

    // Create and fund a test wallet
    let wallet_name = "test_wallet";
    let _wallet = client
        .createwallet(
            wallet_name.to_string(),
            false,          // disable_private_keys
            false,          // blank
            "".to_string(), // passphrase
            false,          // avoid_reuse
            true,           // descriptors
            false,          // load_on_startup
            false,          // external_signer
        )
        .await?;

    // Generate a new bech32m address
    let address = client.getnewaddress("".to_string(), "bech32m".to_string()).await?.0;

    Ok(())
}
```

## Development and Contributions

We welcome contributions! Please check out our [Contributing Guide](CONTRIBUTING.md) to get started.

## License

MIT License—see [LICENSE](LICENSE) for details.
