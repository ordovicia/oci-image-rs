//! Seccomp configuration for a container.
//!
//! For more information about seccomp, see the [kernel docs].
//!
//! [kernel docs]: https://www.kernel.org/doc/Documentation/prctl/seccomp_filter.txt

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Seccomp config for a container.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Seccomp {
    /// Default action for seccomp.
    pub default_action: Action,

    /// Architectures used for system calls.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub architectures: Vec<Architecture>,

    /// List of system call filters.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub syscalls: Vec<Syscall>,
}

/// Types of actions for seccomp rules.
///
/// When the feature `serde` is enabled, `Action` can be serialized to / deserialized from a name of
/// an action in a SCREAMING_SNAKE_CASE (e.g. `SCMP_ACT_KILL`).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
#[allow(missing_docs)]
pub enum Action {
    ScmpActKill,
    ScmpActTrap,
    ScmpActErrno,
    ScmpActTrace,
    ScmpActAllow,
}

/// List of architectures used for system calls.
///
/// When the feature `serde` is enabled, `Architecture` can be serialized to / deserialized from a
/// name of an action in a SCREAMING_SNAKE_CASE (e.g. `SCMP_ARCH_X86`).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
#[allow(missing_docs)]
pub enum Architecture {
    ScmpArchX86,
    ScmpArchX86_64,
    ScmpArchX32,
    ScmpArchArm,
    ScmpArchAarch64,
    ScmpArchMips,
    ScmpArchMips64,
    ScmpArchMips64n32,
    ScmpArchMipsel,
    ScmpArchMipsel64,
    ScmpArchMipsel64n32,
    ScmpArchPpc,
    ScmpArchPpc64,
    ScmpArchPpc64le,
    ScmpArchS390,
    ScmpArchS390x,
    ScmpArchParisc,
    ScmpArchParisc64,
}

/// List of system call filters in seccomp.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Syscall {
    /// Names of the syscalls.
    pub names: Vec<String>,

    /// Action for the seccomp rules.
    pub action: Action,

    /// System call filter.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub args: Vec<SyscallArg>,
}

/// System call filter in seccomp.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct SyscallArg {
    /// Index for system call arguments.
    pub index: usize,

    /// Value for system call arguments.
    pub value: u64,

    /// Second value for system call arguments.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub value_two: Option<u64>,

    /// Comparator for system call arguments.
    pub op: SyscallCmp,
}

/// Comparator for system call arguments in seccomp.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
#[allow(missing_docs)]
pub enum SyscallCmp {
    ScmpCmpNe,
    ScmpCmpLt,
    ScmpCmpLe,
    ScmpCmpEq,
    ScmpCmpGe,
    ScmpCmpGt,
    ScmpCmpMaskedEq,
}
