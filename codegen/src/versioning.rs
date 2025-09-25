//! Version handling utilities for Bitcoin Core RPC code generation.
//!
//! Provides a `Version` type with helpers for formatting and comparison.
use thiserror::Error;

/// Midas build number component of the generated crate version (e.g., 29.1.{BUILD_VERSION}).
pub const BUILD_VERSION: u32 = 0; // TODO: bump this

/// Bitcoin Core version representation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u32,
    minor: u32,

    version_string: String,
}

impl Version {
    /// Construct a new Version from `major.minor`.
    pub fn new(major: u32, minor: u32) -> Self {
        let version_string =
            if minor == 0 { format!("v{}", major) } else { format!("v{}.{}", major, minor) };
        Self { major, minor, version_string }
    }

    /// Parse strings like "v29.1" or "29.1" into a Version.
    pub fn from_string(s: &str) -> Result<Self, VersionError> {
        let s = s.trim_start_matches('v');
        let parts: Vec<&str> = s.split('.').collect();

        if parts.is_empty() || parts.len() > 2 {
            return Err(VersionError::ParseError(format!("Invalid version format: '{}'", s)));
        }

        let major = parts[0]
            .parse::<u32>()
            .map_err(|_| VersionError::ParseError(format!("Invalid major '{}'", parts[0])))?;

        let minor = if parts.len() > 1 {
            parts[1]
                .parse::<u32>()
                .map_err(|_| VersionError::ParseError(format!("Invalid minor '{}'", parts[1])))?
        } else {
            0
        };

        Ok(Self::new(major, minor))
    }

    /// Return the major component.
    pub fn major(&self) -> u32 { self.major }
    /// Return the minor component.
    pub fn minor(&self) -> u32 { self.minor }

    /// Return the pretty string form (e.g., "v29.1").
    pub fn as_str(&self) -> &str { &self.version_string }
    /// Return the module name form (e.g., "v29_1").
    pub fn as_module_name(&self) -> String {
        if self.minor == 0 {
            format!("v{}", self.major)
        } else {
            format!("v{}_{}", self.major, self.minor)
        }
    }
    /// Return the documentation label (e.g., "29.1").
    pub fn as_doc_version(&self) -> String {
        if self.minor == 0 {
            format!("{}", self.major)
        } else {
            format!("{}.{}", self.major, self.minor)
        }
    }
    /// Return a numeric-ish form without the leading 'v' (e.g., "29_1").
    pub fn as_number(&self) -> String {
        if self.minor == 0 {
            format!("{}", self.major)
        } else {
            format!("{}_{}", self.major, self.minor)
        }
    }

    /// Compose crate version (e.g., "29.1.1") with midas build number
    pub fn crate_version(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, BUILD_VERSION)
    }
}

/// Errors related to parsing or handling Version values.
#[derive(Error, Debug)]
pub enum VersionError {
    /// Parsing failed for the provided version string.
    #[error("Failed to parse version: {0}")]
    ParseError(String),
}
