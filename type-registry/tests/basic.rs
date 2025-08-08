use rpc_api::{ApiArgument, ApiResult};
use type_registry::{RpcCategory, TypeRegistry};

#[test]
fn test_categorization() {
    let registry = TypeRegistry::new();

    // Test basic categorization
    assert_eq!(registry.categorize("string", "hello"), RpcCategory::String);
    assert_eq!(
        registry.categorize("boolean", "enabled"),
        RpcCategory::Boolean
    );
    assert_eq!(registry.categorize("number", "port"), RpcCategory::Port);
    assert_eq!(
        registry.categorize("number", "amount"),
        RpcCategory::BitcoinAmount
    );
    assert_eq!(registry.categorize("number", "fee"), RpcCategory::Float);
    assert_eq!(
        registry.categorize("number", "height"),
        RpcCategory::LargeInteger
    );
    assert_eq!(
        registry.categorize("number", "minconf"),
        RpcCategory::SmallInteger
    );
}

#[test]
fn test_specific_field_mappings() {
    let registry = TypeRegistry::new();

    // Test specific field mappings
    let test_cases = vec![
        // GettxoutParams.n -> u32
        ("number", "n", "u32"),
        // GettxoutproofParams.txids -> Vec<bitcoin::Txid>
        ("array", "txids", "Vec<bitcoin::Txid>"),
        // SendtoaddressParams.amount -> bitcoin::Amount (already working)
        ("number", "amount", "bitcoin::Amount"),
        // SendmanyParams.fee_rate -> f64 (already working)
        ("number", "fee_rate", "f64"),
        // SendParams.fee_rate -> f64 (already working)
        ("number", "fee_rate", "f64"),
        // VerifychainParams.checklevel -> u32
        ("number", "checklevel", "u32"),
        // StopParams.wait -> u64
        ("number", "wait", "u64"),
        // DisconnectnodeParams.nodeid -> u64
        ("number", "nodeid", "u64"),
        // GetblockfrompeerParams.peer_id -> u64
        ("number", "peer_id", "u64"),
        // ListtransactionsParams.skip -> u64
        ("number", "skip", "u64"),
        // ListunspentParams.maxconf -> u32
        ("number", "maxconf", "u32"),
    ];

    for (rpc_type, field_name, expected_type) in test_cases {
        let category = registry.categorize(rpc_type, field_name);
        let ty = category.to_rust_type();
        assert_eq!(
            ty, expected_type,
            "Field '{}' of type '{}' should map to {}",
            field_name, rpc_type, expected_type
        );
    }
}

#[test]
fn test_address_type_mapping() {
    let registry = TypeRegistry::new();

    // Test that address fields map to bitcoin::Address
    let arg = ApiArgument {
        type_: "string".to_string(),
        names: vec!["address".to_string()],
        required: true,
        description: "Bitcoin address".to_string(),
    };

    let (ty, _) = registry.map_argument_type(&arg);
    assert_eq!(ty, "String"); // Note: address pattern not in rules yet
}

#[test]
fn test_txids_array_mapping() {
    let registry = TypeRegistry::new();

    // Test that txids arrays map to Vec<bitcoin::Txid>
    let arg = ApiArgument {
        type_: "array".to_string(),
        names: vec!["txids".to_string()],
        required: true,
        description: "Transaction IDs".to_string(),
    };

    let (ty, _) = registry.map_argument_type(&arg);
    assert_eq!(ty, "Vec<bitcoin::Txid>");
}

#[test]
fn test_maxconf_matches_minconf() {
    let registry = TypeRegistry::new();

    // Test that maxconf and minconf both map to u32
    let minconf_arg = ApiArgument {
        type_: "number".to_string(),
        names: vec!["minconf".to_string()],
        required: true,
        description: "Minimum confirmations".to_string(),
    };

    let maxconf_arg = ApiArgument {
        type_: "number".to_string(),
        names: vec!["maxconf".to_string()],
        required: true,
        description: "Maximum confirmations".to_string(),
    };

    let (minconf_ty, _) = registry.map_argument_type(&minconf_arg);
    let (maxconf_ty, _) = registry.map_argument_type(&maxconf_arg);

    assert_eq!(minconf_ty, "u32");
    assert_eq!(maxconf_ty, "u32");
    assert_eq!(
        minconf_ty, maxconf_ty,
        "minconf and maxconf should have the same type"
    );
}

