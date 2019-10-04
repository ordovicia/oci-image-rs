//! Schema of Linux-specific config section.
//!
//! For more information about the spec, see the [OCI runtime spec for Linux-specific section].
//!
//! [OCI runtime spec for Linux-specific section]: https://github.com/opencontainers/runtime-spec/blob/v1.0.0/config-linux.md

pub mod resources;
pub mod seccomp;

pub use resources::Resources;
pub use seccomp::Seccomp;

use std::{collections::HashMap, path::PathBuf};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Schema of Linux-specific config section.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct LinuxConfig {
    /// List of namespaces attached to the container.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub namespaces: Vec<Namespace>,

    /// User namespace UID mappings from the host to the container.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub uid_mappings: Vec<UserNamespaceMappings>,

    /// User namespace GID mappings from the host to the container.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub gid_mappings: Vec<UserNamespaceMappings>,

    /// List of devices that MUST be available in the container.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub devices: Vec<Device>,

    /// Path to the cgroups to which the container is attached.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cgroups_path: Option<PathBuf>,

    /// Resource limits for the container forced by cgroups.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub resources: Option<Resources>,

    /// Intel Resource Director Technology.
    ///
    /// See the [kernel docs] for more information.
    ///
    /// [kernel docs]: https://www.kernel.org/doc/Documentation/x86/intel_rdt_ui.txt
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub intel_rdt: Option<IntelRdt>,

    /// Kernel parameters to be modified at runtime for the container.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "HashMap::is_empty"))]
    pub sysctl: HashMap<String, String>,

    /// Seccomp config for the container.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub seccomp: Option<Seccomp>,

    /// Rootfs mount propagation.
    ///
    /// See the [kernel docs] for more information.
    ///
    /// [kernel docs]: https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub rootfs_propagation: Option<RootfsPropagation>,

    /// List of paths that will be masked so that they cannot be read. The values MUST be absolute
    /// paths in the container namespace.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub masked_paths: Vec<PathBuf>,

    /// List of paths that will made readonly inside the container. The values MUST be absolute
    /// paths in the container namespace.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub readonly_paths: Vec<PathBuf>,

    /// SELinux context for the mounts in the container.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mount_label: Option<String>,
}

/// Namespace attached to this container.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Namespace {
    /// Namespace type.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: NamespaceType,

    /// Absolute path to the namespace file in the runtime mount namespace.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub path: Option<PathBuf>,
}

/// Namespace types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum NamespaceType {
    /// PID namespace.
    Pid,
    /// Network namespace.
    Network,
    /// Mount namespace.
    Mount,
    /// IPC namespace.
    Ipc,
    /// UTS namespace.
    Uts,
    /// User namespace.
    User,
    /// Isolated view of a cgroup hierarchy.
    Cgroup,
}

/// User namespace ID mappings from a host to a container.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UserNamespaceMappings {
    /// Starting ID on the host to be mapped to `container_id`.
    #[cfg_attr(feature = "serde", serde(rename = "hostID"))]
    pub host_id: u32,

    /// Starting ID in the container.
    #[cfg_attr(feature = "serde", serde(rename = "containerID"))]
    pub container_id: u32,

    /// Number of IDs to be mapped.
    pub size: u32,
}

/// Device that MUST be available in the container.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Device {
    /// Device type.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: DeviceType,

    /// Absolute path to the device inside the container.
    pub path: PathBuf,

    /// Major number for the device.
    pub major: Option<i64>,

    /// Minor number for the device.
    pub minor: Option<i64>,

    /// File mode for the device.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub file_mode: Option<u32>,

    /// UID of the device owner.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub uid: Option<u32>,

    /// GID of the device group.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub gid: Option<u32>,
}

/// Device type.
///
/// When the feature `serde` is enabled, `DeviceType` can be serialized to / deserialized from a
/// single character representing the device type (e.g. `c`, `b`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeviceType {
    /// Character device (type `c`).
    #[cfg_attr(feature = "serde", serde(rename = "c"))]
    Character,

    /// Block device (type `b`).
    #[cfg_attr(feature = "serde", serde(rename = "b"))]
    Block,

    /// Unbuffered device (type `u`).
    #[cfg_attr(feature = "serde", serde(rename = "u"))]
    Unbuffered,

    /// FIFO device (type `p`).
    #[cfg_attr(feature = "serde", serde(rename = "p"))]
    Fifo,
}

/// Intel Resource Director Technology.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct IntelRdt {
    /// Schema for L3 cache ID and capacity bitmask (CBM).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub l3_cache_schema: Option<String>,
}

/// Rootfs mount propagation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
#[allow(missing_docs)]
pub enum RootfsPropagation {
    Slave,
    Private,
    Shared,
    Unbindable,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intel_rdt_deser() {
        const JSON: &str = r#"{
            "l3CacheSchema": "L3:0=ffff0;1=3ff"
        }"#;

        let intel_rdt: IntelRdt = serde_json::from_str(JSON).unwrap();

        assert_eq!(
            intel_rdt,
            IntelRdt {
                l3_cache_schema: Some(String::from("L3:0=ffff0;1=3ff"))
            }
        );
    }
}
