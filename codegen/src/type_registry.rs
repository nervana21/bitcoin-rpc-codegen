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

/// A single mapping rule:
/// - `rpc_type`: the JSON schema type
/// - `pattern`: an optional substring to match on the (normalized) field/key name
/// - `rust_type`: the Rust type to use
/// - `is_optional`: wrap in `Option<T>`?
struct Rule {
    /// Get the category for a result type
    pub fn categorize_result(&self, result: &ApiResult) -> RpcCategory {
        self.categorize(&result.type_, &result.key_name)
    }

    /// Get the category for an argument type
    pub fn categorize_argument(&self, arg: &ApiArgument) -> RpcCategory {
        self.categorize(&arg.type_, &arg.names[0])
    }

    /// Generate documentation for all categories
    pub fn generate_category_docs(&self) -> String {
        let mut docs = String::new();
        docs.push_str("# Bitcoin RPC Type Categories\n\n");
        docs.push_str("This document describes the systematic categorization of JSON-RPC types into Rust types.\n\n");

        let mut categories_by_type = std::collections::HashMap::new();
        for rule in CATEGORY_RULES {
            if rule.rpc_type != "*" {
                categories_by_type
                    .entry(rule.rpc_type)
                    .or_insert_with(Vec::new)
                    .push(rule.category);
            }
        }

        for (rpc_type, categories) in categories_by_type {
            docs.push_str(&format!("## {} Types\n\n", rpc_type.to_uppercase()));
            for category in categories {
                docs.push_str(&format!(
                    "- **{}** (`{}`): {}\n",
                    format!("{:?}", category),
                    category.to_rust_type(),
                    category.description()
                ));
            }
            docs.push_str("\n");
        }

        docs
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
    Rule {
        rpc_type: "number",
        pattern: Some("port"),
        rust_type: "u16",
        is_optional: false,
    },
    Rule {
        rpc_type: "number",
        pattern: Some("nrequired"),
        rust_type: "u32",
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
