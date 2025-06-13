use rpc_api::{ApiArgument, ApiResult};
use std::collections::HashMap;

/// Represents a mapping from Bitcoin RPC types to Rust types
#[derive(Debug, Clone)]
pub struct TypeMapping {
    /// The base Rust type to use
    pub rust_type: &'static str,
    /// Whether this type should be wrapped in Option<T>
    pub is_optional: bool,
    /// Special case field names that should use this mapping
    pub field_patterns: Vec<&'static str>,
    /// Priority of the mapping (higher wins)
    pub priority: u8,
}

/// Central registry for mapping Bitcoin RPC types to Rust types
pub struct TypeRegistry {
    type_mappings: HashMap<String, Vec<TypeMapping>>,
}

impl TypeRegistry {
    /// Create a new TypeRegistry with default mappings
    pub fn new() -> Self {
        let mut registry = Self {
            type_mappings: HashMap::new(),
        };
        registry.register_default_mappings();
        registry.finalize();
        registry
    }

    /// Register the default type mappings
    fn register_default_mappings(&mut self) {
        // 1. Basic JSON → Rust defaults
        self.register_mapping("string", "String", false, vec![]);
        self.register_mapping("boolean", "bool", false, vec![]);
        self.register_mapping("null", "()", false, vec![]);

        // 2. Numeric & amount mappings in descending priority
        number_mapping(
            self,
            "bitcoin::Amount",
            &[
                "amount",
                "balance",
                "fee",
                "maxburnamount",
                "maxfeerate",
                "maximumamount",
                "minimumamount",
                "minimumsumamount",
                "total",
                "value",
            ],
            50,
        );
        number_mapping(
            self,
            "f64",
            &[
                "difficulty",
                "feerate",
                "feerate",
                "percentage",
                "probability",
                "rate",
            ],
            40,
        );
        number_mapping(
            self,
            "u64",
            &[
                "blocks",
                "confirmations",
                "count",
                "headers",
                "height",
                "index",
                "maxtries",
                "size",
                "time",
                "version",
            ],
            30,
        );
        number_mapping(self, "u128", &["bits", "hashrate", "target"], 20);
        // minconf is a small integer
        number_mapping(self, "u32", &["conftarget", "minconf"], 60);

        // 3. Hex
        self.register_mapping("hex", "String", false, vec![]);
        self.register_mapping("hex", "bitcoin::Txid", false, vec!["txid"]);
        self.register_mapping("hex", "bitcoin::BlockHash", false, vec!["blockhash"]);
        self.register_mapping("hex", "bitcoin::ScriptBuf", false, vec!["script"]);
        self.register_mapping("hex", "bitcoin::PublicKey", false, vec!["pubkey"]);

        // 4. Arrays
        self.register_mapping("array", "Vec<serde_json::Value>", false, vec![]);
        self.register_mapping(
            "array",
            "Vec<bitcoin::Address<bitcoin::address::NetworkUnchecked>>",
            false,
            vec!["address"],
        );
        self.register_mapping("array", "Vec<bitcoin::BlockHash>", false, vec!["blockhash"]);
        self.register_mapping("array", "Vec<bitcoin::ScriptBuf>", false, vec!["script"]);
        self.register_mapping("array", "Vec<bitcoin::Txid>", false, vec!["txid"]);
        self.register_mapping(
            "array",
            "Vec<String>",
            false,
            vec!["error", "message", "warning"],
        );

        // 5. Objects
        self.register_mapping("object", "serde_json::Value", false, vec![]);
        self.register_mapping("object", "bitcoin::Transaction", false, vec!["transaction"]);
        self.register_mapping("object", "bitcoin::Block", false, vec!["block"]);
        self.register_mapping(
            "object-named-parameters",
            "serde_json::Value",
            false,
            vec![],
        );
        self.register_mapping("object-user-keys", "serde_json::Value", false, vec![]);
        self.register_mapping("mixed", "serde_json::Value", false, vec![]);
    }

    /// Sort all mappings by priority (descending) so highest priority matches first
    fn finalize(&mut self) {
        for mappings in self.type_mappings.values_mut() {
            mappings.sort_by_key(|m| std::cmp::Reverse(m.priority));
        }
    }

    /// Register a new type mapping given RPC type string (e.g. "number", "array")
    pub fn register(&mut self, rpc_type: &str, mapping: TypeMapping) {
        // ensure no duplicate field-pattern
        if let Some(existing) = self.type_mappings.get(rpc_type) {
            for pat in &mapping.field_patterns {
                for ex in existing {
                    if ex.field_patterns.contains(pat) {
                        panic!("pattern conflict: `{}` for `{}`", pat, rpc_type);
                    }
                }
            }
        }
        self.type_mappings
            .entry(rpc_type.to_string())
            .or_default()
            .push(mapping);
    }

