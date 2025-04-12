# Bitcoin RPC Code Generator

A tool for generating type-safe RPC client code for Bitcoin Core's JSON-RPC API.

## Features

- Generates Rust client code from Bitcoin Core's RPC documentation
- Full type safety with Rust type definitions
- Support for all Bitcoin Core RPC methods
- Automatic parameter validation
- Comprehensive error handling

## Supported Versions

- Bitcoin Core versions: v17 to v28

## Installation

```bash
cargo install bitcoin-rpc-codegen
```

## Usage

This library is used to generate type-safe Rust client code for Bitcoin Core's JSON-RPC API. It offers a tool to generate the necessary client code based on the API documentation.

### Example

To generate client code, you can use the provided macros and functions. Here's a basic example of how you might set up your project to use the generated code:

```rust
use bitcoin_rpc_codegen::generate_client_macro;
use anyhow::Result;

fn main() -> Result<()> {
    // Example of generating client code for a specific Bitcoin Core version
    let version = "v28";
    let api_json = std::fs::read_to_string("api.json")?;
    let methods = bitcoin_rpc_codegen::parse_api_json(&api_json)?;

    // Generate client code
    for method in methods {
        let client_code = generate_client_macro(&method, version)?;
        println!("{}", client_code);
    }

    Ok(())
}
```

### Generating Code

1. Ensure you have the `api.json` file containing the Bitcoin Core RPC methods.
2. Use the `parse_api_json` function to parse the API methods.
3. Use the `generate_client_macro` function to generate the client code for each method.

This approach allows you to create a custom RPC client tailored to the specific version of Bitcoin Core you are working with.

## Development

1. Clone the repository:

```

```
