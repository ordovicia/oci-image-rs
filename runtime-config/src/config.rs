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
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
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

    /// Container's hostname as seen by processes running inside the container.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub hostname: Option<String>,

    /// [POSIX] Set of hooks for configuring custom actions related to the lifecycle of the container.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub hooks: Option<Hooks>,

    /// Arbitrary metadata for the container.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "HashMap::is_empty", default)
    )]
    pub annotations: HashMap<String, String>,

    /// Linux-specific configuration.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub linux: Option<crate::LinuxConfig>,
    // TODO: windows
    // TODO: solaris
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
    pub type_: Option<String>, // TODO: Use proper type?

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

    /// Absolute path to the working directory that will be set for the executable.
    pub cwd: PathBuf,

    /// Environment variables for the process, with similar semantics to IEEE Std 1003.1-2008's `environ`.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub env: Vec<String>,

    /// Arguments for the process, with similar semantics to IEEE Std 1003.1-2008 `execvp`'s `argv`.
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

    /// [Linux] Set of capabilities for the process.
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
pub struct ConsoleSize {
    /// Height of the console in characters of the terminal.
    pub height: usize,
    /// Width of the console in characters of the terminal.
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
    pub type_: String, // TODO: Use proper type?

    /// Value of the limit enforced for the corresponding resource.
    pub soft: u64,

    /// Ceiling for the soft limit.
    pub hard: u64,
}

/// Set of capabilities for a container process.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Capabilities {
    /// Effective capabilities that are kept for the process.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub effective: Vec<Capability>,

    /// Bounding capabilities that are kept for the process.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub bounding: Vec<Capability>,

    /// Inheritable capabilities that are kept for the process.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub inheritable: Vec<Capability>,

    /// Permitted capabilities that are kept for the process.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub permitted: Vec<Capability>,

    /// Ambient capabilities that are kept for the process.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub ambient: Vec<Capability>,
}

/// Valid kinds of capabilities.
///
/// When the feature `serde` is enabled, `Capability` can be serialized to / deserialized from a
/// capability name as defined in the [capabilities(7)] man page (e.g. `CAP_AUDIT_CONTROL`).
///
/// [capabilities(7)]: http://man7.org/linux/man-pages/man7/capabilities.7.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Capability {
    /// `CAP_AUDIT_CONTROL`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_AUDIT_CONTROL"))]
    AuditControl,

    /// `CAP_AUDIT_READ`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_AUDIT_READ"))]
    AuditRead,

    /// `CAP_AUDIT_WRITE`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_AUDIT_WRITE"))]
    AuditWrite,

    /// `CAP_BLOCK_SUSPEND`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_BLOCK_SUSPEND"))]
    BlockSuspend,

    /// `CAP_CHOWN`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_CHOWN"))]
    Chown,

    /// `CAP_DAC_OVERRIDE`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_DAC_OVERRIDE"))]
    DacOverride,

    /// `CAP_DAC_READ_SEARCH`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_DAC_READ_SEARCH"))]
    DacReadSearch,

    /// `CAP_FOWNER`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_FOWNER"))]
    Fowner,

    /// `CAP_FSETID`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_FSETID"))]
    Fsetid,

    /// `CAP_IPC_LOCK`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_IPC_LOCK"))]
    IpcLock,

    /// `CAP_IPC_OWNER`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_IPC_OWNER"))]
    IpcOwner,

    /// `CAP_KILL`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_KILL"))]
    Kill,

    /// `CAP_LEASE`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_LEASE"))]
    Lease,

    /// `CAP_LINUX_IMMUTABLE`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_LINUX_IMMUTABLE"))]
    LinuxImmutable,

    /// `CAP_MAC_ADMIN`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_MAC_ADMIN"))]
    MacAdmin,

    /// `CAP_MAC_OVERRIDE`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_MAC_OVERRIDE"))]
    MacOverride,

    /// `CAP_MKNOD`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_MKNOD"))]
    Mknod,

    /// `CAP_NET_ADMIN`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_NET_ADMIN"))]
    NetAdmin,

    /// `CAP_NET_BIND_SERVICE`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_NET_BIND_SERVICE"))]
    NetBindService,

    /// `CAP_NET_BROADCAST`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_NET_BROADCAST"))]
    NetBroadcast,

    /// `CAP_NET_RAW`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_NET_RAW"))]
    NetRaw,

    /// `CAP_SETFCAP`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SETFCAP"))]
    Setfcap,

    /// `CAP_SETGID`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SETGID"))]
    Setgid,

    /// `CAP_SETPCAP`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SETPCAP"))]
    Setpcap,

    /// `CAP_SETUID`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SETUID"))]
    Setuid,

    /// `CAP_SYSLOG`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYSLOG"))]
    Syslog,

    /// `CAP_SYS_ADMIN`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYS_ADMIN"))]
    SysAdmin,

    /// `CAP_SYS_BOOT`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYS_BOOT"))]
    SysBoot,

    /// `CAP_SYS_CHROOT`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYS_CHROOT"))]
    SysChroot,

    /// `CAP_SYS_MODULE`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYS_MODULE"))]
    SysModule,

    /// `CAP_SYS_NICE`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYS_NICE"))]
    SysNice,

    /// `CAP_SYS_PACCT`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYS_PACCT"))]
    SysPacct,

    /// `CAP_SYS_PTRACE`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYS_PTRACE"))]
    SysPtrace,

    /// `CAP_SYS_RAWIO`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYS_RAWIO"))]
    SysRawio,

    /// `CAP_SYS_RESOURCE`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYS_RESOURCE"))]
    SysResource,

    /// `CAP_SYS_TIME`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYS_TIME"))]
    SysTime,

    /// `CAP_SYS_TTY_CONFIG`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_SYS_TTY_CONFIG"))]
    SysTtyConfig,

    /// `CAP_WAKE_ALARM`
    #[cfg_attr(feature = "serde", serde(rename = "CAP_WAKE_ALARM"))]
    WakeAlarm,
}

/// Set of hooks for configuring custom actions related to the lifecycle of a container.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Hooks {
    /// Pre-start hooks.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub prestart: Vec<Hook>,

    /// Post-start hooks.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub poststart: Vec<Hook>,

    /// Post-stop hooks.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub poststop: Vec<Hook>,
}

/// Hook for configuring custom actions related to the lifecycle of a container.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Hook {
    /// Absolute path to the executable, with similar semantics to IEEE Std 1003.1-2008 `execv`'s
    /// `path`.
    pub path: PathBuf,

    /// Arguments for the executable, with similar semantics to IEEE Std 1003.1-2008 `execv`'s
    /// `argv`.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub args: Vec<String>,

    /// Environment variables for the executable, with similar semantics to IEEE Std 1003.1-2008's
    /// `environ`.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub env: Vec<String>,

    /// The number of seconds before aborting the hook.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub timeout: Option<u32>,
}
