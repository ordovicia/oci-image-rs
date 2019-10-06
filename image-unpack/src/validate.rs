use spec::MediaType;

use crate::{layout::Layout, Error, ErrorKind, Result};

/// Image layout, whose layout version is validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedImageLayout {
    image_layout: spec::ImageLayout,
}

/// Validates the image layout version.
///
/// Returns `ValidatedImageLayout` if validated, otherwise an error with kind
/// [`ErrorKind::LayoutVersionNotSupported`].
///
/// [`ErrorKind::LayoutVersionNotSupported`]: ../enum.ErrorKind.html#variant.LayoutVersionNotSupported
pub fn validate_image_layout(image_layout: spec::ImageLayout) -> Result<ValidatedImageLayout> {
    // FIXME: Detect other versions
    match image_layout.image_layout_version.as_ref() {
        spec::layout::IMAGE_LAYOUT_VERSION => Ok(ValidatedImageLayout { image_layout }),
        _ => Err(Error::new(ErrorKind::LayoutVersionNotSupported)),
    }
}

/// Image index, whose schema version is validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedIndex {
    index: spec::Index,
}

/// Validates the schema version of an image index.
///
/// Returns `ValidatedIndex` if validated, otherwise an error with kind
/// [`ErrorKind::SchemaVersionNotSupported`].
///
/// [`ErrorKind::SchemaVersionNotSupported`]: ../enum.ErrorKind.html#variant.SchemaVersionNotSupported
pub fn validate_index(index: spec::Index) -> Result<ValidatedIndex> {
    match index.schema_version {
        spec::SCHEMA_VERSION => Ok(ValidatedIndex { index }),
        _ => Err(Error::new(ErrorKind::SchemaVersionNotSupported)),
    }
}

impl ValidatedIndex {
    /// Returns the descriptors to manifests the underlying index has.
    pub fn manifests(&self) -> &[spec::Descriptor] {
        &self.index.manifests
    }
}

/// Image manifest, whose schema version is validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedManifest {
    manifest: spec::Manifest,
}

/// Validates the schema version of an image manifest.
///
/// Returns `ValidatedManifest` if validated, otherwise an error with kind
/// [`ErrorKind::SchemaVersionNotSupported`].
///
/// [`ErrorKind::SchemaVersionNotSupported`]: ../enum.ErrorKind.html#variant.SchemaVersionNotSupported
pub fn validate_manifest(manifest: spec::Manifest) -> Result<ValidatedManifest> {
    match manifest.schema_version {
        spec::SCHEMA_VERSION => Ok({ ValidatedManifest { manifest } }),
        _ => Err(Error::new(ErrorKind::SchemaVersionNotSupported)),
    }
}

impl ValidatedManifest {
    /// Returns the descriptor to image config the underlying manifest has.
    pub fn config(&self) -> &spec::Descriptor {
        &self.manifest.config
    }

    /// Returns the descriptors to layers the underlying manifest has.
    pub fn layers(&self) -> &[spec::Descriptor] {
        &self.manifest.layers
    }
}

/// Descriptor that refers an image config, whose media type is validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedImageConfigDescriptor<'a> {
    desc: &'a spec::Descriptor,
}

/// Validates the media type of a descriptor that should refer an image config.
///
/// Returns `ValidatedImageConfigDescriptor` if validated, otherwise an error with kind
/// [`ErrorKind::UnexpectedMediaType`].
///
/// [`ErrorKind::UnexpectedMediaType`]: ../enum.ErrorKind.html#variant.UnexpectedMediaType
pub fn validate_image_config_descriptor(
    desc: &spec::Descriptor,
) -> Result<ValidatedImageConfigDescriptor<'_>> {
    match desc.media_type {
        MediaType::ImageConfig => Ok(ValidatedImageConfigDescriptor { desc }),
        _ => Err(Error::new(ErrorKind::UnexpectedMediaType)),
    }
}

impl AsRef<spec::Descriptor> for ValidatedImageConfigDescriptor<'_> {
    fn as_ref(&self) -> &spec::Descriptor {
        &self.desc
    }
}

/// Descriptor that refers a layer, whose media type is validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedLayerDescriptor<'a> {
    desc: &'a spec::Descriptor,
}

/// Validates the media type of a descriptor that should refer a layer.
///
/// Returns `ValidatedLayerDescriptor` if validated, otherwise an error with kind
/// [`ErrorKind::UnexpectedMediaType`].
///
/// [`ErrorKind::UnexpectedMediaType`]: ../enum.ErrorKind.html#variant.UnexpectedMediaType
pub fn validate_layer_descriptor(desc: &spec::Descriptor) -> Result<ValidatedLayerDescriptor<'_>> {
    match desc.media_type {
        MediaType::LayerTar
        | MediaType::LayerTarGzip
        | MediaType::LayerTarNondistributable
        | MediaType::LayerTarGzipNondistributable => Ok(ValidatedLayerDescriptor { desc }),
        _ => Err(Error::new(ErrorKind::UnexpectedMediaType)),
    }
}

impl AsRef<spec::Descriptor> for ValidatedLayerDescriptor<'_> {
    fn as_ref(&self) -> &spec::Descriptor {
        &self.desc
    }
}

/// Descriptor, whose content is verified with the digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedDescriptor<'a> {
    desc: &'a spec::Descriptor,
    content_path: std::path::PathBuf,
}

/// Verifies a content referenced by a descriptor by the size and digest.
///
/// Returns `VerifiedDescriptor` if verified, otherwise an error with kind
/// [`ErrorKind::VerifyContent`]. Other kinds of errors can be returned if e.g. failed to open a
/// file.
///
/// [`ErrorKind::VerifyContent`]: ../enum.ErrorKind.html#variant.VerifyContent
pub fn verify_descriptor<'a>(
    desc: &'a spec::Descriptor,
    layout: &Layout,
) -> Result<VerifiedDescriptor<'a>> {
    let content_path = layout.content_path(desc);
    let file = std::fs::File::open(&content_path)?;

    if file.metadata()?.len() == desc.size && desc.digest.verify(&file)? {
        Ok(VerifiedDescriptor { desc, content_path })
    } else {
        Err(Error::new(ErrorKind::VerifyContent))
    }
}

impl<'a> VerifiedDescriptor<'a> {
    /// Deserializes JSON file referenced by this descriptor into a `T` value.
    pub fn deser<T>(&self) -> Result<T>
    where
        for<'de> T: serde::de::Deserialize<'de>,
    {
        let file = std::fs::File::open(&self.content_path)?;
        serde_json::from_reader(&file).map_err(Error::deser)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO
}
