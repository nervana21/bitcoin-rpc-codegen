//! Core schema types for Bitcoin RPC API definitions
//!
//! This module defines the fundamental types used to represent Bitcoin RPC
//! method definitions, arguments, and results. These types are designed to
//! be serializable and provide a clean abstraction over the raw JSON schema.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Bitcoin method argument specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtcArgument {
    /// Names this argument can be called by
    pub names: Vec<String>,
    /// Human-readable description of this argument
    pub description: String,
    /// One-line description for concise display
    #[serde(default, rename = "oneline_description")]
    pub oneline_description: String,
    /// Whether this argument can also be passed positionally
    #[serde(default, rename = "also_positional")]
    pub also_positional: bool,
    /// Type string representations
    #[serde(default, rename = "type_str")]
    pub type_str: Option<Vec<String>>,
    /// Whether this argument is required
    pub required: bool,
    /// Whether this argument is hidden from help
    #[serde(default)]
    pub hidden: bool,
    /// The type of this argument
    #[serde(rename = "type")]
    pub type_: String,
}

/// Bitcoin method result specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtcResult {
    /// The type of this result field
    #[serde(rename = "type")]
    pub type_: String,
    /// Whether this result is optional (from JSON)
    #[serde(default, rename = "optional")]
    pub optional: bool,
    /// Whether this result is required (computed from optional)
    #[serde(skip)]
    pub required: bool,
    /// Human-readable description of this result
    pub description: String,
    /// Whether to skip type checking for this result
    #[serde(default, rename = "skip_type_check")]
    pub skip_type_check: bool,
    /// Key name for this result field
    #[serde(default, rename = "key_name")]
    pub key_name: String,
    /// Condition for when this result is present
    #[serde(default)]
    pub condition: String,
    /// Nested result fields (for complex objects)
    #[serde(default)]
    pub inner: Vec<BtcResult>,
}

/// Bitcoin method definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtcMethod {
    /// The name of the Bitcoin method
    pub name: String,
    /// Human-readable description of what this method does
    pub description: String,
    /// Example usage strings
    #[serde(default)]
    pub examples: String,
    /// List of argument names for positional arguments
    #[serde(default, rename = "argument_names")]
    pub argument_names: Vec<String>,
    /// Detailed argument specifications
    pub arguments: Vec<BtcArgument>,
    /// Result specifications
    pub results: Vec<BtcResult>,
}

impl Default for BtcResult {
    fn default() -> Self {
        Self {
            type_: String::new(),
            optional: false,
            required: true,
            description: String::new(),
            skip_type_check: false,
            key_name: String::new(),
            condition: String::new(),
            inner: Vec::new(),
        }
    }
}

impl BtcResult {
    /// Create a new RpcResult with computed required field
    pub fn new(
        type_: String,
        optional: bool,
        description: String,
        skip_type_check: bool,
        key_name: String,
        condition: String,
        inner: Vec<BtcResult>,
    ) -> Self {
        Self {
            type_,
            optional,
            required: !optional,
            description,
            skip_type_check,
            key_name,
            condition,
            inner,
        }
    }

    /// Post-process after deserialization to compute required field
    pub fn post_process(&mut self) {
        self.required = !self.optional;
        for inner in &mut self.inner {
            inner.post_process();
        }
    }
}

/// Complete Bitcoin RPC API definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDefinition {
    /// Map of method names to their definitions
    pub rpcs: HashMap<String, BtcMethod>,
}

/// Error types for schema operations
#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("Failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Schema validation error: {0}")]
    Validation(String),

    #[error("Method not found: {0}")]
    MethodNotFound(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

/// Result type for schema operations
pub type Result<T> = std::result::Result<T, SchemaError>;

/// Bitcoin Core version representation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u32,
    minor: u32,
    version_string: String,
}

impl Version {
    /// Create a new version from major and minor numbers
    pub fn new(major: u32, minor: u32) -> Self { 
        let version_string = if minor == 0 {
            format!("v{}", major)
        } else {
            format!("v{}.{}", major, minor)
        };
        Self { major, minor, version_string }
    }

