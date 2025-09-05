# Bitcoin RPC Type System

This document describes how JSON-RPC fields in Bitcoin Core are categorized and mapped into Rust types using a centralized `RpcCategory` enum and pattern-matching registry.

## Type Categories

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

### Collections & Objects

| Category        | Rust Type                | Description                            |
| --------------- | ------------------------ | -------------------------------------- |
| `StringArray`   | `Vec<String>`            | Lists of addresses, labels, keys       |
| `BitcoinArray`  | `Vec<bitcoin::Txid>`     | Lists of Bitcoin-native types          |
| `GenericArray`  | `Vec<serde_json::Value>` | Catch-all for arbitrary arrays         |
| `HashOrHeight`  | `serde_json::Value`      | Block hash (string) or height (number) |
| `GenericObject` | `serde_json::Value`      | Dynamic key-value pairs                |

### Special Cases

| Category  | Rust Type           | Description                             |
| --------- | ------------------- | --------------------------------------- |
| `Dummy`   | `String` (optional) | Placeholder/testing fields              |
| `Unknown` | `serde_json::Value` | Fallback for unknown or unmapped fields |

## Key Rules

Fields are categorized using pattern matching on JSON type + field name:

- **Bitcoin-specific**: `"string"` + `"txid"` → `BitcoinTxid`
- **Monetary**: `"number"` + `"amount"` → `BitcoinAmount`
- **Fees/Rates**: `"number"` + `"fee"` → `Float`
- **Numeric domains**: `"number"` + `"port"` → `Port` (u16)
- **Arrays**: `"array"` + `"txids"` → `BitcoinArray`

## Special Handling

- **Optional fields**: Non-required fields are wrapped in `Option<T>`
- **Union types**: Fields with "string or numeric" type use `serde_json::Value`
- **Amount override**: All `"amount"` type fields become `Float` (f64)
- **Serde attributes**: `BitcoinAmount` includes custom deserialization
