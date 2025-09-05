//! Version handling for Bitcoin Core RPC API.

use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// Represents a Bitcoin Core version
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Version {
    /// Bitcoin Core v28
    V28,
    /// Bitcoin Core v29
    V29,
}

impl Version {
    /// Get the default version
    pub const fn default() -> Self {
        Self::V29
    }

    /// Get the version as a number
    pub fn as_number(&self) -> u32 {
        match self {
            Version::V28 => 28,
            Version::V29 => 29,
        }
    }

    /// Get the version as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Version::V28 => "V28",
            Version::V29 => "V29",
        }
    }

    /// Get the version as a lowercase string
    pub fn as_str_lowercase(&self) -> &'static str {
        match self {
            Version::V28 => "v28",
            Version::V29 => "v29",
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::default()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "v28" | "28" => Version::V28,
            "v29" | "29" => Version::V29,
            _ => Version::V29, // Default fallback
        }
    }
}

impl From<u32> for Version {
    fn from(n: u32) -> Self {
        match n {
            28 => Version::V28,
            29 => Version::V29,
            _ => Version::V29, // Default fallback
        }
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v28" | "28" => Ok(Version::V28),
            "v29" | "29" => Ok(Version::V29),
            _ => Err(VersionError::InvalidFormat(s.to_string())),
        }
    }
}

/// Error type for version parsing
#[derive(Debug, Error)]
pub enum VersionError {
    #[error("Invalid version string: {0}")]
    InvalidFormat(String),
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(u32),
}

/// The default version used throughout the codebase
pub const DEFAULT_VERSION: Version = Version::V29;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_from_str() {
        assert_eq!(Version::from("v28"), Version::V28);
        assert_eq!(Version::from("v29"), Version::V29);
        assert_eq!(Version::from("28"), Version::V28);
        assert_eq!(Version::from("29"), Version::V29);
        assert_eq!(Version::from("invalid"), Version::V29); // fallback
    }

    #[test]
    fn test_version_from_u32() {
        assert_eq!(Version::from(28), Version::V28);
        assert_eq!(Version::from(29), Version::V29);
        assert_eq!(Version::from(999), Version::V29); // fallback
    }

    #[test]
    fn test_version_display() {
        assert_eq!(Version::V28.to_string(), "V28");
        assert_eq!(Version::V29.to_string(), "V29");
    }

    #[test]
    fn test_version_as_number() {
        assert_eq!(Version::V28.as_number(), 28);
        assert_eq!(Version::V29.as_number(), 29);
    }

    #[test]
    fn test_version_as_str() {
        assert_eq!(Version::V28.as_str(), "V28");
        assert_eq!(Version::V29.as_str(), "V29");
    }

    #[test]
    fn test_version_ordering() {
        assert!(Version::V28 < Version::V29);
        assert!(Version::V29 > Version::V28);
        assert_eq!(Version::V28, Version::V28);
    }
}
