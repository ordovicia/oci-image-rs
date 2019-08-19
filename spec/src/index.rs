#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Annotations, Descriptor};

/// Image index.
/// See the [OCI image spec] for more information.
///
/// [OCI image spec]: https://github.com/opencontainers/image-spec/blob/master/image-index.md
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Index {
    /// Image manifest schema version.
    pub schema_version: u32,

    // /// Reserved to maintain compatibility.
    // #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    // pub media_type: Option<MediaType>,
    //
    /// List of manifests for specific platforms.
    pub manifests: Vec<Descriptor>,

    /// Arbitrary metadata.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Annotations::is_empty", default)
    )]
    pub annotations: Annotations,
}

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;
    use crate::{
        descriptor::{Architecture, Os, Platform},
        Digest, MediaType,
    };
    use std::str::FromStr;

    // Example from https://github.com/opencontainers/image-spec/blob/master/image-index.md#example-image-index

    #[test]
    fn test_index_deser() {
        let json = r#"
{
  "schemaVersion": 2,
  "manifests": [
    {
      "mediaType": "application/vnd.oci.image.manifest.v1+json",
      "size": 7143,
      "digest": "sha256:e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f",
      "platform": {
        "architecture": "ppc64le",
        "os": "linux"
      }
    },
    {
      "mediaType": "application/vnd.oci.image.manifest.v1+json",
      "size": 7682,
      "digest": "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270",
      "platform": {
        "architecture": "amd64",
        "os": "linux"
      }
    }
  ],
  "annotations": {
    "com.example.key1": "value1",
    "com.example.key2": "value2"
  }
}
"#;

        let index: Index = serde_json::from_str(&json).unwrap();

        assert_eq!(
            index,
            Index {
                schema_version: 2,
                manifests: vec![
          Descriptor {
            media_type: MediaType::ImageManifest,
            digest: Digest::from_str(
              "sha256:e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f"
            )
            .unwrap(),
            size: 7143,
            urls: vec![],
            annotations: Annotations::new(),
            platform: Some(Platform {
              architecture: Architecture::Ppc64Le,
              os: Os::Linux,
              os_version: None,
              os_features: vec![],
              variant: None,
            }),
          },
          Descriptor {
            media_type: MediaType::ImageManifest,
            digest: Digest::from_str(
              "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270"
            )
            .unwrap(),
            size: 7682,
            urls: vec![],
            annotations: Annotations::new(),
            platform: Some(Platform {
              architecture: Architecture::Amd64,
              os: Os::Linux,
              os_version: None,
              os_features: vec![],
              variant: None,
            }),
          },
        ],
                annotations: [
                    ("com.example.key1", "value1"),
                    ("com.example.key2", "value2")
                ]
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<Annotations>(),
            }
        );
    }

    #[test]
    fn test_index_ser() {
        let index = Index {
            schema_version: 2,
            manifests: vec![
                Descriptor {
                    media_type: MediaType::ImageManifest,
                    digest: Digest::from_str(
                        "sha256:e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f",
                    )
                    .unwrap(),
                    size: 7143,
                    urls: vec![],
                    annotations: Annotations::new(),
                    platform: Some(Platform {
                        architecture: Architecture::Ppc64Le,
                        os: Os::Linux,
                        os_version: None,
                        os_features: vec![],
                        variant: None,
                    }),
                },
                Descriptor {
                    media_type: MediaType::ImageManifest,
                    digest: Digest::from_str(
                        "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270",
                    )
                    .unwrap(),
                    size: 7682,
                    urls: vec![],
                    annotations: Annotations::new(),
                    platform: Some(Platform {
                        architecture: Architecture::Amd64,
                        os: Os::Linux,
                        os_version: None,
                        os_features: vec![],
                        variant: None,
                    }),
                },
            ],
            annotations: [
                ("com.example.key1", "value1"),
                // ("com.example.key2", "value2"),
            ]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Annotations>(),
        };

        assert_eq!(
            serde_json::to_string_pretty(&index).unwrap(),
            r#"{
  "schemaVersion": 2,
  "manifests": [
    {
      "mediaType": "application/vnd.oci.image.manifest.v1+json",
      "digest": "sha256:e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f",
      "size": 7143,
      "platform": {
        "architecture": "ppc64le",
        "os": "linux"
      }
    },
    {
      "mediaType": "application/vnd.oci.image.manifest.v1+json",
      "digest": "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270",
      "size": 7682,
      "platform": {
        "architecture": "amd64",
        "os": "linux"
      }
    }
  ],
  "annotations": {
    "com.example.key1": "value1"
  }
}"#
        );
    }
}
