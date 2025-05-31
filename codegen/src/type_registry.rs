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
    /// Priority of the mapping
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
        registry
    }

    /// Register the default type mappings
    fn register_default_mappings(&mut self) {
        // Basic type mappings
        self.register_mapping("string", "String", false, vec![]);
        self.register_mapping("boolean", "bool", false, vec![]);
        self.register_mapping("null", "()", false, vec![]);

        // Number types
        self.register_mapping("number", "u64", false, vec![]); // Default mapping first
        self.register_mapping(
            "number",
            "u64",
            false,
            vec![
                "height", "blocks", "headers", "time", "size", "count", "index",
            ],
        );
        self.register_mapping("numeric", "f64", false, vec![]);
        self.register_mapping("amount", "bitcoin::Amount", false, vec![]);

        // Hex types
        self.register_mapping("hex", "String", false, vec![]); // Default mapping first
        self.register_mapping("hex", "bitcoin::Txid", false, vec!["txid"]);
        self.register_mapping("hex", "bitcoin::BlockHash", false, vec!["blockhash"]);
        self.register_mapping("hex", "bitcoin::ScriptBuf", false, vec!["script"]);
        self.register_mapping("hex", "bitcoin::PublicKey", false, vec!["pubkey"]);

        // Array types
        self.register_mapping("array", "Vec<serde_json::Value>", false, vec![]); // Default mapping first
        self.register_mapping(
            "array",
            "Vec<bitcoin::Address<bitcoin::address::NetworkUnchecked>>",
            false,
            vec!["address"],
        );
        self.register_mapping("array", "Vec<bitcoin::Txid>", false, vec!["txid"]);
        self.register_mapping("array", "Vec<bitcoin::BlockHash>", false, vec!["blockhash"]);
        self.register_mapping("array", "Vec<bitcoin::ScriptBuf>", false, vec!["script"]);
        self.register_mapping(
            "array",
            "Vec<String>",
            false,
            vec!["warning", "error", "message"],
        );

        // Object types
        self.register_mapping("object", "serde_json::Value", false, vec![]); // Default mapping first
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

    /// Register a new type mapping
    fn register_mapping(
        &mut self,
        rpc_type: &str,
        rust_type: &'static str,
        is_optional: bool,
        field_patterns: Vec<&'static str>,
    ) {
        // Validate that patterns don't conflict
        if let Some(existing) = self.type_mappings.get(rpc_type) {
            for existing_mapping in existing {
                for pattern in &field_patterns {
                    if existing_mapping.field_patterns.contains(pattern) {
                        panic!("Conflicting pattern '{}' for type '{}'", pattern, rpc_type);
                    }
                }
            }
        }
        let mapping = TypeMapping {
            rust_type,
            is_optional,
            field_patterns,
            priority: 0,
        };
        self.type_mappings
            .entry(rpc_type.to_string())
            .or_default()
            .push(mapping);
    }

    /// Map a Bitcoin RPC type to a Rust type
    pub fn map_type(&self, type_str: &str, field_name: &str) -> (&'static str, bool) {
        // Special case for fee_rate to always be Option<Amount>
        if field_name == "fee_rate" {
            return ("bitcoin::Amount", true);
        }

        if let Some(mappings) = self.type_mappings.get(type_str) {
            // First try exact matches
            for mapping in mappings {
                if mapping.field_patterns.contains(&field_name) {
                    return (mapping.rust_type, mapping.is_optional);
                }
            }
            // Then try partial matches
            for mapping in mappings {
                for pattern in &mapping.field_patterns {
                    if field_name.contains(pattern) {
                        return (mapping.rust_type, mapping.is_optional);
                    }
                }
            }
            // Find the default mapping (one with empty patterns)
            for mapping in mappings {
                if mapping.field_patterns.is_empty() {
                    return (mapping.rust_type, mapping.is_optional);
                }
            }
        }
        ("serde_json::Value", false)
    }

    /// Map an ApiResult to a Rust type
    pub fn map_result_type(&self, result: &ApiResult) -> (&'static str, bool) {
        let (ty, _) = self.map_type(&result.type_, &result.key_name);
        (ty, result.optional)
    }

    /// Map an ApiArgument to a Rust type
    pub fn map_argument_type(&self, arg: &ApiArgument) -> (&'static str, bool) {
        let (ty, _) = self.map_type(&arg.type_, &arg.names[0]);
        (ty, arg.optional)
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
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
    fn test_special_case_mapping() {
        let registry = TypeRegistry::new();
        assert_eq!(registry.map_type("hex", "txid"), ("bitcoin::Txid", false));
        assert_eq!(
            registry.map_type("hex", "blockhash"),
            ("bitcoin::BlockHash", false)
        );
        assert_eq!(
            registry.map_type("hex", "script"),
            ("bitcoin::ScriptBuf", false)
        );
        assert_eq!(
            registry.map_type("hex", "pubkey"),
            ("bitcoin::PublicKey", false)
        );
    }

    #[test]
    fn test_number_mapping() {
        let registry = TypeRegistry::new();
        assert_eq!(registry.map_type("number", "height"), ("u64", false));
        assert_eq!(registry.map_type("number", "blocks"), ("u64", false));
        assert_eq!(registry.map_type("number", "headers"), ("u64", false));
        assert_eq!(registry.map_type("number", "time"), ("u64", false));
        assert_eq!(registry.map_type("number", "size"), ("u64", false));
        assert_eq!(registry.map_type("number", "count"), ("u64", false));
        assert_eq!(registry.map_type("number", "index"), ("u64", false));
        assert_eq!(registry.map_type("number", "random"), ("u64", false));
    }

    #[test]
    fn test_array_mapping() {
        let registry = TypeRegistry::new();
        assert_eq!(
            registry.map_type("array", "address"),
            (
                "Vec<bitcoin::Address<bitcoin::address::NetworkUnchecked>>",
                false
            )
        );
        assert_eq!(
            registry.map_type("array", "txid"),
            ("Vec<bitcoin::Txid>", false)
        );
        assert_eq!(
            registry.map_type("array", "blockhash"),
            ("Vec<bitcoin::BlockHash>", false)
        );
        assert_eq!(
            registry.map_type("array", "script"),
            ("Vec<bitcoin::ScriptBuf>", false)
        );
        assert_eq!(
            registry.map_type("array", "warning"),
            ("Vec<String>", false)
        );
        assert_eq!(registry.map_type("array", "error"), ("Vec<String>", false));
        assert_eq!(
            registry.map_type("array", "message"),
            ("Vec<String>", false)
        );
        assert_eq!(
            registry.map_type("array", "random"),
            ("Vec<serde_json::Value>", false)
        );
    }

    #[test]
    fn test_object_mapping() {
        let registry = TypeRegistry::new();
        assert_eq!(
            registry.map_type("object", "transaction"),
            ("bitcoin::Transaction", false)
        );
        assert_eq!(
            registry.map_type("object", "block"),
            ("bitcoin::Block", false)
        );
        assert_eq!(
            registry.map_type("object", "random"),
            ("serde_json::Value", false)
        );
    }

    #[test]
    fn test_unknown_type_mapping() {
        let registry = TypeRegistry::new();
        assert_eq!(
            registry.map_type("unknown", "any"),
            ("serde_json::Value", false)
        );
    }

    #[test]
    fn test_result_type_mapping() {
        let registry = TypeRegistry::new();

        // Test with optional flag
        let result = create_test_result("string", "name", true);
        assert_eq!(registry.map_result_type(&result), ("String", true));

        // Test with special case
        let result = create_test_result("hex", "txid", false);
        assert_eq!(registry.map_result_type(&result), ("bitcoin::Txid", false));
    }

    #[test]
    fn test_argument_type_mapping() {
        let registry = TypeRegistry::new();

        // Test with optional flag
        let arg = create_test_argument("string", "name", true);
        assert_eq!(registry.map_argument_type(&arg), ("String", true));

        // Test with special case
        let arg = create_test_argument("hex", "txid", false);
        assert_eq!(registry.map_argument_type(&arg), ("bitcoin::Txid", false));
    }
}
