/// Pre-defined and other media types.
///
/// See the [OCI image spec] for more information.
///
/// [OCI image spec]: https://github.com/opencontainers/image-spec/blob/master/media-types.md
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MediaType {
    /// Content descriptor,
    ContentDescriptor,
    /// `oci-layout` file.
    OciLayout,
    /// Image index.
    ImageIndex,
    /// Image manifest.
    ImageManifest,
    /// Image configuration.
    ImageConfig,
    /// Layers as a tar archive.
    LayerTar,
    /// Layers as a tar archive compressed with gzip.
    LayerTarGzip,
    /// Layers as a tar archive with distribution restrictions.
    LayerTarNondistributable,
    /// Layers as a tar archive compressed with gzip with distribution restrictions.
    LayerTarGzipNondistributable,
    /// Other (not pre-defined) media type.
    Other(String),
}

macro_rules! _impl_str_conv {
    ( $( ($v: ident, $s: literal) ),* ) => {
        impl_str_conv!(MediaType, $( ( $v, concat!("application/vnd.oci.", $s) ) ),* );
    };
}

_impl_str_conv! {
    (ContentDescriptor, "descriptor.v1+json"),
    (OciLayout, "layout.header.v1+json"),
    (ImageIndex, "image.index.v1+json"),
    (ImageManifest, "image.manifest.v1+json"),
    (ImageConfig, "image.config.v1+json"),
    (LayerTar, "image.layer.v1.tar"),
    (LayerTarGzip, "image.layer.v1.tar+gzip"),
    (LayerTarNondistributable, "image.layer.nondistributable.v1.tar"),
    (LayerTarGzipNondistributable, "image.layer.nondistributable.v1.tar+gzip")
}
impl_serde_for_str_conv!(MediaType);

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;

    #[test]
    fn test_media_type_deser() {
        let media_type: MediaType =
            serde_json::from_str(r#""application/vnd.oci.image.index.v1+json""#).unwrap();
        assert_eq!(media_type, MediaType::ImageIndex);

        let media_type: MediaType =
            serde_json::from_str(r#""application/vnd.oci.image.manifest.v1+json""#).unwrap();
        assert_eq!(media_type, MediaType::ImageManifest);

        let media_type: MediaType =
            serde_json::from_str(r#""application/vnd.oci.image.config.v1+json""#).unwrap();
        assert_eq!(media_type, MediaType::ImageConfig);
    }

    #[test]
    fn test_media_type_ser() {
        assert_eq!(
            serde_json::to_string(&MediaType::ContentDescriptor).unwrap(),
            r#""application/vnd.oci.descriptor.v1+json""#
        );
        assert_eq!(
            serde_json::to_string(&MediaType::OciLayout).unwrap(),
            r#""application/vnd.oci.layout.header.v1+json""#
        );
        assert_eq!(
            serde_json::to_string(&MediaType::Other(
                "application/vnd.oci.foo.bar.v1+json".to_string()
            ))
            .unwrap(),
            r#""application/vnd.oci.foo.bar.v1+json""#
        );
    }
}
