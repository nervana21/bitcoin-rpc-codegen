//! Sub-crate: **`type_registry`**
//!
//! Central registry for mapping Bitcoin RPC types to Rust types.
//! Provides `TypeRegistry` and `TypeMapping` for canonical type conversions.

use rpc_api::{ApiArgument, ApiResult};

/// Categories for RPC types based on their semantic meaning and usage patterns.
/// This enum provides a systematic way to categorize and map JSON-RPC types to Rust types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpcCategory {
    /// Generic string values
    String,
    /// Boolean true/false values
    Boolean,
    /// Null/empty values
    Null,

    /// Bitcoin transaction IDs
    BitcoinTxid,
    /// Bitcoin block hashes
    BitcoinBlockHash,
    /// Bitcoin amounts with satoshi precision
    BitcoinAmount,
    /// Bitcoin addresses (P2PKH, P2SH, Bech32, etc.)
    BitcoinAddress,

    // Numeric types with specific domains
    /// Network port numbers (0-65535)
    Port,
    /// Small bounded integers (u32)
    SmallInteger,
    /// Large integers for counts, heights, timestamps (u64)
    LargeInteger,
    /// Floating-point values for rates, probabilities, percentages, difficulties
    Float,

    // Complex types
    /// Arrays of Bitcoin-specific types
    BitcoinArray,
    /// Arrays of strings (addresses, keys, etc.)
    StringArray,
    /// Generic arrays with dynamic content
    GenericArray,
    /// Specific Bitcoin objects (Transaction, Block, etc.)
    BitcoinObject,
    /// Dynamic JSON objects
    GenericObject,

    /// Optional dummy fields for testing
    Dummy,
    /// Fallback for unrecognized types
    Unknown,
}

