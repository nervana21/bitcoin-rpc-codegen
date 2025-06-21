# Bitcoin RPC Type System

This document describes how JSON-RPC fields in Bitcoin Core are categorized and mapped into Rust types.

The system is powered by a centralized `RpcCategory` enum and a pattern-matching registry (`CategoryRule`), which assigns every RPC field a semantic category based on its type and name. This allows us to generate consistent, correct Rust types across all methods.

---

## Semantic Categories

Each RPC field is assigned one of the following categories:

### Primitive Types

| Category       | Rust Type | Description                        |
| -------------- | --------- | ---------------------------------- |
| `String`       | `String`  | Generic string values              |
| `Boolean`      | `bool`    | `true` or `false`                  |
| `Null`         | `()`      | JSON null                          |
| `Float`        | `f64`     | Rates, probabilities, percentages  |
| `Port`         | `u16`     | Network port numbers               |
| `SmallInteger` | `u32`     | Bounded integers (e.g. minconf)    |
| `LargeInteger` | `u64`     | Heights, sizes, timestamps, counts |

### Bitcoin Types

| Category           | Rust Type            | Description                            |
| ------------------ | -------------------- | -------------------------------------- |
| `BitcoinAmount`    | `bitcoin::Amount`    | Monetary values (fees, balances, etc.) |
| `BitcoinTxid`      | `bitcoin::Txid`      | Transaction IDs                        |
| `BitcoinBlockHash` | `bitcoin::BlockHash` | Block hashes                           |
| `BitcoinAddress`   | `bitcoin::Address`   | Bech32 or legacy addresses             |

### Collection Types

| Category       | Rust Type                | Description                      |
| -------------- | ------------------------ | -------------------------------- |
| `StringArray`  | `Vec<String>`            | Lists of addresses, labels, keys |
| `BitcoinArray` | `Vec<bitcoin::Txid>`     | Lists of Bitcoin-native types    |
| `GenericArray` | `Vec<serde_json::Value>` | Catch-all for arbitrary arrays   |

### Object Types

| Category        | Rust Type           | Description                          |
| --------------- | ------------------- | ------------------------------------ |
| `BitcoinObject` | `serde_json::Value` | Reserved for future structured types |
| `GenericObject` | `serde_json::Value` | Dynamic key-value pairs              |

### Special Cases

| Category  | Rust Type           | Description                             |
| --------- | ------------------- | --------------------------------------- |
| `Dummy`   | `String` (optional) | Placeholder/testing fields              |
| `Unknown` | `serde_json::Value` | Fallback for unknown or unmapped fields |

---

## Categorization Rules

Each field is mapped according to a pattern-matching rule set:

1. **Exact type + name match** (e.g. `"amount"` fields of type `"number"` → `BitcoinAmount`)
2. **Name pattern match only** (e.g. `"feerate"`, `"balance"`, etc.)
3. **Type-based fallback** (e.g. all `"string"` types default to `String`)
4. **Wildcard fallback** to `Unknown`

These rules are maintained in the `CATEGORY_RULES` list and interpreted by the `TypeRegistry` engine. The system is designed to be:

- **Deterministic**: all fields deterministically map to one category.
- **Extensible**: new categories or patterns can be added with no disruption.
- **Auditable**: each field can be traced to its categorization logic.
