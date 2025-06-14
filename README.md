[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/bitcoin-rpc-midas)](https://crates.io/crates/bitcoin-rpc-midas)

# Bitcoin RPC Code Generator

Instantly generate [bitcoin-rpc-midas](https://github.com/nervana21/bitcoin-rpc-midas), a type-safe Bitcoin Core client that makes Bitcoin testing and development 10× easier.

## Why Use This?

Bitcoin Core's RPC interface is powerful — but:

- ❌ Repetitive to wrap by hand
- ❌ Error-prone across versions
- ❌ Fragile at runtime without type safety
- ❌ Difficult to test reliably without full node setup logic
- ❌ Prone to flaky bugs from port conflicts and manual wiring

This toolchain solves these problems by providing:

- ✅ **Automatic code generation** — fully generated, production-grade client
- ✅ **Version compatibility** — matches your node's exact RPC interface
- ✅ **Type safety** — compile-time guarantees for all methods
- ✅ **Built-in testing support** — built-in regtest node management
- ✅ **Reliable execution** — no port conflicts or manual wiring

## Semantic Compression: The Guiding Principle

This project implements a **semantic compression** architecture: rather than manually implementing thousands of lines of code to interface with an evolving protocol, we transform the interface into a concise, structured schema that drives a code generator to produce type-safe Rust clients.

This approach delivers:

- Minimal maintenance overhead
- Comprehensive version compatibility
- A canonical source of truth (`api.json`) that governs all generated code

The architecture is designed to **systematically reduce complexity**. Code duplication, version incompatibilities, and behavioral inconsistencies are treated as defects in the generator implementation rather than inherent limitations of the system.

Read more: [`docs/semantic-compression.md`](docs/semantic-compression.md)

## 🪙 Focused on Bitcoin Core & Rust

This project targets Bitcoin Core's live RPC interface and encodes it directly into idiomatic, async Rust clients. This tight coupling means strict fidelity to upstream behavior, with zero runtime guessing.

## Architecture

See [`docs/architecture.mmd`](docs/architecture.mmd) for a full system diagram.

### Key Components

- `rpc_api/` — JSON model of RPC methods and parameters
- `parser/` — Parses `help` or `api.json` into structured form
- `schema/` — Normalizes and validates parsed data
- `codegen/` — Emits Rust modules and client implementations
- `transport/` — Minimal async RPC transport + error handling
- `node/` — Regtest node management and test client support
- `pipeline/` — Orchestrates parsing → schema → generation

All components are modular and reusable. You can build overlays, language targets, or devtools by composing with this core.

## Quick Start

> **⚠️ Note:** This repository is the code generator. The runtime client is published separately.

### Installing the Client Library

Install the generated RPC client as `bitcoin-rpc-midas`:

```bash
cargo add bitcoin-rpc-midas
```

Or manually:

```toml
[dependencies]
bitcoin-rpc-midas = "0.1.1"
anyhow = "1.0"
bitcoin = "0.32.0"
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Example Usage

```rust
use anyhow::Result;
use bitcoin-rpc-midas::BitcoinTestClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BitcoinTestClient::new().await?;
    let wallet_info = client.getwalletinfo().await?;
    println!("Wallet state:\n{:#?}\n", wallet_info);
    Ok(())
}
```

## Contributing

This project is designed for collaboration and extension. Forks, PRs, patches, overlays, etc are all welcome.

To get started, check out [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE](LICENSE)

## Security Note

This and all related software can communicate directly with _bitcoind_. Mainnet use requires caution: always audit the code, restrict RPC access to trusted interfaces, and avoid exposing your node to the public internet.
