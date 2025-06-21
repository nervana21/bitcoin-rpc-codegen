use rpc_api::{ApiArgument, ApiResult};

/// Categories for RPC types based on their semantic meaning and usage patterns.
/// This enum provides a systematic way to categorize and map JSON-RPC types to Rust types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpcCategory {
    // Primitive types
    String,
    Boolean,
    Null,

    // Bitcoin-specific types
    BitcoinTxid,
    BitcoinBlockHash,
    BitcoinAmount,
    BitcoinAddress, // NEW: bitcoin::Address for address fields

    // Numeric types with specific domains
    Port,         // u16: network ports
    SmallInteger, // u32: bounded integers (minconf, locktime, version, verbosity)
    LargeInteger, // u64: block heights, counts, sizes, timestamps
    Float,        // f64: rates, probabilities, percentages, difficulties

    // Complex types
    BitcoinArray,  // Vec<bitcoin::*> for typed Bitcoin arrays
    StringArray,   // Vec<String> for address lists, keys, etc.
    GenericArray,  // Vec<serde_json::Value> for complex arrays
    BitcoinObject, // Specific Bitcoin objects (Transaction, Block, etc.)
    GenericObject, // serde_json::Value for dynamic objects

    // Special cases
    Dummy,   // Optional fields for testing
    Unknown, // Fallback for unrecognized types
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
                    if field_norm.contains(&pat_norm) {
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

        println!(
            "[categorize] rpc_type={}, field='{}' → {:?}",
            rpc_type, field, best_match
        );

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
            println!(
                "[map_result_type] key='{}', type='{}' → 'f64' (amount result)",
                result.key_name, result.type_
            );
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
        println!(
            "[map_argument_type] field='{}', type='{}' → '{}'",
            field, arg.type_, ty
        );
        (ty, is_opt || arg.optional)
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
}

