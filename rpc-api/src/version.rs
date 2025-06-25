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

// This becomes the single source of truth for supported versions
pub const KNOWN: &[Version] = &[Version::V28, Version::V29];

pub const DEFAULT_VERSION: Version = Version::V29;

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

    /// Convert the version into its lowercase string representation like "v28".
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
        if let Some(num) = s.strip_prefix('V') {
            num.parse::<u32>()
                .map(Version::from_number)
                .map_err(|_| VersionError::InvalidFormat(s.to_string()))
        } else {
            Err(VersionError::InvalidFormat(s.to_string()))
        }
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

/// Get the Bitcoin Core version this crate was compiled against
pub fn compiled_version() -> &'static str {
    option_env!("BITCOIN_CORE_VERSION")
        .expect("BITCOIN_CORE_VERSION not set — ensure build.rs ran and api.json was valid")
}

/// Get the version as a Version enum
pub fn compiled_version_enum() -> Version {
    Version::from(compiled_version())
}

/// Check if the current compiled version matches the given version
pub fn is_compiled_version(version: Version) -> bool {
    compiled_version_enum() == version
}

/// Get all supported versions as a slice
pub fn supported_versions() -> &'static [Version] {
    KNOWN
}