impl RpcCategory {
    /// Convert the category to its corresponding Rust type string
    pub fn to_rust_type(&self) -> &'static str {
        match self {
            RpcCategory::String => "String",
            RpcCategory::Boolean => "bool",
            RpcCategory::Null => "()",
            RpcCategory::BitcoinTxid => "bitcoin::Txid",
            RpcCategory::BitcoinBlockHash => "bitcoin::BlockHash",
            RpcCategory::BitcoinAmount => "bitcoin::Amount",
            RpcCategory::BitcoinAddress => "bitcoin::Address",
            RpcCategory::Port => "u16",
            RpcCategory::SmallInteger => "u32",
            RpcCategory::LargeInteger => "u64",
            RpcCategory::Float => "f64",
            RpcCategory::BitcoinArray => "Vec<bitcoin::Txid>",
            RpcCategory::StringArray => "Vec<String>",
            RpcCategory::GenericArray => "Vec<serde_json::Value>",
            RpcCategory::BitcoinObject => "serde_json::Value",
            RpcCategory::GenericObject => "serde_json::Value",
            RpcCategory::Dummy => "String",
            RpcCategory::Unknown => "serde_json::Value",
        }
    }

    /// Check if this category should be wrapped in Option<T>
    pub fn is_optional(&self) -> bool {
        matches!(self, RpcCategory::Dummy)
    }

    /// Get serde attributes for this category (if any)
    pub fn serde_attributes(&self) -> Option<&'static str> {
        match self {
            RpcCategory::BitcoinAmount => {
                Some("#[serde(deserialize_with = \"amount_from_btc_float\")]")
            }
            _ => None,
        }
    }

    /// Get a description of this category for documentation
    pub fn description(&self) -> &'static str {
        match self {
            RpcCategory::String => "Generic string values",
            RpcCategory::Boolean => "Boolean true/false values",
            RpcCategory::Null => "Null/empty values",
            RpcCategory::BitcoinTxid => "Bitcoin transaction IDs",
            RpcCategory::BitcoinBlockHash => "Bitcoin block hashes",
            RpcCategory::BitcoinAmount => "Bitcoin amounts with satoshi precision",
            RpcCategory::BitcoinAddress => "Bitcoin addresses (P2PKH, P2SH, Bech32, etc.)",
            RpcCategory::Port => "Network port numbers (0-65535)",
            RpcCategory::SmallInteger => "Small bounded integers (u32)",
            RpcCategory::LargeInteger => "Large integers for counts, heights, timestamps (u64)",
            RpcCategory::Float => "Floating-point values for rates and probabilities",
            RpcCategory::BitcoinArray => "Arrays of Bitcoin-specific types",
            RpcCategory::StringArray => "Arrays of strings (addresses, keys, etc.)",
            RpcCategory::GenericArray => "Generic arrays with dynamic content",
            RpcCategory::BitcoinObject => "Structured Bitcoin objects",
            RpcCategory::GenericObject => "Dynamic JSON objects",
            RpcCategory::Dummy => "Optional dummy fields for testing",
            RpcCategory::Unknown => "Unknown or unrecognized types",
        }
    }
}

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

    /// Categorize an RPC type based on its JSON schema type and field name
    pub fn categorize(&self, rpc_type: &str, field: &str) -> RpcCategory {
        let field_norm = normalize(field);
        let mut best_match = RpcCategory::Unknown;
        let mut best_pattern_len = 0;

        for rule in CATEGORY_RULES {
            if rule.rpc_type == "*" || rule.rpc_type == rpc_type {
                if let Some(pat) = rule.pattern {
                    let pat_norm = normalize(pat);
                    let is_exact = rule.exact && field_norm == pat_norm;
                    let is_contains = !rule.exact && field_norm.contains(&pat_norm);
                    if is_exact || is_contains {
                        let pattern_len = pat_norm.len();
                        if pattern_len > best_pattern_len {
                            best_pattern_len = pattern_len;
                            best_match = rule.category;
                        }
                    }
                } else if best_match == RpcCategory::Unknown {
                    best_match = rule.category;
                }
            }
        }
        best_match
    }

    /// Core mapper - returns (rust_type, is_optional)
    fn map(&self, rpc_type: &str, field: &str) -> (&'static str, bool) {
        let category = self.categorize(rpc_type, field);
        (category.to_rust_type(), category.is_optional())
    }

    /// For results, respect the `optional` flag on ApiResult
    pub fn map_result_type(&self, result: &ApiResult) -> (&'static str, bool) {
        // Special case: ALL result fields of "amount" type should be Float
        if result.type_ == "amount" {
            return ("f64", result.optional);
        }

        // Use description as fallback when key_name is empty
        let name = if result.key_name.is_empty() {
            &result.description
        } else {
            &result.key_name
        };
        let (ty, is_opt) = self.map(&result.type_, name);
        (ty, is_opt || result.optional)
    }

    /// For arguments
    pub fn map_argument_type(&self, arg: &ApiArgument) -> (&'static str, bool) {
        // Always use the first name - no special handling for unnamed fields in arguments
        let field = &arg.names[0];
        let (ty, is_opt) = self.map(&arg.type_, field);
        (ty, is_opt || !arg.required)
    }

    /// Get the category for a result type
    pub fn categorize_result(&self, result: &ApiResult) -> RpcCategory {
        // Use description as fallback when key_name is empty
        let name = if result.key_name.is_empty() {
            &result.description
        } else {
            &result.key_name
        };
        self.categorize(&result.type_, name)
    }

    /// Get the category for an argument type
    pub fn categorize_argument(&self, arg: &ApiArgument) -> RpcCategory {
        self.categorize(&arg.type_, &arg.names[0])
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        TypeRegistry::new()
    }
}

/// A mapping rule that categorizes RPC types based on their JSON schema type and field name patterns
struct CategoryRule {
    rpc_type: &'static str,
    pattern: Option<&'static str>,
    category: RpcCategory,
    exact: bool,
}