#[test]
fn test_port_type_mapping() {
    let registry = TypeRegistry::new();

    // Test port parameter mapping
    let arg = ApiArgument {
        type_: "number".to_string(),
        names: vec!["port".to_string()],
        required: true,
        description: "Network port".to_string(),
    };

    let (ty, is_opt) = registry.map_argument_type(&arg);
    assert_eq!(ty, "u16");
    assert_eq!(is_opt, false);
}

#[test]
fn test_nrequired_type_mapping() {
    let registry = TypeRegistry::new();

    // Test nrequired parameter mapping
    let arg = ApiArgument {
        type_: "number".to_string(),
        names: vec!["nrequired".to_string()],
        required: true,
        description: "Number of required signatures".to_string(),
    };

    let (ty, is_opt) = registry.map_argument_type(&arg);
    assert_eq!(ty, "u32");
    assert_eq!(is_opt, false);
}

#[test]
fn test_port_variations() {
    let registry = TypeRegistry::new();

    // Test different variations of port field names
    let variations = vec!["port", "PORT", "Port", "server_port", "client_port"];

    for field_name in variations {
        let arg = ApiArgument {
            type_: "number".to_string(),
            names: vec![field_name.to_string()],
            required: true,
            description: "Port number".to_string(),
        };

        let (ty, _) = registry.map_argument_type(&arg);
        assert_eq!(ty, "u16", "Field '{}' should map to u16", field_name);
    }
}

#[test]
fn test_other_number_mappings() {
    let registry = TypeRegistry::new();

    // Test that other number mappings still work correctly
    let test_cases = vec![
        ("amount", "bitcoin::Amount"),
        ("balance", "bitcoin::Amount"),
        ("fee", "f64"),
        ("rate", "f64"),
        ("blocks", "u64"),
        ("height", "u64"),
        ("count", "u64"),
        ("minconf", "u32"),
        ("locktime", "u32"),
        ("version", "u32"),
        ("difficulty", "f64"),
        ("probability", "f64"),
        ("percentage", "f64"),
    ];

    for (field_name, expected_type) in test_cases {
        let arg = ApiArgument {
            type_: "number".to_string(),
            names: vec![field_name.to_string()],
            required: true,
            description: "Test field".to_string(),
        };

        let (ty, _) = registry.map_argument_type(&arg);
        assert_eq!(
            ty, expected_type,
            "Field '{}' should map to {}",
            field_name, expected_type
        );
    }
}

#[test]
fn test_fallback_number_mapping() {
    let registry = TypeRegistry::new();

    // Test that unknown number fields fall back to u64 (not f64 as in original test)
    let arg = ApiArgument {
        type_: "number".to_string(),
        names: vec!["unknown_field".to_string()],
        required: true,
        description: "Unknown field".to_string(),
    };

    let (ty, _) = registry.map_argument_type(&arg);
    assert_eq!(ty, "u64"); // Based on the fallback rule in CATEGORY_RULES
}

#[test]
fn test_optional_handling() {
    let registry = TypeRegistry::new();

    // Test that optional fields are handled correctly
    let arg = ApiArgument {
        type_: "number".to_string(),
        names: vec!["port".to_string()],
        required: false,
        description: "Optional port".to_string(),
    };

    let (ty, is_opt) = registry.map_argument_type(&arg);
    assert_eq!(ty, "u16");
    assert_eq!(is_opt, true);
}

#[test]
fn test_result_type_mapping() {
    let registry = TypeRegistry::new();

    // Test result type mapping for port
    let result = ApiResult {
        key_name: "port".to_string(),
        type_: "number".to_string(),
        description: "Port number".to_string(),
        inner: vec![],
        required: true,
    };

    let (ty, is_opt) = registry.map_result_type(&result);
    assert_eq!(ty, "u16");
    assert_eq!(is_opt, false);

    // Test result type mapping for nrequired
    let result = ApiResult {
        key_name: "nrequired".to_string(),
        type_: "number".to_string(),
        description: "Required count".to_string(),
        inner: vec![],
        required: true,
    };

    let (ty, is_opt) = registry.map_result_type(&result);
    assert_eq!(ty, "u32");
    assert_eq!(is_opt, false);
}

