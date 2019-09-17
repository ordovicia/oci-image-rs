#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Config {
    pub oci_version: String,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub root: Option<Root>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub mounts: Vec<Mount>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub process: Option<Process>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub hostname: Option<String>,

    // TODO: linux
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Root {
    pub path: PathBuf,

    // not on Windows
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub readonly: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mount {
    pub destination: PathBuf,

    // POSIX
    #[cfg_attr(
        feature = "serde",
        serde(rename = "type", skip_serializing_if = "Option::is_none")
    )]
    pub type_: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub source: Option<PathBuf>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub options: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Process {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub terminal: Option<bool>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub console_size: Option<ConsoleSize>,

    pub user: User,

    pub cwd: String,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub env: Vec<String>,

    pub args: Vec<String>,

    // POSIX
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub rlimits: Vec<Rlimit>,

    // Linux
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub apparmor_profile: Option<String>,

    // Linux
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub capabilities: Option<Capabilities>,

    // Linux
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub no_new_privileges: Option<bool>,

    // Linux
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub oom_score_adj: Option<i32>,

    // Linux
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub selinux_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConsoleSize {
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct User {
    // POSIX
    pub uid: u32,
    // POSIX
    pub gid: u32,

    // POSIX
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub additional_gids: Vec<u32>,

    // Windows
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rlimit {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: String,

    pub soft: u64,
    pub hard: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Capabilities {
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub effective: Vec<CapabilityType>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub bounding: Vec<CapabilityType>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub inheritable: Vec<CapabilityType>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub permitted: Vec<CapabilityType>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub ambient: Vec<CapabilityType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum CapabilityType {
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