    /// Helper to register simple mappings (priority defaults to 0)
    fn register_mapping(
        &mut self,
        rpc_type: &str,
        rust_type: &'static str,
        is_optional: bool,
        field_patterns: Vec<&'static str>,
    ) {
        let m = TypeMapping {
            rust_type,
            is_optional,
            field_patterns,
            priority: 0,
        };
        self.type_mappings
            .entry(rpc_type.to_string())
            .or_default()
            .push(m);
    }

    /// Normalize field names and patterns for matching
    fn normalize(name: &str) -> String {
        name.to_lowercase().replace(&['_', '-', ' '][..], "")
    }

    /// Map a Bitcoin RPC type and its field (or method) name to Rust
    pub fn map_type(&self, type_str: &str, field_name: &str) -> (&'static str, bool) {
        let key = type_str;
        if let Some(mappings) = self.type_mappings.get(key) {
            let norm_field = Self::normalize(field_name);
            // 1) Pattern-based match
            for m in mappings {
                if m.field_patterns
                    .iter()
                    .any(|pat| norm_field.contains(&Self::normalize(pat)))
                {
                    return (m.rust_type, m.is_optional);
                }
            }
            // 2) Default empty-pattern mapping
            if let Some(d) = mappings.iter().find(|m| m.field_patterns.is_empty()) {
                return (d.rust_type, d.is_optional);
            }
        }
        // Fallback
        ("serde_json::Value", false)
    }

    /// Map an ApiResult to a Rust type
    pub fn map_result_type(&self, result: &ApiResult) -> (&'static str, bool) {
        let (ty, opt) = self.map_type(&result.type_, &result.key_name);
        (ty, result.optional || opt)
    }

    /// Map an ApiArgument to a Rust type
    pub fn map_argument_type(&self, arg: &ApiArgument) -> (&'static str, bool) {
        self.map_type(&arg.type_, &arg.names[0])
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper for numeric mappings to reduce boilerplate
fn number_mapping(
    reg: &mut TypeRegistry,
    rust_type: &'static str,
    patterns: &[&'static str],
    priority: u8,
) {
    reg.register(
        "number",
        TypeMapping {
            rust_type,
            is_optional: false,
            field_patterns: patterns.to_vec(),
            priority,
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rpc_api::{ApiArgument, ApiResult};

    fn create_test_result(type_: &str, key_name: &str, optional: bool) -> ApiResult {
        ApiResult {
            type_: type_.to_string(),
            key_name: key_name.to_string(),
            description: String::new(),
            inner: vec![],
            optional,
        }
    }

    fn create_test_argument(type_: &str, name: &str, optional: bool) -> ApiArgument {
        ApiArgument {
            names: vec![name.to_string()],
            type_: type_.to_string(),
            optional,
            description: String::new(),
        }
    }

    #[test]
    fn test_basic_type_mapping() {
        let registry = TypeRegistry::new();
        assert_eq!(registry.map_type("string", "any"), ("String", false));
        assert_eq!(registry.map_type("boolean", "any"), ("bool", false));
        assert_eq!(registry.map_type("null", "any"), ("()", false));
    }

    #[test]
    fn test_priority_and_normalization() {
        let mut reg = TypeRegistry::new();
        reg.finalize();
        // fee_rate from default mappings
        assert_eq!(
            reg.map_type("number", "fee_rate"),
            ("bitcoin::Amount", false)
        );
        assert_eq!(
            reg.map_type("number", "FeeRate"),
            ("bitcoin::Amount", false)
        );
        // custom overrides
        assert_eq!(reg.map_type("number", "maxburnamount"), ("u64", false));
        assert_eq!(reg.map_type("number", "minconf"), ("u32", false));
    }

    #[test]
    fn test_hex_array_object() {
        let registry = TypeRegistry::new();
        assert_eq!(registry.map_type("hex", "txid"), ("bitcoin::Txid", false));
        assert_eq!(
            registry.map_type("array", "address"),
            (
                "Vec<bitcoin::Address<bitcoin::address::NetworkUnchecked>>",
                false
            )
        );
        assert_eq!(
            registry.map_type("object", "transaction"),
            ("bitcoin::Transaction", false)
        );
    }

    #[test]
    fn test_unknown_fallback() {
        let registry = TypeRegistry::new();
        assert_eq!(
            registry.map_type("unknown", "any"),
            ("serde_json::Value", false)
        );
    }

    #[test]
    fn test_map_argument_and_result() {
        let registry = TypeRegistry::new();
        let arg = create_test_argument("number", "height", false);
        assert_eq!(registry.map_argument_type(&arg), ("u64", false));
        let res = create_test_result("hex", "txid", true);
        assert_eq!(registry.map_result_type(&res), ("bitcoin::Txid", true));
    }
}