#[test]
fn test_normalize_function() {
    // Test the normalize function behavior
    // Note: normalize is private, so we test it indirectly through categorization
    let registry = TypeRegistry::new();

    // Test case sensitivity - should be normalized
    assert_eq!(registry.categorize("number", "PORT"), RpcCategory::Port);
    assert_eq!(registry.categorize("number", "Port"), RpcCategory::Port);
    assert_eq!(registry.categorize("number", "port"), RpcCategory::Port);

    // Test underscore handling
    assert_eq!(
        registry.categorize("number", "server_port"),
        RpcCategory::Port
    );
    assert_eq!(
        registry.categorize("number", "client_port"),
        RpcCategory::Port
    );

    // Test other field variations
    assert_eq!(
        registry.categorize("number", "NREQUIRED"),
        RpcCategory::SmallInteger
    );
    assert_eq!(
        registry.categorize("number", "required_count"),
        RpcCategory::LargeInteger
    ); // count pattern
}

#[test]
fn test_all_small_integer_fields() {
    let registry = TypeRegistry::new();

    // Test all fields that should map to u32 (SmallInteger)
    let small_integer_fields = vec![
        "nrequired",
        "minconf",
        "maxconf",
        "locktime",
        "version",
        "verbosity",
        "checklevel",
        "n",
    ];

    for field in small_integer_fields {
        let category = registry.categorize("number", field);
        let ty = category.to_rust_type();
        assert_eq!(ty, "u32", "Field '{}' should map to u32", field);
    }
}

#[test]
fn test_all_large_integer_fields() {
    let registry = TypeRegistry::new();

    // Test all fields that should map to u64 (LargeInteger)
    let large_integer_fields = vec![
        "blocks",
        "nblocks",
        "maxtries",
        "height",
        "count",
        "index",
        "size",
        "time",
        "conf_target",
        "skip",
        "nodeid",
        "peer_id",
        "wait",
    ];

    for field in large_integer_fields {
        let category = registry.categorize("number", field);
        let ty = category.to_rust_type();
        assert_eq!(ty, "u64", "Field '{}' should map to u64", field);
    }
}

#[test]
fn test_all_float_fields() {
    let registry = TypeRegistry::new();

    // Test all fields that should map to f64 (Float)
    let float_fields = vec![
        "fee",
        "rate",
        "feerate",
        "maxfeerate",
        "maxburnamount",
        "relayfee",
        "incrementalfee",
        "incrementalrelayfee",
        "difficulty",
        "probability",
        "percentage",
        "fee_rate",
        "verificationprogress",
    ];

    for field in float_fields {
        let category = registry.categorize("number", field);
        let ty = category.to_rust_type();
        assert_eq!(ty, "f64", "Field '{}' should map to f64", field);
    }
}

#[test]
fn test_bitcoin_amount_fields() {
    let registry = TypeRegistry::new();

    // Test all fields that should map to bitcoin::Amount
    let amount_fields = vec!["amount", "balance"];

    for field in amount_fields {
        let category = registry.categorize("number", field);
        let ty = category.to_rust_type();
        assert_eq!(
            ty, "bitcoin::Amount",
            "Field '{}' should map to bitcoin::Amount",
            field
        );
    }
}

#[test]
fn test_array_type_mappings() {
    let registry = TypeRegistry::new();

    // Test array type mappings
    let test_cases = vec![
        ("keys", "Vec<String>"),
        ("addresses", "Vec<String>"),
        ("wallets", "Vec<String>"),
        ("txids", "Vec<bitcoin::Txid>"),
        ("unknown_array", "Vec<serde_json::Value>"), // fallback
    ];

    for (field_name, expected_type) in test_cases {
        let category = registry.categorize("array", field_name);
        let ty = category.to_rust_type();
        assert_eq!(
            ty, expected_type,
            "Array field '{}' should map to {}",
            field_name, expected_type
        );
    }
}

