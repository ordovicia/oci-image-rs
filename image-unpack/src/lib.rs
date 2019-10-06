//! Unpack an OCI image into a runtime bundle.

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

mod convert_config;
mod error;
mod layout;
mod unpack;
mod validate;

use std::path::Path;

pub use error::{Error, ErrorKind, Result};

/// Filter manifests by a set of criteria.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Filter {
    /// Filter manifests by the `org.opencontainers.image.ref.name` annotation.
    RefName(String),

    /// Filter manifests by the targeted platform.
    Platform {
        /// Targeted operating system.
        os: spec::descriptor::Os,

        /// Targeted CPU architecture.
        arch: spec::descriptor::Architecture,
    },
}

/// Unpacks an image layout at `image_dir` into a runtime bundle at `bundle_dir`.
///
/// Filters image manifests by the set of criteria, and selects the one that matches all filters. If
/// no manifest matches with the filters, or multiple manifests match, an error will be returned.
///
/// Other kinds of errors will be returned if e.g. failed to open the directory or a content is
/// invalid.
pub fn unpack(
    image_dir: impl AsRef<Path>,
    bundle_dir: impl AsRef<Path>,
    filters: &[Filter],
) -> Result<()> {
    let layout = layout::read_layout(image_dir.as_ref())?;
    unpack::unpack_index(&layout.index(), &layout, bundle_dir, filters)
}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO
}
