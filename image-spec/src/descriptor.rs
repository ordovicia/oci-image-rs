//! Content descriptor.
//!
//! See the [OCI image spec] for more information.
//!
//! [OCI image spec]: https://github.com/opencontainers/image-spec/blob/v1.0.1/descriptor.md

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{Annotations, Digest, MediaType};

/// Content descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Descriptor {
    /// Media type of the referenced content.
    pub media_type: MediaType,

    /// Digest of the targeted content.
    pub digest: Digest,

    /// Size of the blob in bytes.
    pub size: u64,

    /// List of URLs from which this object MAY be downloaded.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub urls: Vec<Url>,

    /// Arbitrary meta data.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Annotations::is_empty", default)
    )]
    pub annotations: Annotations,

    /// Platform on which the image in the manifest runs.
    ///
    /// This should only be used when referring to a `Manifest`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub platform: Option<Platform>,
    //
    // /// Reserved for future versions of the specification.
    // pub data: String,
}

/// Minimum runtime requirements of an image.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Platform {
    /// CPU architecture.
    pub architecture: Architecture,

    /// Operating system.
    pub os: Os,

    /// Targeted version of the OS.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "os.version", skip_serializing_if = "Option::is_none")
    )]
    pub os_version: Option<String>,

    /// Mandatory OS features.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "os.features", skip_serializing_if = "Vec::is_empty", default)
    )]
    pub os_features: Vec<String>,

    /// Variant of the CPU.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub variant: Option<CpuVariant>,
    //
    // /// Reserved for future versions of the specification.
    // #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    // pub features: Vec<String>,
}

/// Pre-defined types of OSs.
// Listed on https://golang.org/doc/install/source#environment
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Os {
    /// Android.
    Android,
    /// macOS 10.10 and above and iOS.
    Darwin,
    /// DragonFly BSD.
    DragonFly,
    /// FreeBSD.
    FreeBsd,
    /// Linux.
    Linux,
    /// NetBSD.
    NetBsd,
    /// OpenBSD.
    OpenBsd,
    /// Plan9.
    Plan9,
    /// Solaris.
    Solaris,
    /// Windows.
    Windows,
}

/// Error type for parsing a string into an `Os`.
///
/// In a future version, this struct may have fields that convey the cause of error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseOsError;

impl_string_conversion! {
    Os, ParseOsError,
    (Android, "android"),
    (Darwin, "darwin"),
    (DragonFly, "dragonfly"),
    (FreeBsd, "freebsd"),
    (Linux, "linux"),
    (NetBsd, "netbsd"),
    (OpenBsd, "openbsd"),
    (Plan9, "plan9"),
    (Solaris, "solaris"),
    (Windows, "windows"),
}

impl_serde_with_string_conversion!(Os);

/// Pre-defined types of architectures.
// Listed on https://golang.org/doc/install/source#environment
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Architecture {
    /// x86 64-bit.
    Amd64,
    /// Arm 32-bit.
    Arm,
    /// Arm 32-bit.
    Arm64,
    /// x86 32-bit.
    #[allow(non_camel_case_types)]
    i386,
    /// MIPS 32-bit, big-endian.
    Mips,
    /// MIPS 64-bit, big-endian.
    Mips64,
    /// MIPS 64-bit, little-endian.
    MipsLe,
    /// PowerPC 64-bit, big-endian.
    Ppc64,
    /// PowerPC 64-bit, little-endian.
    Ppc64Le,
    /// IBM System z 64-bit, big-endian.
    S390X,
}

/// Error type for parsing a string into an `Architecture`.
///
/// In a future version, this struct may have fields that convey the cause of error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseArchitectureError;

impl_string_conversion! {
    Architecture, ParseArchitectureError,
    (Amd64, "amd64"),
    (Arm, "arm"),
    (Arm64, "arm64"),
    (i386, "i386"),
    (Mips, "mips"),
    (Mips64, "mips64"),
    (MipsLe, "mipsle"),
    (Ppc64, "ppc64"),
    (Ppc64Le, "ppc64le"),
    (S390X, "s390x"),
}

impl_serde_with_string_conversion!(Architecture);

/// Pre-defined variants of CPUs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum CpuVariant {
    /// Arm 32-bit, v6.
    V6,
    /// Arm 32-bit, v7.
    V7,
    /// Arm 32/64-bit, v8.
    V8,
}

// pub const WIN_32K: &str = "win32k";

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;
    use crate::Descriptor;
    use std::str::FromStr;

    // Example from https://github.com/opencontainers/image-spec/blob/v1.0.1/descriptor.md#examples

    #[test]
    fn test_descriptor_deser() {
        const JSON: &str = r#"{
  "mediaType": "application/vnd.oci.image.manifest.v1+json",
  "size": 7682,
  "digest": "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270"
}
"#;

        let descriptor: Descriptor = serde_json::from_str(JSON).unwrap();

        assert_eq!(
            descriptor,
            Descriptor {
                media_type: MediaType::ImageManifest,
                digest: Digest::from_str(
                    "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270"
                )
                .unwrap(),
                size: 7682,
                urls: vec![],
                platform: None,
                annotations: Annotations::new(),
            }
        );

        const JSON_WITH_URL: &str = r#"{
  "mediaType": "application/vnd.oci.image.manifest.v1+json",
  "size": 7682,
  "digest": "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270",
  "urls": [
    "https://example.com/example-manifest"
  ]
}
        "#;

        let descriptor: Descriptor = serde_json::from_str(JSON_WITH_URL).unwrap();

        assert_eq!(
            descriptor,
            Descriptor {
                media_type: MediaType::ImageManifest,
                digest: Digest::from_str(
                    "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270"
                )
                .unwrap(),
                size: 7682,
                urls: vec!["https://example.com/example-manifest"
                    .parse::<Url>()
                    .unwrap()],
                platform: None,
                annotations: Annotations::new(),
            }
        );
    }

    #[test]
    fn test_descriptor_ser() {
        let descriptor = Descriptor {
            media_type: MediaType::ImageManifest,
            digest: Digest::from_str(
                "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270",
            )
            .unwrap(),
            size: 7682,
            urls: vec![],
            platform: None,
            annotations: Annotations::new(),
        };

        const JSON: &str = r#"{
  "mediaType": "application/vnd.oci.image.manifest.v1+json",
  "digest": "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270",
  "size": 7682
}"#;

        assert_eq!(serde_json::to_string_pretty(&descriptor).unwrap(), JSON);

        let descriptor = Descriptor {
            media_type: MediaType::ImageManifest,
            digest: Digest::from_str(
                "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270",
            )
            .unwrap(),
            size: 7682,
            urls: vec!["https://example.com/example-manifest"
                .parse::<Url>()
                .unwrap()],
            platform: None,
            annotations: Annotations::new(),
        };

        const JSON_WITH_URL: &str = r#"{
  "mediaType": "application/vnd.oci.image.manifest.v1+json",
  "digest": "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270",
  "size": 7682,
  "urls": [
    "https://example.com/example-manifest"
  ]
}"#;

        assert_eq!(
            serde_json::to_string_pretty(&descriptor).unwrap(),
            JSON_WITH_URL
        );
    }
}
