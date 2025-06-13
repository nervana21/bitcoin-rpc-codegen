# Bitcoin RPC Type System

This document describes how JSON-RPC return values from Bitcoin Core are mapped into Rust types. Each field is assigned one of several categories based on its intended use and precision requirements.

---

## Integer Values (`u64`)

These are exact, nonnegative whole-number values:

- Block heights
- Transaction counts
- Version numbers
- Confirmation counts
- Block counts
- Header counts
- Index values
- Size values (bytes)
- Time values (timestamps)

## Small Integers (`u32`)

Used when the domain is known to be bounded and smaller:

- Confirmation targets
- Minimum confirmations

## Floating Point Values (`f64`)

For values requiring fractional precision:

- Mining difficulty
- Fee rates (BTC/kB)
- Probabilities
- Percentages
- General rates (e.g. mempool growth)

## Bitcoin Amounts (`bitcoin::Amount`)

Monetary values with satoshi precision:

- Transaction amounts
- Fees
- Balances
- Minimum/maximum transaction amounts
- Maximum fee rates
- Total burned amounts

## Large Numbers (`u128`)

Values that can exceed 2<sup>64</sup> yet remain integer:

- Difficulty targets (raw bits)
- Hash rates (H/s)
- Block “bits” field

## Hex Values

Encoded as hex strings in JSON, but mapped to typed wrappers:

- Transaction IDs (`bitcoin::Txid`)
- Block hashes (`bitcoin::BlockHash`)
- Scripts (`bitcoin::ScriptBuf`)
- Public keys (`bitcoin::PublicKey`)
- Raw hex blobs (`String`)

## Arrays

Lists of items; element type depends on context:

- Address lists (`Vec<bitcoin::Address>`)
- Block hash lists (`Vec<bitcoin::BlockHash>`)
- Script lists (`Vec<bitcoin::ScriptBuf>`)
- Transaction ID lists (`Vec<bitcoin::Txid>`)
- Error/warning lists (`Vec<String>`)
- Generic arrays (`Vec<serde_json::Value>`)

## Objects

Nested JSON objects, mapped to rich Rust types or generic values:

- Transactions (`bitcoin::Transaction`)
- Blocks (`bitcoin::Block`)
- Wallet info (`GetWalletInfo`)
- Generic/unknown objects (`serde_json::Value`)

---

Each mapping is selected by a priority system:

1. Exact RPC type hint (e.g. `"txid" → Txid`)
2. JSON schema primitive (`"integer"` vs. `"number"`)
3. Field name pattern (e.g. `*_amount` → `Amount`)
4. Fallback to `serde_json::Value`

> **Next steps TODO**: Introduce a `RpcCategory` enum and wire it through `TypeRegistry` so that both code and docs are generated programmatically.
