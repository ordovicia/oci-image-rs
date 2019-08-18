//! OCI image schema definitions and validation functions.
//!
//! For more information about the spec, see the [OCI image spec].
//!
//! [OCI image spec]: https://github.com/opencontainers/image-spec/blob/master/spec.md

#[macro_use]
macro_rules! impl_str_conv {
    ( $e: ident, $( ($v: ident, $s: expr) ),* ) => {
        impl std::fmt::Display for $e {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
                f.write_str(match self {
                    $(Self::$v => $s),*,
                    Self::Other(s) => s
                })
            }
        }

        impl std::str::FromStr for $e {
            type Err = std::convert::Infallible;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $($s => Self::$v),*,
                    _ => Self::Other(s.to_string()),
                })
            }
        }
    }
}

#[macro_use]
macro_rules! impl_serde_for_str_conv {
    ($s: ident) => {
        #[cfg(feature = "serde")]
        impl serde::Serialize for $s {
            fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                ser.collect_str(self)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $s {
            fn deserialize<D: serde::Deserializer<'de>>(deser: D) -> Result<Self, D::Error> {
                let s = String::deserialize(deser)?;
                Ok(s.parse::<$s>().unwrap())
            }
        }
    };
}

pub mod annotation_keys;
pub mod config;
pub mod descriptor;
pub mod digest;
mod go_set;
mod index;
pub mod layout;
mod manifest;
mod media_types;

/// Schema version for the current specification.
pub const SCHEMA_VERSION: u32 = 2;

/// Key-value map of annotations.
pub type Annotations = std::collections::HashMap<String, String>;

pub use config::{Config, Image};
pub use descriptor::Descriptor;
pub use digest::Digest;
pub use go_set::GoSet;
pub use index::Index;
pub use layout::ImageLayout;
pub use manifest::Manifest;
pub use media_types::MediaType;
