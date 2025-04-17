# Bitcoin RPC Code Generator

A **Universal Adapter** for Bitcoin RPC communication, seamlessly translating your Rust applications' requests into the native language of your Bitcoin nodes.

No more compatibility headaches—simply tell your application, "Connect to my Bitcoin node," and the adapter intelligently handles the rest.

## What Does "Universal Adapter" Mean?

- **Automatic Version Detection:** It queries your Bitcoin node, asking, "Which language (version) do you speak?"
- **Dynamic Client Generation:** It responds with, "Great, I speak that language too!" and automatically generates type-safe Rust RPC client methods tailored specifically for your node's Bitcoin Core version.
- **Type-safe and Secure:** The generated Rust methods and response types ensure compile-time validation, drastically reducing runtime errors.

## Features

- **Version-Agnostic:** Supports Bitcoin Core RPC versions v17 to v28, adapting automatically based on your node's capabilities.
- **Strongly Typed API:** Eliminates guesswork and manual parsing, improving maintainability and security.
- **Robust Error Handling:** Uses idiomatic Rust error handling (`anyhow`) to clearly communicate issues during RPC calls.
- **Seamless Integration:** Generated code is easily integrated into your Rust projects, fitting naturally into your build process via Cargo's `OUT_DIR`.

## Quick Start

Install the generator with Cargo:

```bash
cargo install bitcoin-rpc-codegen
```

Or clone the repository:

```bash
git clone https://github.com/yourusername/bitcoin-rpc-codegen.git
cd bitcoin-rpc-codegen
cargo build --release
```

Try out the basic example:

```bash
bitcoind -regtest -daemon
cargo run --example basic_client
```

## How It Works

The generator automatically parses Bitcoin Core's RPC API definitions and creates:

- **Rust macros** for each RPC method.
- **Type-safe Rust structs** for request and response payloads.
- **Dynamic adapters** to seamlessly match your node's version.

## Example Usage

Here's how you connect your Rust application to your Bitcoin node:

```rust
use bitcoin_rpc_codegen::Client;

fn main() -> anyhow::Result<()> {
    // Connect to your Bitcoin node. The client auto-detects version.
    let client = Client::new_auto("http://127.0.0.1:18443", "rpcuser", "rpcpassword")?;

    // Make an RPC call without worrying about compatibility.
    let blockchain_info = client.getblockchaininfo()?;

    println!("Blockchain info: {:?}", blockchain_info);

    Ok(())
}
```

Your application never needs to manage compatibility again—the universal adapter ensures communication is always fluent and effective.

## Development and Contributions

We welcome contributions! Please check out our [Contributing Guide](CONTRIBUTING.md) to get started.

## License

MIT License—see [LICENSE](LICENSE) for details.
