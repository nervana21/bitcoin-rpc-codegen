// Important design decisions will be explained here

The crate is named `rpc-api` due to name clashing with the name `core`

The crate is named `rpc-metrics` due to name clashing with other crates named `metrics`

## Port Selection in BitcoinNodeManager

### Problem

When running tests or multiple instances of Bitcoin nodes, port conflicts are a common source of issues. The default behavior of using port 0 (which bitcoind rejects) or fixed ports (which can conflict) leads to frequent test failures and user frustration.

### Solution

The `BitcoinNodeManager` implements automatic port selection when `rpc_port` is set to 0 in the `TestConfig`. This is achieved by:

1. Detecting when port 0 is specified
2. Using the OS's port allocation mechanism to find an available port
3. Using that port for the Bitcoin node

### Trade-offs

We chose to prioritize test reliability over flexibility:

#### Advantages

- Eliminates most port-related test failures
- Makes tests more reliable and deterministic
- Reduces user frustration with port conflicts
- Simplifies test setup code

#### Disadvantages

- Adds complexity to the port selection logic
- Introduces a small race condition window
- Makes port selection behavior implicit rather than explicit
- May use more system resources due to port checking

### Implementation Details

The port selection is implemented in `BitcoinNodeManager::new_with_config`:

```rust
let rpc_port = if config.rpc_port == 0 {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
    listener.local_addr()?.port()
} else {
    config.rpc_port
};
```

### Testing

The behavior is verified through the `test_auto_port_selection` test, which ensures:

1. A non-zero port is selected when port 0 is specified
2. The selected port is actually available for use
3. The port selection mechanism works reliably

### Future Considerations

While the current implementation prioritizes simplicity and reliability, we acknowledge that some users might need more control over port selection. However, we maintain a strict policy of only adding complexity when it's significantly outweighed by usability benefits.

If we do extend the functionality, it would likely be through a simple flag or environment variable, such as:

```rust
// Example of potential future extension
pub struct TestConfig {
    pub rpc_port: u16,
    pub force_port: bool,  // If true, use rpc_port even if 0
    // ... other fields ...
}
```

This would allow users to:

1. Use automatic port selection (default behavior)
2. Force a specific port (including 0) when needed
3. Maintain backward compatibility

However, we would only implement such an extension if:

1. There's clear evidence of user need
2. The complexity cost is minimal
3. The usability benefit is significant
4. It doesn't compromise the reliability of the default behavior

The current implementation serves the most common use case (testing) while keeping the codebase simple and maintainable.