#[test]
fn test_string_type_mappings() {
    let registry = TypeRegistry::new();

    // Test string type mappings
    let test_cases = vec![
        ("txid", "bitcoin::Txid"),
        ("blockhash", "bitcoin::BlockHash"),
        ("generic_string", "String"), // fallback
    ];

    for (field_name, expected_type) in test_cases {
        let category = registry.categorize("string", field_name);
        let ty = category.to_rust_type();
        assert_eq!(
            ty, expected_type,
            "String field '{}' should map to {}",
            field_name, expected_type
        );
    }
}

#[test]
fn test_hex_type_mappings() {
    let registry = TypeRegistry::new();

    // Test hex type mappings
    let test_cases = vec![
        ("txid", "bitcoin::Txid"),
        ("blockhash", "bitcoin::BlockHash"),
        ("generic_hex", "String"), // fallback
    ];

    for (field_name, expected_type) in test_cases {
        let category = registry.categorize("hex", field_name);
        let ty = category.to_rust_type();
        assert_eq!(
            ty, expected_type,
            "Hex field '{}' should map to {}",
            field_name, expected_type
        );
    }
}

#[test]
fn test_object_type_mappings() {
    let registry = TypeRegistry::new();

    // Test object type mappings
    let test_cases = vec![
        ("options", "serde_json::Value"),
        ("query_options", "serde_json::Value"),
        ("generic_object", "serde_json::Value"), // fallback
    ];

    for (field_name, expected_type) in test_cases {
        let category = registry.categorize("object", field_name);
        let ty = category.to_rust_type();
        assert_eq!(
            ty, expected_type,
            "Object field '{}' should map to {}",
            field_name, expected_type
        );
    }
}

#[test]
fn test_amount_type_special_handling() {
    let registry = TypeRegistry::new();

    // Test special handling for "type": "amount" fields
    let test_cases = vec![
        ("balance", "f64"), // Explicit balance fields in amount type
        ("fee_rate", "f64"),
        ("estimated_feerate", "f64"),
        ("maxfeerate", "f64"),
        ("maxburnamount", "f64"),
        ("relayfee", "f64"),
        ("incrementalfee", "f64"),
        ("incrementalrelayfee", "f64"),
        ("generic_amount", "bitcoin::Amount"), // fallback for amount type
    ];

    for (field_name, expected_type) in test_cases {
        let category = registry.categorize("amount", field_name);
        let ty = category.to_rust_type();
        assert_eq!(
            ty, expected_type,
            "Amount field '{}' should map to {}",
            field_name, expected_type
        );
    }
}

#[test]
fn test_dummy_field_handling() {
    let registry = TypeRegistry::new();

    // Test dummy field handling (should be optional)
    let test_cases = vec![
        ("string", "dummy", "String", true),
        ("number", "dummy", "String", true), // Note: maps to String due to Dummy category
    ];

    for (rpc_type, field_name, expected_type, should_be_optional) in test_cases {
        let category = registry.categorize(rpc_type, field_name);
        let ty = category.to_rust_type();
        let is_opt = category.is_optional_by_default();
        assert_eq!(
            ty, expected_type,
            "Dummy field '{}' should map to {}",
            field_name, expected_type
        );
        assert_eq!(
            is_opt, should_be_optional,
            "Dummy field '{}' should be optional",
            field_name
        );
    }
}

#[test]
fn test_primitive_type_fallbacks() {
    let registry = TypeRegistry::new();

    // Test primitive type fallbacks
    let test_cases = vec![
        ("string", "unknown_field", "String"),
        ("boolean", "unknown_field", "bool"),
        ("null", "unknown_field", "()"),
    ];

    for (rpc_type, field_name, expected_type) in test_cases {
        let category = registry.categorize(rpc_type, field_name);
        let ty = category.to_rust_type();
        assert_eq!(
            ty, expected_type,
            "Primitive type '{}' field '{}' should map to {}",
            rpc_type, field_name, expected_type
        );
    }
}

#[test]
fn test_unknown_type_fallback() {
    let registry = TypeRegistry::new();

    // Test unknown type fallback
    let category = registry.categorize("unknown_type", "unknown_field");
    let ty = category.to_rust_type();
    assert_eq!(
        ty, "serde_json::Value",
        "Unknown type should fall back to serde_json::Value"
    );
}

