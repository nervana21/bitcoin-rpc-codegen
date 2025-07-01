## Crate Naming

The crate is named `rpc-api` due to name clashing with the name `core`.

## Port Selection in BitcoinNodeManager

### Context

When running tests or multiple Bitcoin node instances, port conflicts are a common issue. Using port 0 (which bitcoind rejects) or fixed ports (which can conflict) both have drawbacks. There's no universally "correct" answer—each approach has trade-offs.

### Current Approach

We use automatic port selection when `rpc_port` is set to 0 in `TestConfig`, letting the OS allocate an available port for the Bitcoin node:

```rust
let rpc_port = if config.rpc_port == 0 {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
    listener.local_addr()?.port()
} else {
    config.rpc_port
};
```

This logic is modular and can be adapted if a better solution emerges.

### Why This Design?

- Both fixed and dynamic port selection have pros and cons. Dynamic selection reduces test flakiness and user friction, which is valuable for our main use case.
- The implementation keeps things simple for most users, while still allowing for future changes if needed.

### Trade-offs

- **Pros:** Reduces port conflicts, improves test reliability, and simplifies setup.
- **Cons:** Adds a small amount of logic and a potential (but rare) race condition.

### Future Evolution

If user needs or new constraints arise, this logic can be easily adapted. For example, a flag or environment variable could allow users to force a specific port or opt into different selection strategies:

```rust
pub struct TestConfig {
    pub rpc_port: u16,
    pub force_port: bool,  // If true, use rpc_port even if 0
    // ... other fields ...
}
```

## RPC Batching

### Context

Sequential RPC calls can introduce unnecessary latency due to repeated network roundtrips. There are multiple ways to address this, each with its own trade-offs.

### Current Approach

We provide built-in RPC batching, allowing multiple RPC calls to be combined into a single network request. This is implemented via a fluent interface in the transport layer:

```rust
let results = client
    .batch()
    .getblockcount()
    .getnetworkinfo()
    .getdifficulty()
    .execute()
    .await?;
```

Partial failures within a batch are returned as individual errors in the results array, so users can handle them as needed.

### Why This Design?

- Batching reduces network roundtrips and improves performance for latency-sensitive or RPC-heavy workloads.
- The fluent interface is ergonomic and keeps batching logic out of user code.

### Trade-offs

- **Pros:** Fewer network roundtrips, improved performance, more efficient resource usage.
- **Cons:** Slightly increased complexity in the transport layer, and users must handle partial failures within batches.

### Future Evolution

The batching implementation can be improved or replaced as needed. Potential future enhancements include:

- Improved error reporting and partial batch response handling
- Automatic batching optimizations based on request frequency or patterns

The current design provides immediate performance benefits while keeping future options open.
