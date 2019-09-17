//! Runtime configuration schema.
//!
//! See the [OCI runtime spec] for more information.
//!
//! [OCI runtime spec]: https://github.com/opencontainers/runtime-spec/blob/v1.0.1/config.md

mod capability;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, path::PathBuf};

pub use capability::{Capabilities, Capability};

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
    pub linux: Option<crate::Linux>,
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

/// User as which a container process runs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum User {
    /// Posix-platform user.
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    Posix {
        /// User ID in the container namespace.
        uid: u32,

        /// Group ID in the container namespace.
        gid: u32,

        /// Additional group IDs in the container namespace.
        #[cfg_attr(
            feature = "serde",
            serde(skip_serializing_if = "Vec::is_empty", default)
        )]
        additional_gids: Vec<u32>,
    },

    /// Windows user.
    Windows {
        /// User name for the process.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        username: Option<String>,
    },
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

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;

    #[test]
    fn test_deser_user() {
        const JSON_USER_POSIX: &str = r#"{
            "uid": 1,
            "gid": 1,
            "additionalGids": [5, 6]
        }"#;

        const JSON_USER_WINDOWS: &str = r#"{
            "username": "containeradministrator"
        }"#;

        let user_posix: User = serde_json::from_str(JSON_USER_POSIX).unwrap();
        let user_windows: User = serde_json::from_str(JSON_USER_WINDOWS).unwrap();

        assert_eq!(
            user_posix,
            User::Posix {
                uid: 1,
                gid: 1,
                additional_gids: vec![5, 6]
            }
        );

        assert_eq!(
            user_windows,
            User::Windows {
                username: Some(String::from("containeradministrator")),
            }
        );
    }
}
