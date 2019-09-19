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

mod config;
mod linux;

pub use config::Config;
pub use linux::LinuxConfig;

// /// Schema version for the current specification.
// pub const SCHEMA_VERSION: u32 = 2;