const CATEGORY_RULES: &[CategoryRule] = &[
    // Primitives (no pattern needed)
    CategoryRule {
        rpc_type: "string",
        pattern: None,
        category: RpcCategory::String,
        exact: false,
    },
    CategoryRule {
        rpc_type: "boolean",
        pattern: None,
        category: RpcCategory::Boolean,
        exact: false,
    },
    CategoryRule {
        rpc_type: "null",
        pattern: None,
        category: RpcCategory::Null,
        exact: false,
    },
    // Bitcoin-specific string types
    CategoryRule {
        rpc_type: "string",
        pattern: Some("txid"),
        category: RpcCategory::BitcoinTxid,
        exact: false,
    },
    CategoryRule {
        rpc_type: "string",
        pattern: Some("blockhash"),
        category: RpcCategory::BitcoinBlockHash,
        exact: false,
    },
    // Bitcoin amounts (monetary values)
    CategoryRule {
        rpc_type: "number",
        pattern: Some("amount"),
        category: RpcCategory::BitcoinAmount,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("balance"),
        category: RpcCategory::BitcoinAmount,
        exact: false,
    },
    // Handle "type": "amount" fields - specific patterns first
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("balance"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("fee_rate"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("estimated_feerate"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("maxfeerate"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("maxburnamount"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("relayfee"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("incrementalfee"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("incrementalrelayfee"),
        category: RpcCategory::Float,
        exact: false,
    },
    // Fallback rule for other "type": "amount" fields (like result amounts)
    CategoryRule {
        rpc_type: "amount",
        pattern: None,
        category: RpcCategory::BitcoinAmount,
        exact: false,
    },
    // Fee and rate fields (floating point)
    CategoryRule {
        rpc_type: "number",
        pattern: Some("fee"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("rate"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("feerate"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("maxfeerate"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("maxburnamount"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("relayfee"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("incrementalfee"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("incrementalrelayfee"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("difficulty"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("probability"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("percentage"),
        category: RpcCategory::Float,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("fee_rate"),
        category: RpcCategory::Float,
        exact: false,
    },
    // Port numbers
    CategoryRule {
        rpc_type: "number",
        pattern: Some("port"),
        category: RpcCategory::Port,
        exact: false,
    },
    // Small integers (u32)
    CategoryRule {
        rpc_type: "number",
        pattern: Some("nrequired"),
        category: RpcCategory::SmallInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("minconf"),
        category: RpcCategory::SmallInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("maxconf"),
        category: RpcCategory::SmallInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("locktime"),
        category: RpcCategory::SmallInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("version"),
        category: RpcCategory::SmallInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("verbosity"),
        category: RpcCategory::SmallInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("checklevel"),
        category: RpcCategory::SmallInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("n"),
        category: RpcCategory::SmallInteger,
        exact: true,
    },
    // 7. Large integers (u64)
    CategoryRule {
        rpc_type: "number",
        pattern: Some("blocks"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("nblocks"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("maxtries"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("height"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("count"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("index"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("size"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("time"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("conf_target"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("skip"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("nodeid"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("peer_id"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("wait"),
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    // Hex types
    CategoryRule {
        rpc_type: "hex",
        pattern: Some("txid"),
        category: RpcCategory::BitcoinTxid,
        exact: false,
    },
    CategoryRule {
        rpc_type: "hex",
        pattern: Some("blockhash"),
        category: RpcCategory::BitcoinBlockHash,
        exact: false,
    },
    CategoryRule {
        rpc_type: "hex",
        pattern: None,
        category: RpcCategory::String,
        exact: false,
    },
    // Array types
    CategoryRule {
        rpc_type: "array",
        pattern: Some("keys"),
        category: RpcCategory::StringArray,
        exact: false,
    },
    CategoryRule {
        rpc_type: "array",
        pattern: Some("addresses"),
        category: RpcCategory::StringArray,
        exact: false,
    },
    CategoryRule {
        rpc_type: "array",
        pattern: Some("wallets"),
        category: RpcCategory::StringArray,
        exact: false,
    },
    CategoryRule {
        rpc_type: "array",
        pattern: Some("txids"),
        category: RpcCategory::BitcoinArray,
        exact: false,
    },
    CategoryRule {
        rpc_type: "array",
        pattern: None,
        category: RpcCategory::GenericArray,
        exact: false,
    },
    // Object types - specific patterns first
    CategoryRule {
        rpc_type: "object",
        pattern: Some("options"),
        category: RpcCategory::GenericObject,
        exact: false,
    },
    CategoryRule {
        rpc_type: "object",
        pattern: Some("query_options"),
        category: RpcCategory::GenericObject,
        exact: false,
    },
    CategoryRule {
        rpc_type: "object",
        pattern: None,
        category: RpcCategory::GenericObject,
        exact: false,
    },
    // Specific floating-point fields that should be f64
    CategoryRule {
        rpc_type: "number",
        pattern: Some("verificationprogress"),
        category: RpcCategory::Float,
        exact: false,
    },
    // Catchall for remaining number fields
    CategoryRule {
        rpc_type: "number",
        pattern: None,
        category: RpcCategory::LargeInteger,
        exact: false,
    },
    // Dummy fields (for testing)
    CategoryRule {
        rpc_type: "string",
        pattern: Some("dummy"),
        category: RpcCategory::Dummy,
        exact: false,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("dummy"),
        category: RpcCategory::Dummy,
        exact: false,
    },
    // Fallback for unknown types
    CategoryRule {
        rpc_type: "*",
        pattern: None,
        category: RpcCategory::Unknown,
        exact: false,
    },
];
