use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// Represents a Bitcoin Core RPC version.
/// Includes both explicitly supported versions and runtime-discovered ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    V28,
    V29,
}

pub const KNOWN: &[Version] = &[Version::V28, Version::V29];

// Default fallback if no compile-time version is obtained
pub const DEFAULT_VERSION: Version = Version::V28;

impl Version {
    /// Convert the version into its numeric representation.
    pub fn as_number(&self) -> u32 {
        match self {
            Version::V28 => 28,
            Version::V29 => 29,
        }
    }

    /// Convert the version into its string representation like "V29".
    pub fn as_str(&self) -> String {
        format!("V{}", self.as_number())
    }

    /// Convert the version into its lowercase string representation like "v29".
    /// Used specifically for types module names to follow Rust naming conventions.
    pub fn as_str_lowercase(&self) -> String {
        format!("v{}", self.as_number())
    }

    /// Whether the version is one of the explicitly supported ones.
    pub fn is_known(&self) -> bool {
        KNOWN.contains(self)
    }

    /// Whether the version is currently supported (including runtime).
    pub fn is_supported(&self) -> bool {
        self.as_number() >= 28
    }

    /// Converts a number to a Version, mapping known versions to their enum form.
    pub fn from_number(n: u32) -> Self {
        KNOWN
            .iter()
            .find(|v| v.as_number() == n)
            .copied()
            .unwrap_or(DEFAULT_VERSION)
    }

    /// Normalize known Runtime values to their canonical enum form.
    pub fn normalize(&self) -> Version {
        Version::from_number(self.as_number())
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "V{}", self.as_number())
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Accept both uppercase 'V' and lowercase 'v' prefixes
        let num = if let Some(num) = s.strip_prefix('V') {
            num
        } else if let Some(num) = s.strip_prefix('v') {
            num
        } else {
            return Err(VersionError::InvalidFormat(s.to_string()));
        };

        num.parse::<u32>()
            .map(Version::from_number)
            .map_err(|_| VersionError::InvalidFormat(s.to_string()))
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        Version::from_str(s).unwrap_or(DEFAULT_VERSION) // Use the default version if parsing fails
    }
}

#[derive(Error, Debug)]
pub enum VersionError {
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(String),
    #[error("Invalid version format: {0}")]
    InvalidFormat(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_number() {
        assert_eq!(Version::V28.as_number(), 28);
        assert_eq!(Version::V29.as_number(), 29);
    }

    #[test]
    fn test_is_supported() {
        assert!(Version::V28.is_supported());
        assert!(Version::V29.is_supported());
        // Test edge case - from_number(27) returns V28 (DEFAULT_VERSION), which IS supported
        let old_version = Version::from_number(27);
        assert!(old_version.is_supported()); // This is V28, which is supported
    }

    #[test]
    fn test_as_str_lowercase() {
        assert_eq!(Version::V28.as_str_lowercase(), "v28");
        assert_eq!(Version::V29.as_str_lowercase(), "v29");
    }

    #[test]
    fn test_from_number() {
        assert_eq!(Version::from_number(28), Version::V28);
        assert_eq!(Version::from_number(29), Version::V29);
        // Test fallback to default
        assert_eq!(Version::from_number(999), Version::V28); // DEFAULT_VERSION
        assert_eq!(Version::from_number(27), Version::V28); // DEFAULT_VERSION
    }

    #[test]
    fn test_normalize() {
        assert_eq!(Version::V28.normalize(), Version::V28);
        assert_eq!(Version::V29.normalize(), Version::V29);
    }

    #[test]
    fn test_version_roundtrip() {
        for version in KNOWN {
            let num = version.as_number();
            let from_num = Version::from_number(num);
            assert_eq!(*version, from_num);
        }
    }

    #[test]
    fn test_version_string_representations() {
        assert_eq!(Version::V28.as_str(), "V28");
        assert_eq!(Version::V29.as_str(), "V29");
        assert_eq!(Version::V28.as_str_lowercase(), "v28");
        assert_eq!(Version::V29.as_str_lowercase(), "v29");
    }

    #[test]
    fn test_version_support_logic() {
        // Known versions should be supported
        assert!(Version::V28.is_supported());
        assert!(Version::V29.is_supported());

        // Test boundary condition
        let boundary_version = Version::from_number(28);
        assert!(boundary_version.is_supported());

        // Test that from_number(27) returns V28 (DEFAULT_VERSION), which IS supported
        let old_version = Version::from_number(27);
        assert!(old_version.is_supported()); // This is V28, which is supported
    }

    #[test]
    fn test_version_known_logic() {
        // Known versions should be known
        assert!(Version::V28.is_known());
        assert!(Version::V29.is_known());

        // Test with unknown version - from_number(999) returns V28 (DEFAULT_VERSION), which IS known
        let unknown_version = Version::from_number(999);
        assert!(unknown_version.is_known()); // This is V28, which is known
    }

    #[test]
    fn test_from_str_edge_cases() {
        // Test that invalid strings fall back to DEFAULT_VERSION
        let invalid_version = Version::from("invalid");
        assert_eq!(invalid_version, Version::V28);

        // Test that valid but unknown numbers fall back to DEFAULT_VERSION
        let unknown_version = Version::from("v999");
        assert_eq!(unknown_version, Version::V28);
    }

    #[test]
    fn test_display_implementation() {
        assert_eq!(Version::V28.to_string(), "V28");
        assert_eq!(Version::V29.to_string(), "V29");
    }
}
