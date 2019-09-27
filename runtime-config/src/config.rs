//! Runtime configuration schema.
//!
//! See the [OCI runtime spec] for more information.
//!
//! [OCI runtime spec]: https://github.com/opencontainers/runtime-spec/blob/v1.0.1/config.md

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, path::PathBuf};

/// Runtime configuration schema.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Config {
    /// Version of the OCI runtime spec.
    pub oci_version: String,

    /// Container's root filesystem.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub root: Option<Root>,

    /// Additional mounts beyond the root filesystem.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub mounts: Vec<Mount>,

    /// Container process.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub process: Option<Process>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub hostname: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub linux: Option<crate::LinuxConfig>,
    // TODO: windows
    // TODO: solaris
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub hooks: Option<Hooks>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "HashMap::is_empty", default)
    )]
    pub annotations: HashMap<String, String>,
}

/// Container's root filesystem.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Root {
    /// Path to the root filesystem.
    pub path: PathBuf,

    /// Whether the root filesystem MUST be read-only inside the container.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub readonly: Option<bool>,
}

/// Additional filesystem mounts beyond the root filesystem of a container.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mount {
    /// Destination of the mount point as an absolute path inside the container.
    pub destination: PathBuf,

    /// [POSIX] Type of the filesystem to be mounted.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "type", skip_serializing_if = "Option::is_none")
    )]
    pub type_: Option<String>,

    /// Device name, directory name, or dummy.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub source: Option<PathBuf>,

    /// Mount options of the filesystem to be mounted.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub options: Vec<String>,
}

/// Container process.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Process {
    /// Whether a terminal is attached to the process.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub terminal: Option<bool>,

    /// Console size in characters of the terminal.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub console_size: Option<ConsoleSize>,

    /// As which user the process runs.
    pub user: User,

    /// Working directory that will be set for the executable.
    pub cwd: PathBuf,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub env: Vec<String>,

    pub args: Vec<String>,

    /// [POSIX] Resource limits for the process.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub rlimits: Vec<Rlimit>,

    /// [Linux] Name of the AppArmor profile for the process.
    ///
    /// See the [AppArmor docs] for more information.
    ///
    /// [AppArmor docs]: https://wiki.ubuntu.com/AppArmor
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub apparmor_profile: Option<String>,

    // [Linux]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub capabilities: Option<Capabilities>,

    /// [Linux] Whether to prevent the process from gaining additional privileges.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub no_new_privileges: Option<bool>,

    /// [Linux] Adjusts the OOM killer score.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub oom_score_adj: Option<i32>,

    /// [Linux] SELinux label for the process.
    ///
    /// See the [SELinux docs] for more information.
    ///
    /// [SELinux docs]: http://selinuxproject.org/page/Main_Page
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub selinux_label: Option<String>,
}

/// Console size in characters of a terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(missing_docs)]
pub struct ConsoleSize {
    pub height: usize,
    pub width: usize,
}

/// As which user a container process runs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct User {
    /// [POSIX] User ID in the container namespace.
    pub uid: u32,
    /// [POSIX] Group ID in the container namespace.
    pub gid: u32,

    /// [POSIX] Additional group IDs in the container namespace.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub additional_gids: Vec<u32>,

    /// [Windows] User name for the process.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub username: Option<String>,
}

/// Resource limits for a container process.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rlimit {
    /// Type of platform resource being limited.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: String,

    /// Value of the limit enforced for the corresponding resource.
    pub soft: u64,

    /// Ceiling for the soft limit.
    pub hard: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Capabilities {
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub effective: Vec<Capability>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub bounding: Vec<Capability>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub inheritable: Vec<Capability>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub permitted: Vec<Capability>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub ambient: Vec<Capability>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum Capability {
    CapAuditControl,
    CapAuditRead,
    CapAuditWrite,
    CapBlockSuspend,
    CapChown,
    CapDacOverride,
    CapDacReadSearch,
    CapFowner,
    CapFsetid,
    CapIpcLock,
    CapIpcOwner,
    CapKill,
    CapLease,
    CapLinuxImmutable,
    CapMacAdmin,
    CapMacOverride,
    CapMknod,
    CapNetAdmin,
    CapNetBindService,
    CapNetBroadcast,
    CapNetRaw,
    CapSetfcap,
    CapSetgid,
    CapSetpcap,
    CapSetuid,
    CapSyslog,
    CapSysAdmin,
    CapSysBoot,
    CapSysChroot,
    CapSysModule,
    CapSysNice,
    CapSysPacct,
    CapSysPtrace,
    CapSysRawio,
    CapSysResource,
    CapSysTime,
    CapSysTtyConfig,
    CapWakeAlarm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Hooks {
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub prestart: Vec<Hook>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub poststart: Vec<Hook>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub poststop: Vec<Hook>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Hook {
    pub path: PathBuf,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub args: Vec<String>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub env: Vec<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub timeout: Option<u32>,
}
