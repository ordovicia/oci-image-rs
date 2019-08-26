//! Image layout.
//!
//! See the [OCI image spec] for more information.
//!
//! [OCI image spec]: https://github.com/opencontainers/image-spec/blob/master/image-layout.md

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// File name of the OCI image layout file.
pub const IMAGE_LAYOUT: &str = "oci-layout";

/// Version of the image layout.
pub const IMAGE_LAYOUT_VERSION: &str = "1.0.0";

/// File name of the entry point for an image layout.
pub const INDEX_JSON: &str = "index.json";

/// Name of directory that contains blobs.
pub const BLOBS: &str = "blobs";

/// Structure in the `oci-layout` file.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ImageLayout {
    /// Version of the image layout.
    pub image_layout_version: String,
}

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;

    #[test]
    fn test_image_layout_deser() {
        const JSON: &str = r#"{"imageLayoutVersion":"1.0.0"}"#;
        let image_layout: ImageLayout = serde_json::from_str(JSON).unwrap();
        assert_eq!(
            image_layout,
            ImageLayout {
                image_layout_version: IMAGE_LAYOUT_VERSION.to_string(),
            }
        );
    }

    #[test]
    fn test_image_layout_ser() {
        let image_layout = ImageLayout {
            image_layout_version: IMAGE_LAYOUT_VERSION.to_string(),
        };
        assert_eq!(
            serde_json::to_string(&image_layout).unwrap(),
            r#"{"imageLayoutVersion":"1.0.0"}"#
        );
    }
}
