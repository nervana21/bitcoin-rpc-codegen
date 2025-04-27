// src/generator/versions.rs

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

/// Error parsing a `Version` from string or number.
#[derive(Debug)]
pub struct ParseVersionError(String);

impl fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ParseVersionError {}

/// Macro to define Version enum, the SUPPORTED slice,
/// Display, FromStr & TryFrom<u32> all in one shot.
macro_rules! define_versions {
    ( $( $name:ident = $num:expr ),* $(,)? ) => {
        #[repr(u32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Version {
            $( $name = $num, )*
        }

        impl Version {
            /// All supported versions.
            pub const SUPPORTED: &'static [Version] = &[
                $( Version::$name, )*
            ];

            /// Numeric major.
            pub fn as_number(self) -> u32 {
                self as u32
            }
        }

        impl fmt::Display for Version {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // prints "v17", "v18", etc.
                write!(f, "v{}", *self as u32)
            }
        }

        impl FromStr for Version {
            type Err = ParseVersionError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // expect "vNN"
                if let Some(rest) = s.strip_prefix('v') {
                    let n: u32 = rest.parse()
                        .map_err(|_| ParseVersionError(format!("invalid version '{}'", s)))?;
                    Version::try_from(n)
                } else {
                    Err(ParseVersionError(format!("invalid version '{}'", s)))
                }
            }
        }

        impl TryFrom<u32> for Version {
            type Error = ParseVersionError;
            fn try_from(n: u32) -> Result<Self, Self::Error> {
                match n {
                    $( $num => Ok(Version::$name), )*
                    other => Err(ParseVersionError(format!("unsupported major version: {}", other))),
                }
            }
        }
    };
}

define_versions! {
    V17 = 17,
    V18 = 18,
    V19 = 19,
    V20 = 20,
    V21 = 21,
    V22 = 22,
    V23 = 23,
    V24 = 24,
    V25 = 25,
    V26 = 26,
    V27 = 27,
    V28 = 28,
    V29 = 29,
}
