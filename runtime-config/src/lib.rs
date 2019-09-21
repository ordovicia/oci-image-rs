//! OCI runtime config schema in Rust.
//!
//! For more information about the spec, see the [OCI runtime spec].
//!
//! [OCI runtime spec]: https://github.com/opencontainers/runtime-spec/blob/v1.0.0/config.md

#![warn(
    future_incompatible,
    // missing_docs,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused
)]

pub mod config;
pub mod linux;

pub use config::Config;
pub use linux::LinuxConfig;

// /// Schema version for the current specification.
// pub const SCHEMA_VERSION: u32 = 2;

#[cfg(all(feature = "serde", test))]
mod tests {
    // use super::*;

    #[test]
    fn test_config_ser() {
        // TODO
    }

    #[test]
    fn test_config_deser() {
        // TODO
    }
}