const CATEGORY_RULES: &[CategoryRule] = &[
    // 1. Primitives (no pattern needed)
    CategoryRule {
        rpc_type: "string",
        pattern: None,
        category: RpcCategory::String,
    },
    CategoryRule {
        rpc_type: "boolean",
        pattern: None,
        category: RpcCategory::Boolean,
    },
    CategoryRule {
        rpc_type: "null",
        pattern: None,
        category: RpcCategory::Null,
    },
    // 2. Bitcoin-specific string types
    CategoryRule {
        rpc_type: "string",
        pattern: Some("txid"),
        category: RpcCategory::BitcoinTxid,
    },
    CategoryRule {
        rpc_type: "string",
        pattern: Some("blockhash"),
        category: RpcCategory::BitcoinBlockHash,
    },
    // 3. Bitcoin amounts (monetary values)
    CategoryRule {
        rpc_type: "number",
        pattern: Some("amount"),
        category: RpcCategory::BitcoinAmount,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("balance"),
        category: RpcCategory::BitcoinAmount,
    },
    // NEW: Handle "type": "amount" fields - specific patterns first
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("balance"), // Explicit balance fields
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("fee_rate"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("estimated_feerate"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("maxfeerate"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("maxburnamount"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("relayfee"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("incrementalfee"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "amount",
        pattern: Some("incrementalrelayfee"),
        category: RpcCategory::Float,
    },
    // Fallback rule for other "type": "amount" fields (like result amounts)
    CategoryRule {
        rpc_type: "amount",
        pattern: None,
        category: RpcCategory::BitcoinAmount,
    },
    // 4. Fee and rate fields (floating point)
    CategoryRule {
        rpc_type: "number",
        pattern: Some("fee"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("rate"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("feerate"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("maxfeerate"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("maxburnamount"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("relayfee"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("incrementalfee"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("incrementalrelayfee"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("difficulty"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("probability"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("percentage"),
        category: RpcCategory::Float,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("fee_rate"),
        category: RpcCategory::Float,
    },
    // 5. Port numbers
    CategoryRule {
        rpc_type: "number",
        pattern: Some("port"),
        category: RpcCategory::Port,
    },
    // 6. Small integers (u32)
    CategoryRule {
        rpc_type: "number",
        pattern: Some("nrequired"),
        category: RpcCategory::SmallInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("minconf"),
        category: RpcCategory::SmallInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("maxconf"),
        category: RpcCategory::SmallInteger, // NEW: Should match minconf
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("locktime"),
        category: RpcCategory::SmallInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("version"),
        category: RpcCategory::SmallInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("verbosity"),
        category: RpcCategory::SmallInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("checklevel"),
        category: RpcCategory::SmallInteger, // NEW: verifychain checklevel (0-4)
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("n"),
        category: RpcCategory::SmallInteger, // NEW: gettxout output index
    },
    // 7. Large integers (u64)
    CategoryRule {
        rpc_type: "number",
        pattern: Some("blocks"),
        category: RpcCategory::LargeInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("nblocks"),
        category: RpcCategory::LargeInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("maxtries"),
        category: RpcCategory::LargeInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("height"),
        category: RpcCategory::LargeInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("count"),
        category: RpcCategory::LargeInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("index"),
        category: RpcCategory::LargeInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("size"),
        category: RpcCategory::LargeInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("time"),
        category: RpcCategory::LargeInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("conf_target"),
        category: RpcCategory::LargeInteger,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("skip"),
        category: RpcCategory::LargeInteger, // NEW: listtransactions skip
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("nodeid"),
        category: RpcCategory::LargeInteger, // NEW: disconnectnode nodeid
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("peer_id"),
        category: RpcCategory::LargeInteger, // NEW: getblockfrompeer peer_id
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("wait"),
        category: RpcCategory::LargeInteger, // NEW: stop wait parameter
    },
    // 8. Hex types
    CategoryRule {
        rpc_type: "hex",
        pattern: Some("txid"),
        category: RpcCategory::BitcoinTxid,
    },
    CategoryRule {
        rpc_type: "hex",
        pattern: Some("blockhash"),
        category: RpcCategory::BitcoinBlockHash,
    },
    CategoryRule {
        rpc_type: "hex",
        pattern: None,
        category: RpcCategory::String,
    },
    // 9. Array types
    CategoryRule {
        rpc_type: "array",
        pattern: Some("keys"),
        category: RpcCategory::StringArray,
    },
    CategoryRule {
        rpc_type: "array",
        pattern: Some("addresses"),
        category: RpcCategory::StringArray,
    },
    CategoryRule {
        rpc_type: "array",
        pattern: Some("wallets"),
        category: RpcCategory::StringArray,
    },
    CategoryRule {
        rpc_type: "array",
        pattern: Some("txids"),
        category: RpcCategory::BitcoinArray, // NEW: Vec<bitcoin::Txid>
    },
    CategoryRule {
        rpc_type: "array",
        pattern: None,
        category: RpcCategory::GenericArray,
    },
    // 10. Object types - specific patterns first
    CategoryRule {
        rpc_type: "object",
        pattern: Some("options"),
        category: RpcCategory::GenericObject,
    },
    CategoryRule {
        rpc_type: "object",
        pattern: Some("query_options"),
        category: RpcCategory::GenericObject,
    },
    CategoryRule {
        rpc_type: "object",
        pattern: None,
        category: RpcCategory::GenericObject,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: None,
        category: RpcCategory::LargeInteger, // ✅ Safe default for height, counts, etc.
    },
    // 11. Dummy fields (for testing)
    CategoryRule {
        rpc_type: "string",
        pattern: Some("dummy"),
        category: RpcCategory::Dummy,
    },
    CategoryRule {
        rpc_type: "number",
        pattern: Some("dummy"),
        category: RpcCategory::Dummy,
    },
    // 12. Fallback for unknown types
    CategoryRule {
        rpc_type: "*",
        pattern: None,
        category: RpcCategory::Unknown,
    },
];