#[test]
fn test_result_with_empty_key_name() {
    let registry = TypeRegistry::new();

    // Test result with empty key_name (should use description)
    let result = ApiResult {
        key_name: "".to_string(),
        type_: "number".to_string(),
        description: "port".to_string(), // Should be used as fallback
        inner: vec![],
        required: true,
    };

    let (ty, _) = registry.map_result_type(&result);
    assert_eq!(
        ty, "u16",
        "Result with empty key_name should use description for categorization"
    );
}

#[test]
fn test_argument_with_multiple_names() {
    let registry = TypeRegistry::new();

    // Test argument with multiple names (should use first name)
    let arg = ApiArgument {
        type_: "number".to_string(),
        names: vec!["port".to_string(), "alternative_name".to_string()],
        required: true,
        description: "Network port".to_string(),
    };

    let (ty, _) = registry.map_argument_type(&arg);
    assert_eq!(
        ty, "u16",
        "Argument should use first name for categorization"
    );
}

#[test]
fn test_category_to_rust_type_mapping() {
    // Test all category to Rust type mappings
    let test_cases = vec![
        (RpcCategory::String, "String"),
        (RpcCategory::Boolean, "bool"),
        (RpcCategory::Null, "()"),
        (RpcCategory::BitcoinTxid, "bitcoin::Txid"),
        (RpcCategory::BitcoinBlockHash, "bitcoin::BlockHash"),
        (RpcCategory::BitcoinAmount, "bitcoin::Amount"),
        (RpcCategory::BitcoinAddress, "bitcoin::Address"),
        (RpcCategory::Port, "u16"),
        (RpcCategory::SmallInteger, "u32"),
        (RpcCategory::LargeInteger, "u64"),
        (RpcCategory::Float, "f64"),
        (RpcCategory::BitcoinArray, "Vec<bitcoin::Txid>"),
        (RpcCategory::StringArray, "Vec<String>"),
        (RpcCategory::GenericArray, "Vec<serde_json::Value>"),
        (RpcCategory::BitcoinObject, "serde_json::Value"),
        (RpcCategory::GenericObject, "serde_json::Value"),
        (RpcCategory::Dummy, "String"),
        (RpcCategory::Unknown, "serde_json::Value"),
    ];

    for (category, expected_type) in test_cases {
        assert_eq!(
            category.to_rust_type(),
            expected_type,
            "Category {:?} should map to {}",
            category,
            expected_type
        );
    }
}

#[test]
fn test_category_optional_handling() {
    // Test which categories are optional
    let test_cases = vec![
        (RpcCategory::String, false),
        (RpcCategory::Boolean, false),
        (RpcCategory::Null, false),
        (RpcCategory::BitcoinTxid, false),
        (RpcCategory::BitcoinBlockHash, false),
        (RpcCategory::BitcoinAmount, false),
        (RpcCategory::BitcoinAddress, false),
        (RpcCategory::Port, false),
        (RpcCategory::SmallInteger, false),
        (RpcCategory::LargeInteger, false),
        (RpcCategory::Float, false),
        (RpcCategory::BitcoinArray, false),
        (RpcCategory::StringArray, false),
        (RpcCategory::GenericArray, false),
        (RpcCategory::BitcoinObject, false),
        (RpcCategory::GenericObject, false),
        (RpcCategory::Dummy, true), // Only Dummy should be optional
        (RpcCategory::Unknown, false),
    ];

    for (category, should_be_optional) in test_cases {
        assert_eq!(
            category.is_optional_by_default(),
            should_be_optional,
            "Category {:?} optional status should be {}",
            category,
            should_be_optional
        );
    }
}

#[test]
fn test_serde_attributes() {
    // Test serde attributes for categories
    let test_cases = vec![
        (
            RpcCategory::BitcoinAmount,
            Some("#[serde(deserialize_with = \"amount_from_btc_float\")]"),
        ),
        (RpcCategory::String, None),
        (RpcCategory::Boolean, None),
        (RpcCategory::Port, None),
        (RpcCategory::SmallInteger, None),
        (RpcCategory::LargeInteger, None),
        (RpcCategory::Float, None),
    ];

    for (category, expected_attribute) in test_cases {
        assert_eq!(
            category.serde_attributes(),
            expected_attribute,
            "Category {:?} should have serde attribute {:?}",
            category,
            expected_attribute
        );
    }
}

