use rpc_api::{ApiArgument, ApiResult};

/// A single mapping rule:
/// - `rpc_type`: the JSON schema type
/// - `pattern`: an optional substring to match on the (normalized) field/key name
/// - `rust_type`: the Rust type to use
/// - `is_optional`: wrap in `Option<T>`?
struct Rule {
    rpc_type: &'static str,
    pattern: Option<&'static str>,
    rust_type: &'static str,
    is_optional: bool,
}

const RULES: &[Rule] = &[
    // 1. Primitives
    Rule {
        rpc_type: "string",
        pattern: None,
        rust_type: "String",
        is_optional: false,
    },
    Rule {
        rpc_type: "boolean",
        pattern: None,
        rust_type: "bool",
        is_optional: false,
    },
    Rule {
        rpc_type: "null",
        pattern: None,
        rust_type: "()",
        is_optional: false,
    },
    // 2. Numbers & amounts (patterned first)
    // Amount-like patterns (BTC amounts, satoshis, etc.)
    Rule {
        rpc_type: "number",
        pattern: Some("amount"),
        rust_type: "bitcoin::Amount",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("balance"),
        rust_type: "bitcoin::Amount",
        is_optional: false,
    },
    // Fee fields → floating-point BTC amounts
    Rule {
        rpc_type: "number",
        pattern: Some("fee"),
        rust_type: "f64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("rate"),
        rust_type: "f64",
        is_optional: false,
    },
    // Specific fee-related overrides to float
    Rule {
        rpc_type: "number",
        pattern: Some("relayfee"),
        rust_type: "f64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("incrementalfee"),
        rust_type: "f64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("incrementalrelayfee"),
        rust_type: "f64",
        is_optional: false,
    },
    // Integer patterns (u64)
    // Block count
    Rule {
        rpc_type: "number",
        pattern: Some("blocks"),
        rust_type: "u64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("nblocks"),
        rust_type: "u64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("maxtries"),
        rust_type: "u64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("height"),
        rust_type: "u64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("count"),
        rust_type: "u64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("index"),
        rust_type: "u64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("size"),
        rust_type: "u64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("time"),
        rust_type: "u64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("conf_target"),
        rust_type: "u64",
        is_optional: false,
    },
    // Integer patterns (u32)
    Rule {
        rpc_type: "number",
        pattern: Some("minconf"),
        rust_type: "u32",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("locktime"),
        rust_type: "u32",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("version"),
        rust_type: "u32",
        is_optional: false,
    },
    // Float patterns
    Rule {
        rpc_type: "number",
        pattern: Some("difficulty"),
        rust_type: "f64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("probability"),
        rust_type: "f64",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("percentage"),
        rust_type: "f64",
        is_optional: false,
    },
    // fallback number → default to floating-point
    Rule {
        rpc_type: "number",
        pattern: None,
        rust_type: "f64",
        is_optional: false,
    },
    // 3. Hex blobs
    Rule {
        rpc_type: "hex",
        pattern: Some("txid"),
        rust_type: "bitcoin::Txid",
        is_optional: false,
    },
    Rule {
        rpc_type: "hex",
        pattern: Some("blockhash"),
        rust_type: "bitcoin::BlockHash",
        is_optional: false,
    },
    Rule {
        rpc_type: "hex",
        pattern: None,
        rust_type: "String",
        is_optional: false,
    },
    // 4. Arrays
    Rule {
        rpc_type: "array",
        pattern: Some("address"),
        rust_type: "Vec<bitcoin::Address<bitcoin::address::NetworkUnchecked>>",
        is_optional: false,
    },
    Rule {
        rpc_type: "array",
        pattern: None,
        rust_type: "Vec<serde_json::Value>",
        is_optional: false,
    },
    // 5. Objects
    Rule {
        rpc_type: "object",
        pattern: Some("transaction"),
        rust_type: "bitcoin::Transaction",
        is_optional: false,
    },
    Rule {
        rpc_type: "object",
        pattern: None,
        rust_type: "serde_json::Value",
        is_optional: false,
    },
    // 6. Everything else
    Rule {
        rpc_type: "*",
        pattern: None,
        rust_type: "serde_json::Value",
        is_optional: false,
    },
];

/// Normalize names by lowercasing and stripping `_`, `-`, and spaces.
fn normalize(name: &str) -> String {
    name.chars()
        .filter(|c| !matches!(c, '_' | '-' | ' '))
        .flat_map(|c| c.to_lowercase())
        .collect()
}

/// A registry of type mappings for JSON RPC types to Rust types.
///
/// This registry provides a centralized mapping of JSON RPC type identifiers
/// (e.g., `"string"`, `"number"`, `"boolean"`, etc.) to their corresponding
/// Rust type names (`String`, `f64`, `bool`, etc.). It also handles optional
/// fields and supports wildcard matching for unknown or dynamic types.
pub struct TypeRegistry;

impl TypeRegistry {
    /// Creates a new `TypeRegistry` instance.
    ///
    /// This constructor is provided for completeness, but the `TypeRegistry`
    /// is a stateless singleton and does not need to be instantiated.
    pub fn new() -> Self {
        TypeRegistry
    }

    /// Core mapper
    fn map(&self, rpc_type: &str, field: &str) -> (&'static str, bool) {
        let field_norm = normalize(field);
        // Scan rules top to bottom
        for rule in RULES {
            if rule.rpc_type == "*" || rule.rpc_type == rpc_type {
                if let Some(pat) = rule.pattern {
                    if !field_norm.contains(&normalize(pat)) {
                        continue;
                    }
                }
                return (rule.rust_type, rule.is_optional);
            }
        }
        // never reached, but fallback anyway
        ("serde_json::Value", false)
    }

    /// For results, respect the `optional` flag on ApiResult
    pub fn map_result_type(&self, result: &ApiResult) -> (&'static str, bool) {
        let (ty, is_opt) = self.map(&result.type_, &result.key_name);
        (ty, is_opt || result.optional)
    }

    /// For arguments
    pub fn map_argument_type(&self, arg: &ApiArgument) -> (&'static str, bool) {
        // just use its first name
        let (ty, is_opt) = self.map(&arg.type_, &arg.names[0]);
        (ty, is_opt || arg.optional)
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        TypeRegistry::new()
    }
}
