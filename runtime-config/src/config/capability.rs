#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