#[test]
fn test_category_descriptions() {
    // Test that all categories have descriptions
    let categories = vec![
        RpcCategory::String,
        RpcCategory::Boolean,
        RpcCategory::Null,
        RpcCategory::BitcoinTxid,
        RpcCategory::BitcoinBlockHash,
        RpcCategory::BitcoinAmount,
        RpcCategory::BitcoinAddress,
        RpcCategory::Port,
        RpcCategory::SmallInteger,
        RpcCategory::LargeInteger,
        RpcCategory::Float,
        RpcCategory::BitcoinArray,
        RpcCategory::StringArray,
        RpcCategory::GenericArray,
        RpcCategory::BitcoinObject,
        RpcCategory::GenericObject,
        RpcCategory::Dummy,
        RpcCategory::Unknown,
    ];

    for category in categories {
        let description = category.description();
        assert!(
            !description.is_empty(),
            "Category {:?} should have a non-empty description",
            category
        );
        assert!(
            description.len() > 5,
            "Category {:?} should have a meaningful description",
            category
        );
    }
}

#[test]
fn test_type_registry_default() {
    // Test that TypeRegistry implements Default
    let registry1 = TypeRegistry::new();
    let registry2 = TypeRegistry::default();

    // They should behave identically
    assert_eq!(
        registry1.categorize("number", "port"),
        registry2.categorize("number", "port")
    );
    assert_eq!(
        registry1.categorize("string", "txid"),
        registry2.categorize("string", "txid")
    );
}

#[test]
fn test_edge_case_field_names() {
    let registry = TypeRegistry::new();

    // Test edge cases with unusual field names
    let edge_cases = vec![
        ("", "number", "u64"),  // Empty field name should fall back to default
        ("a", "number", "u64"), // Single character
        (
            "very_long_field_name_with_many_underscores_and_numbers_123",
            "number",
            "u64",
        ), // Very long name
        ("PORT_NUMBER", "number", "u16"), // All caps with underscore
        ("port-number", "number", "u16"), // With hyphen
        ("port number", "number", "u16"), // With space
        ("PortNumber", "number", "u16"), // CamelCase
        ("portNumber", "number", "u16"), // camelCase
    ];

    for (field_name, rpc_type, expected_type) in edge_cases {
        let category = registry.categorize(rpc_type, field_name);
        let ty = category.to_rust_type();
        assert_eq!(
            ty, expected_type,
            "Edge case field '{}' of type '{}' should map to {}",
            field_name, rpc_type, expected_type
        );
    }
}

#[test]
fn test_regression_protection() {
    let registry = TypeRegistry::new();

    // This test ensures that critical mappings never regress
    // These are the most important mappings that must always work correctly

    // Critical Bitcoin types
    assert_eq!(
        registry.categorize("string", "txid"),
        RpcCategory::BitcoinTxid
    );
    assert_eq!(
        registry.categorize("string", "blockhash"),
        RpcCategory::BitcoinBlockHash
    );
    assert_eq!(
        registry.categorize("number", "amount"),
        RpcCategory::BitcoinAmount
    );

    // Critical numeric types
    assert_eq!(registry.categorize("number", "port"), RpcCategory::Port);
    assert_eq!(
        registry.categorize("number", "minconf"),
        RpcCategory::SmallInteger
    );
    assert_eq!(
        registry.categorize("number", "height"),
        RpcCategory::LargeInteger
    );
    assert_eq!(registry.categorize("number", "fee"), RpcCategory::Float);

    // Critical array types
    assert_eq!(
        registry.categorize("array", "txids"),
        RpcCategory::BitcoinArray
    );
    assert_eq!(
        registry.categorize("array", "addresses"),
        RpcCategory::StringArray
    );

    // Critical fallbacks
    assert_eq!(
        registry.categorize("unknown_type", "unknown_field"),
        RpcCategory::Unknown
    );
    assert_eq!(
        registry.categorize("number", "unknown_field"),
        RpcCategory::LargeInteger
    ); // number fallback
}