    /// Parse a version string like "v29.1" or "29.1"
    pub fn from_string(s: &str) -> Result<Self> {
        let s = s.trim_start_matches('v');
        let parts: Vec<&str> = s.split('.').collect();

        if parts.is_empty() || parts.len() > 2 {
            return Err(SchemaError::InvalidArgument(format!(
                "Invalid version format: '{}'. Expected format like 'v29.1' or '29.1'",
                s
            )));
        }

        let major = parts[0].parse::<u32>().map_err(|_| {
            SchemaError::InvalidArgument(format!("Invalid major version: '{}'", parts[0]))
        })?;

        let minor = if parts.len() > 1 {
            parts[1].parse::<u32>().map_err(|_| {
                SchemaError::InvalidArgument(format!("Invalid minor version: '{}'", parts[1]))
            })?
        } else {
            0
        };

        let version_string = if minor == 0 {
            format!("v{}", major)
        } else {
            format!("v{}.{}", major, minor)
        };
        Ok(Self { major, minor, version_string })
    }

    /// Get the major version number
    pub fn major(&self) -> u32 { self.major }

    /// Get the minor version number
    pub fn minor(&self) -> u32 { self.minor }

    /// Return version as string (e.g., "v29.1")
    pub fn as_str(&self) -> &str {
        &self.version_string
    }

    /// Return version as module name (e.g., "v29_1")
    pub fn as_module_name(&self) -> String {
        if self.minor == 0 {
            format!("v{}", self.major)
        } else {
            format!("v{}_{}", self.major, self.minor)
        }
    }

    /// Return version for documentation (e.g., "29.1")
    pub fn as_doc_version(&self) -> String {
        if self.minor == 0 {
            format!("{}", self.major)
        } else {
            format!("{}.{}", self.major, self.minor)
        }
    }

    /// Return version as number string (e.g., "29_1")
    pub fn as_number(&self) -> String {
        if self.minor == 0 {
            format!("{}", self.major)
        } else {
            format!("{}_{}", self.major, self.minor)
        }
    }

    /// Return version for crate version (e.g., "0.29.1")
    pub fn crate_version(&self) -> String { format!("0.{}.{}", self.major, self.minor) }

    /// Return the major version as u32
    pub fn as_major_version(&self) -> u32 { self.major }
}

impl Default for Version {
    fn default() -> Self { Self::new(29, 1) }
}

/// Version-related error type
#[derive(Error, Debug)]
pub enum VersionError {
    #[error("Failed to parse version: {0}")]
    ParseError(String),
}

impl From<SchemaError> for VersionError {
    fn from(err: SchemaError) -> Self { VersionError::ParseError(err.to_string()) }
}

impl ApiDefinition {
    /// Create a new empty API definition
    pub fn new() -> Self { Self { rpcs: HashMap::new() } }

    /// Load API definition from a JSON file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| SchemaError::Io(e))?;

        let mut api_def: ApiDefinition =
            serde_json::from_str(&content).map_err(|e| SchemaError::JsonParse(e))?;

        // Post-process to compute required fields
        for method in api_def.rpcs.values_mut() {
            for result in &mut method.results {
                result.post_process();
            }
        }

        // Validate the loaded definition
        api_def.validate()?;

        Ok(api_def)
    }

    /// Get a method by name
    pub fn get_method(&self, name: &str) -> Option<&BtcMethod> { self.rpcs.get(name) }

    /// Validate the API definition
    pub fn validate(&self) -> Result<()> {
        for (name, method) in &self.rpcs {
            if method.name != *name {
                return Err(SchemaError::Validation(format!(
                    "Method name mismatch: key '{}' vs method.name '{}'",
                    name, method.name
                )));
            }

            // Check for argument ordering issues and warn about them
            self.validate_argument_ordering(name, method)?;
        }

        Ok(())
    }

    /// Validate argument ordering and detect methods that need reordering
    fn validate_argument_ordering(&self, method_name: &str, method: &BtcMethod) -> Result<()> {
        let args = &method.arguments;
        if args.len() < 2 {
            return Ok(());
        }

        // Check if any required argument comes after an optional one
        for i in 0..args.len() - 1 {
            if !args[i].required && args[i + 1].required {
                eprintln!(
                    "Warning: Method '{}' has argument ordering issue: '{}' (optional) comes before '{}' (required). This will be automatically fixed during code generation.",
                    method_name,
                    args[i].names[0],
                    args[i + 1].names[0]
                );
                break; // Only warn once per method
            }
        }

        Ok(())
    }

}

impl Default for ApiDefinition {
    fn default() -> Self { Self::new() }
}

