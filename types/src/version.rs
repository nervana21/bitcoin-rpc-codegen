//! Version handling for Bitcoin Core RPC API.

use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// Represents a Bitcoin Core version with major and minor components
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub version_string: String,
}

impl Version {
    /// Create a new Version from a version string like "v29.1" or "v29"
    pub fn from_string(version_str: &str) -> Result<Self, VersionError> {
        let version_clean = version_str.trim_start_matches('v').trim_start_matches('V');
        let parts: Vec<&str> = version_clean.split('.').collect();
        
        if parts.is_empty() || parts.len() > 2 {
            return Err(VersionError::InvalidFormat(version_str.to_string()));
        }
        
        let major = parts[0].parse::<u32>()?;
        let minor = if parts.len() == 2 {
            parts[1].parse::<u32>()?
        } else {
            0
        };
        
        Ok(Version {
            major,
            minor,
            version_string: version_str.to_string(),
        })
    }
    
    /// Get the version as a string (e.g., "v29.1")
    pub fn as_str(&self) -> &str {
        &self.version_string
    }
    
    /// Get the major version number
    pub fn major(&self) -> u32 {
        self.major
    }
    
    /// Get the minor version number
    pub fn minor(&self) -> u32 {
        self.minor
    }
    
    /// Get the version as a number (major version for compatibility)
    pub fn as_number(&self) -> u32 {
        self.major
    }
    
    /// Get the version as a lowercase string
    pub fn as_str_lowercase(&self) -> String {
        self.version_string.to_lowercase()
    }

    /// Get the version as a valid Rust module name (replaces dots with underscores)
    pub fn as_module_name(&self) -> String {
        self.version_string.to_lowercase().replace('.', "_")
    }

    /// Get the version string for documentation (v{major}.{minor})
    pub fn as_doc_version(&self) -> String {
        self.version_string.clone()
    }

    /// Generates a version string that mirrors Bitcoin Core's versioning scheme.
    /// This is used to generate the version string for the crate as published on crates.io.
    pub fn crate_version(&self) -> String {
        let midas_version = 0;
        format!("{}.{}.{}", self.major, self.minor, midas_version)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::from_string("v29.1").expect("Default version should be valid")
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version_string)
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        s.parse().unwrap_or_else(|_| panic!("Invalid version string: '{}'. Supported versions: v28, v29, v29.1", s))
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}

/// Error type for version parsing
#[derive(Debug, Error)]
pub enum VersionError {
    #[error("Invalid version string: {0}")]
    InvalidFormat(String),
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(u32),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        assert_eq!(Version::from_string("v28").unwrap().major(), 28);
        assert_eq!(Version::from_string("v28").unwrap().minor(), 0);
        assert_eq!(Version::from_string("v29.1").unwrap().major(), 29);
        assert_eq!(Version::from_string("v29.1").unwrap().minor(), 1);
    }

    #[test]
    fn test_version_display() {
        assert_eq!(Version::from_string("v29.1").unwrap().as_str(), "v29.1");
        assert_eq!(Version::from_string("v28").unwrap().as_str(), "v28");
    }

    #[test]
    fn test_version_compatibility() {
        let v29_1 = Version::from_string("v29.1").unwrap();
        assert_eq!(v29_1.as_number(), 29);
        assert_eq!(v29_1.as_str_lowercase(), "v29.1");
    }

    #[test]
    fn test_module_name() {
        assert_eq!(Version::from_string("v29.1").unwrap().as_module_name(), "v29_1");
        assert_eq!(Version::from_string("v28").unwrap().as_module_name(), "v28");
    }
}
