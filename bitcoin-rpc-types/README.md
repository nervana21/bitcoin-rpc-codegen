# bitcoin-rpc-types

Shared Bitcoin RPC type definitions.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-rpc-types = "0.1.0"
```

## Example

```rust
use bitcoin_rpc_types::{ApiDefinition, BtcMethod, BtcArgument, BtcResult};

// Load an API definition from JSON
let api_def: ApiDefinition = serde_json::from_str(json_data)?;

// Access methods
for method in api_def.methods.values() {
    println!("Method: {}", method.name);
    for arg in &method.arguments {
        println!("  Argument: {} ({})", arg.name, arg.type_name);
    }
}
```

## Contributing

Contributions warmly welcome.

## License

MIT OR Apache-2.0

## Related Projects

This crate is used in the Bitcoin RPC Codegen project, which generates type-safe Rust clients for Bitcoin Core's JSON-RPC API. For more information, see the main [bitcoin-rpc-codegen](https://github.com/nervana21/bitcoin-rpc-codegen) repository.
