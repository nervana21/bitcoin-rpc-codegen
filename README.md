# Bitcoin RPC Code Generator

A tool for generating type-safe Rust RPC client code for Bitcoin Core's JSON-RPC API. This project parses Bitcoin Core's API documentation (via a JSON file) and automatically produces both client methods and Rust types for interacting with the node.

## Features

- **Type-safe Code Generation:** Automatically produces Rust client methods and strongly typed response structures based on Bitcoin Core's RPC API documentation.
- **Support for Multiple Versions:** Generate clients for Bitcoin Core versions from v17 through v28. (Currently, the default configuration targets v28.)
- **Automatic Parameter Validation:** Generated methods include basic input validation and enforce type safety at compile time.
- **Error Handling:** Uses `anyhow` for comprehensive error reporting, ensuring that RPC errors and parsing issues are clear.
- **Integration with Bitcoin Libraries:** Leverages industry-standard crates such as `bitcoin` and `bitcoincore-rpc` for Bitcoin types and connecting to a Bitcoin Core node.
- **Extensible and Maintainable:** The generated code is organized into client and types modules (for example, the generated files are produced into Cargo's build directory via `OUT_DIR`). A dedicated node connection module (`src/node_client.rs`) abstracts the low-level RPC connection, making integration with your regtest (or other network) node easy.

## Supported Versions

- Bitcoin Core versions: v17 to v28  
  (The code generation supports all versions, though you can focus on v28 by default.)

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

## Usage

The library is used both as a standalone code generator and as an integration component in your own projects. At a high level, the tool:

1. Parses the API Definition: Reads an api.json file that defines Bitcoin Core RPC commands.
2. Generates Client Code and Types: Using functions like parse_api_json, generate_client_macro, and generate_return_type, it creates client methods (via macros) and type definitions.

## Output

The generated code is written into Cargo's build directory (OUT_DIR). You can include the generated code in your crate at compile time using, for example:

```rust
include!(concat!(env!("OUT_DIR"), "/client/src/v28/blockchain.rs"));
include!(concat!(env!("OUT_DIR"), "/types/src/v28/blockchain.rs"));
```

This approach ensures that the generated files are available to your project without cluttering your source tree. (During development you may also choose to output them into a generated/ directory at the project root for inspection.)

## Example

Below is an example that demonstrates generating client code and making an RPC call to a Bitcoin Core regtest node using the generated code (focusing on v28):

```rust
use bitcoin_rpc_codegen::generate_client_macro;
use bitcoin_rpc_codegen::parser::{parse_api_json, ApiMethod};
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    // Read Bitcoin Core API documentation.
    let api_json = fs::read_to_string("api.json")?;
    let methods: Vec<ApiMethod> = parse_api_json(&api_json)?;

    // Code generation for v28; generated files will be written to OUT_DIR.
    for method in methods {
        let client_code = generate_client_macro(&method, "v28");
        println!("{}", client_code);
    }

    // At this point, the generated code (client methods and types) is available in your build output.
    // You can include it in your project via the include! macro and concat!(env!("OUT_DIR"), ...).

    Ok(())
}
```

## Integrating with a Bitcoin Node

In your project, you can combine the generated client code with a node connection module. The typical flow is:

1. Node Connection: Use the provided src/node_client.rs to create a low-level connection to your regtest node (using the bitcoincore-rpc crate). For example:

```rust
use bitcoin_rpc_codegen::node_client::NodeClient;
use anyhow::Result;

let rpc_url = "http://127.0.0.1:18443";
let user = "regtestuser";
let password = "regtestpass";
let node_client = NodeClient::new(rpc_url, user, password)?;
```

2. Using the Generated Client: The generated client modules (under generated/client/src/v28) contain macros (e.g. impl_client_v28\_\_getblockchaininfo!()) that extend a client type (usually named Client) with RPC methods. In an integration test or application, you could wrap your NodeClient in a simple Client and then invoke the generated methods:

```rust
// Define a simple Client type that the generated macros extend:
pub struct Client {
    rpc: bitcoincore_rpc::Client,
}

impl Client {
    pub fn new(rpc_url: &str, user: &str, password: &str) -> anyhow::Result<Self> {
        let auth = bitcoincore_rpc::Auth::UserPass(user.to_owned(), password.to_owned());
        let rpc = bitcoincore_rpc::Client::new(rpc_url, auth)?;
        Ok(Self { rpc })
    }

    pub fn call<T>(&self, method: &str, params: &[impl serde::Serialize]) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(self.rpc.call(method, params)?)
    }
}

// Extend Client with a generated method (for example, getblockchaininfo).
impl_client_v28__getblockchaininfo!();

// Now you can use the generated method:
use bitcoin_rpc_codegen::generated::types::v28::blockchain::GetblockchaininfoResponse;
let client = Client::new("http://127.0.0.1:18443", "regtestuser", "regtestpass")?;
let info: serde_json::Value = client.getblockchaininfo()?;
let info: GetblockchaininfoResponse = serde_json::from_value(info)?;
println!("Blockchain info: {:?}", info);
```

## Development & Testing

Ongoing. Consider contributing to the project or providing feedback for future updates.
