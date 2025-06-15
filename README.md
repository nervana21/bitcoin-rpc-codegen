[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/bitcoin-rpc-midas)](https://crates.io/crates/bitcoin-rpc-midas)

# Bitcoin RPC Code Generator

Instantly generate [bitcoin-rpc-midas](https://github.com/nervana21/bitcoin-rpc-midas), a type-safe Bitcoin Core client that makes Bitcoin testing and development 10× easier.

## Why Use This?

Compared to hand-rolled RPC clients, this toolchain provides:

- Less repetition
- Fewer versioning bugs
- More compile-time guarantees
- Easier local testing with embedded regtest
- Better isolation from flaky environments and port conflicts

Each improvement is aimed at making Bitcoin Core RPCs easier to integrate, test, and depend on in a modern Rust codebase. The result is a type-safe client that just works — aligned with upstream, resilient to changes, and ready for production use.

## Semantic Compression: The Guiding Principle

This project implements a **semantic compression** architecture: instead of hand-coding interfaces to a changing protocol, it models the RPC surface as structured data and generates type-safe Rust clients from that schema. This minimizes duplication while preserving fidelity to upstream behavior.

This approach delivers:

- Minimal maintenance overhead
- Comprehensive version compatibility
- A canonical source of truth (`api.json`) that governs all generated code

The architecture is designed to **systematically reduce complexity**. Code duplication, version incompatibilities, and behavioral inconsistencies are treated as defects in the generator implementation rather than inherent limitations of the system.

Read more: [`docs/semantic-compression.md`](docs/semantic-compression.md)

## Architecture

See [`docs/architecture.mmd`](docs/architecture.mmd) for a full system diagram.

### Key Components

- `rpc_api/`: JSON model of RPC methods and parameters
- `parser/`: Parses `help` or `api.json` into structured form
- `schema/`: Normalizes and validates parsed data
- `codegen/`: Emits Rust modules and client implementations
- `transport/`: Minimal async RPC transport + error handling
- `node/`: Regtest node management and test client support
- `pipeline/`: Orchestrates parsing → schema → generation

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
bitcoin-rpc-midas = "0.1.1"
anyhow = "1.0"
bitcoin = "0.32.0"
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Minimal Example

```rust
use anyhow::Result;
use bitcoin_rpc_midas::*; // Re-exports BitcoinTestClient and other helpers

#[tokio::main]
async fn main() -> Result<()> {
    let client = BitcoinTestClient::new().await?;
    let wallet_info = client.getwalletinfo().await?;
    println!("Wallet state:\n{:#?}", wallet_info);
    Ok(())
}
```

---

## Contributing

Contributions are welcome — including forks, patches, extensions, or overlays.
See [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

## License

MIT — see [LICENSE](LICENSE)

## Security

This library communicates directly with `bitcoind`.
**For mainnet use,** audit the code carefully, restrict RPC access to trusted hosts, and avoid exposing RPC endpoints to untrusted networks.
