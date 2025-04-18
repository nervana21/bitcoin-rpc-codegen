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

Try out the simple example:

```bash
# Ensure you have `bitcoind` on your PATH
# The example will automatically start bitcoind in regtest mode if not running
cargo run --example auto_client
```

The example demonstrates:

- Auto-starting bitcoind in regtest mode if not running
- Auto-detecting the Bitcoin Core version
- Making basic RPC calls
- Clean shutdown of bitcoind **only if** it was spawned by the example (no effect if an instance was already running)

## How It Works

The generator automatically parses Bitcoin Core's RPC API definitions and creates:

- **Rust macros** for each RPC method.
- **Type-safe Rust structs** for request and response payloads.
- **Dynamic adapters** to seamlessly match your node's version.

## Example Usage

When writing regtest‑based tests (or any RPC code), you don’t need messy scripts or version‑sniffers. Just import our `Client` and call:

```rust
use bitcoin_rpc_codegen::Client;

fn main() -> anyhow::Result<()> {
    // No need to ask “which version am I talking to?”
    // Client::new_auto always “just knows” your node’s RPC version.
    let client = Client::new_auto(
        "http://127.0.0.1:18443",
        "rpcuser",
        "rpcpassword",
    )?;

    // Now call any RPC method directly:
    let height = client.getblockcount()?;
    println!("Regtest block height: {}", height);

    let info = client.getblockchaininfo()?;
    println!("Chain info: {:?}", info);

    Ok(())
}
```

Key benefit: in your integration tests or scripts, you never implement manual version‑detection logic—this library handles it for you every time.

## Development and Contributions

We welcome contributions! Please check out our [Contributing Guide](CONTRIBUTING.md) to get started.

## License

MIT License—see [LICENSE](LICENSE) for details.
