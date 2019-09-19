#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Annotations, Descriptor};

/// Image manifest.
///
/// See the [OCI image spec] for more information.
///
/// [OCI image spec]: https://github.com/opencontainers/image-spec/blob/master/manifest.md
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Manifest {
    /// Image manifest schema version.
    pub schema_version: u32,

    // /// Reserved to maintain compatibility.
    // #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    // pub media_type: Option<MediaType>,
    //
    /// References a configuration object for a container, by digest.
    pub config: Descriptor,

    /// Indexed list of layers.
    pub layers: Vec<Descriptor>,

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
    use crate::{Digest, MediaType};
    use std::str::FromStr;

    // Example from https://github.com/opencontainers/image-spec/blob/master/manifest.md#example-image-manifest

    #[test]
    fn test_manifest_deser() {
        const JSON: &str = r#"{
  "schemaVersion": 2,
  "config": {
    "mediaType": "application/vnd.oci.image.config.v1+json",
    "size": 7023,
    "digest": "sha256:b5b2b2c507a0944348e0303114d8d93aaaa081732b86451d9bce1f432a537bc7"
  },
  "layers": [
    {
      "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
      "size": 32654,
      "digest": "sha256:9834876dcfb05cb167a5c24953eba58c4ac89b1adf57f28f2f9d09af107ee8f0"
    },
    {
      "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
      "size": 16724,
      "digest": "sha256:3c3a4604a545cdc127456d94e421cd355bca5b528f4a9c1905b15da2eb4a4c6b"
    },
    {
      "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
      "size": 73109,
      "digest": "sha256:ec4b8955958665577945c89419d1af06b5f7636b4ac3da7f12184802ad867736"
    }
  ],
  "annotations": {
    "com.example.key1": "value1",
    "com.example.key2": "value2"
  }
}"#;

        let manifest: Manifest = serde_json::from_str(JSON).unwrap();

        assert_eq!(
            manifest,
            Manifest {
                schema_version: 2,
                config: Descriptor {
                    media_type: MediaType::ImageConfig,
                    digest: Digest::from_str(
                        "sha256:b5b2b2c507a0944348e0303114d8d93aaaa081732b86451d9bce1f432a537bc7"
                    )
                    .unwrap(),
                    size: 7023,
                    urls: vec![],
                    annotations: Annotations::new(),
                    platform: None,
                },
                layers: vec![
          Descriptor {
            media_type: MediaType::LayerTarGzip,
            digest: Digest::from_str(
              "sha256:9834876dcfb05cb167a5c24953eba58c4ac89b1adf57f28f2f9d09af107ee8f0"
            )
            .unwrap(),
            size: 32654,
            urls: vec![],
            annotations: Annotations::new(),
            platform: None,
          },
          Descriptor {
            media_type: MediaType::LayerTarGzip,
            digest: Digest::from_str(
              "sha256:3c3a4604a545cdc127456d94e421cd355bca5b528f4a9c1905b15da2eb4a4c6b"
            )
            .unwrap(),
            size: 16724,
            urls: vec![],
            annotations: Annotations::new(),
            platform: None,
          },
          Descriptor {
            media_type: MediaType::LayerTarGzip,
            digest: Digest::from_str(
              "sha256:ec4b8955958665577945c89419d1af06b5f7636b4ac3da7f12184802ad867736"
            )
            .unwrap(),
            size: 73109,
            urls: vec![],
            annotations: Annotations::new(),
            platform: None,
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
    fn test_manifest_ser() {
        let manifest = Manifest {
            schema_version: 2,
            config: Descriptor {
                media_type: MediaType::ImageConfig,
                digest: Digest::from_str(
                    "sha256:b5b2b2c507a0944348e0303114d8d93aaaa081732b86451d9bce1f432a537bc7",
                )
                .unwrap(),
                size: 7023,
                urls: vec![],
                annotations: Annotations::new(),
                platform: None,
            },
            layers: vec![
                Descriptor {
                    media_type: MediaType::LayerTarGzip,
                    digest: Digest::from_str(
                        "sha256:9834876dcfb05cb167a5c24953eba58c4ac89b1adf57f28f2f9d09af107ee8f0",
                    )
                    .unwrap(),
                    size: 32654,
                    urls: vec![],
                    annotations: Annotations::new(),
                    platform: None,
                },
                Descriptor {
                    media_type: MediaType::LayerTarGzip,
                    digest: Digest::from_str(
                        "sha256:3c3a4604a545cdc127456d94e421cd355bca5b528f4a9c1905b15da2eb4a4c6b",
                    )
                    .unwrap(),
                    size: 16724,
                    urls: vec![],
                    annotations: Annotations::new(),
                    platform: None,
                },
                Descriptor {
                    media_type: MediaType::LayerTarGzip,
                    digest: Digest::from_str(
                        "sha256:ec4b8955958665577945c89419d1af06b5f7636b4ac3da7f12184802ad867736",
                    )
                    .unwrap(),
                    size: 73109,
                    urls: vec![],
                    annotations: Annotations::new(),
                    platform: None,
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

        const JSON: &str = r#"{
  "schemaVersion": 2,
  "config": {
    "mediaType": "application/vnd.oci.image.config.v1+json",
    "digest": "sha256:b5b2b2c507a0944348e0303114d8d93aaaa081732b86451d9bce1f432a537bc7",
    "size": 7023
  },
  "layers": [
    {
      "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
      "digest": "sha256:9834876dcfb05cb167a5c24953eba58c4ac89b1adf57f28f2f9d09af107ee8f0",
      "size": 32654
    },
    {
      "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
      "digest": "sha256:3c3a4604a545cdc127456d94e421cd355bca5b528f4a9c1905b15da2eb4a4c6b",
      "size": 16724
    },
    {
      "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
      "digest": "sha256:ec4b8955958665577945c89419d1af06b5f7636b4ac3da7f12184802ad867736",
      "size": 73109
    }
  ],
  "annotations": {
    "com.example.key1": "value1"
  }
}"#;

        assert_eq!(serde_json::to_string_pretty(&manifest).unwrap(), JSON);
    }
}
