# Bitcoin RPC Code Generator

A tool for generating type-safe Rust RPC client code for Bitcoin Core's JSON-RPC API. This project parses Bitcoin Core's API documentation (from a JSON file) and automatically produces both client methods and Rust types for interacting with the node.

## Features

- **Type-safe Code Generation:** Automatically produces Rust client methods and strongly typed response structures based on Bitcoin Core's RPC API documentation.
- **Support for Multiple Versions:** Generate clients for Bitcoin Core versions from v17 through v28. (Currently, the default configuration targets v28.)
- **Automatic Parameter Validation:** Generated methods include basic input validation and enforce type safety at compile time.
- **Error Handling:** Uses `anyhow` for comprehensive error reporting, ensuring that RPC errors and parsing issues are clear.
- **Integration with Bitcoin Libraries:** Leverages industry-standard crates such as `bitcoin` and `bitcoincore-rpc` for Bitcoin types and connecting to a Bitcoin Core node.
- **Build Process Integration:** Generated code is produced into Cargo's build directory via `OUT_DIR` at compile time, making integration with your Rust projects seamless.

## Supported Versions

- Bitcoin Core versions: v17 to v28  
  (The code generation supports all versions, though v28 is the default target.)

## Installation

You can install the code generator using Cargo:

```bash
cargo install bitcoin-rpc-codegen
```

Alternatively, clone the repository for local development:

```bash
git clone https://github.com/yourusername/bitcoin-rpc-codegen.git
cd bitcoin-rpc-codegen
cargo build --release
```

## How It Works

The library operates through several key components:

1. **Parser Module (`src/parser/mod.rs`):** Reads and parses Bitcoin Core's RPC API definition from the JSON file, converting it into Rust data structures.

2. **Generator Module (`src/generator/mod.rs`):** Takes the parsed API definitions and generates:

   - Rust client methods as macros for each RPC call
   - Type definitions for request parameters and response structures
   - Organized by Bitcoin Core category (blockchain, wallet, network, etc.)

3. **Codegen Process (`src/generator/codegen.rs`):** Orchestrates the code generation process, writing the generated files to Cargo's build directory at compile time.

4. **NodeClient (`src/node_client/mod.rs`):** Provides a simple client implementation for connecting to a Bitcoin Core node.

## Usage

The code generation happens at compile time via Cargo's build script system. The process reads the API definition from `resources/api.json` and generates client methods and type definitions for all supported Bitcoin Core versions.

### Using the Generated Code

In your Rust project, you can include the generated code using the `include!` macro:

```rust
// Include the generated client methods for a specific version and category
include!(concat!(env!("OUT_DIR"), "/client/src/v28/blockchain.rs"));

// Include the generated type definitions
include!(concat!(env!("OUT_DIR"), "/types/src/v28/blockchain.rs"));
```

### Example: Connecting to a Bitcoin Node

```rust
use anyhow::Result;
use bitcoin_rpc_codegen::client::v28::blockchain::impl_client_v28__getblockchaininfo;
use bitcoin_rpc_codegen::types::v28::blockchain::GetblockchaininfoResponse;
use serde_json::Value;

// Define a client type that will be extended with generated methods
pub struct Client {
    rpc: bitcoincore_rpc::Client,
}

impl Client {
    pub fn new(rpc_url: &str, user: &str, password: &str) -> Result<Self> {
        let auth = bitcoincore_rpc::Auth::UserPass(user.to_owned(), password.to_owned());
        let rpc = bitcoincore_rpc::Client::new(rpc_url, auth)?;
        Ok(Self { rpc })
    }

    pub fn call<T>(&self, method: &str, params: &[Value]) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(self.rpc.call(method, params)?)
    }
}

// Implement getblockchaininfo method on Client
impl_client_v28__getblockchaininfo!();

fn main() -> Result<()> {
    // Connect to a regtest node
    let client = Client::new("http://127.0.0.1:18443", "rpcuser", "rpcpassword")?;

    // Call the generated method
    let info = client.getblockchaininfo()?;
    println!("Chain: {}, Blocks: {}", info.chain, info.blocks);

    Ok(())
}
```

## Development

### Project Structure

```
bitcoin-rpc-codegen/
├── src/
│   ├── generator/           # Code generation logic
│   ├── parser/              # API JSON parsing logic
│   ├── node_client/         # Bitcoin Core connection utilities
│   ├── bin/                 # Example binaries
│   ├── lib.rs               # Library entry point
│   └── main.rs              # CLI entry point
├── resources/
│   └── api.json             # Bitcoin Core API definition
└── build.rs                 # Build script for code generation
```

### Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to improve the project.

### Testing

To test the generated code with a local Bitcoin Core node:

1. Start a Bitcoin Core node in regtest mode
2. Run the example client: `cargo run --bin bitcoin_client`
