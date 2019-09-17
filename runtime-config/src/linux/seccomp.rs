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
/// When the feature `serde` is enabled, `Action` can be serialized to / deserialized from an action
/// name as defined in libseccomp (e.g. `SCMP_ACT_KILL`).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum Action {
    /// `SCMP_ACT_KILL`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ACT_KILL"))]
    Kill,

    /// `SCMP_ACT_TRAP`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ACT_TRAP"))]
    Trap,

    /// `SCMP_ACT_ERRNO`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ACT_ERRNO"))]
    Errno,

    /// `SCMP_ACT_TRACE`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ACT_TRACE"))]
    Trace,

    /// `SCMP_ACT_ALLOW`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ACT_ALLOW"))]
    Allow,
}

/// List of architectures used for system calls.
///
/// When the feature `serde` is enabled, `Architecture` can be serialized to / deserialized from an
/// architecture name as defined in libseccomp (e.g. `SCMP_ARCH_X86`).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum Architecture {
    /// `SCMP_ARCH_X86`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_X86"))]
    X86,

    /// `SCMP_ARCH_X86_64`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_X86_64"))]
    X86_64,

    /// `SCMP_ARCH_X32`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_X32"))]
    X32,

    /// `SCMP_ARCH_ARM`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_ARM"))]
    Arm,

    /// `SCMP_ARCH_AARCH64`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_AARCH64"))]
    Aarch64,

    /// `SCMP_ARCH_MIPS`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_MIPS"))]
    Mips,

    /// `SCMP_ARCH_MIPS64`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_MIPS64"))]
    Mips64,

    /// `SCMP_ARCH_MIPS64N32`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_MIPS64N32"))]
    Mips64n32,

    /// `SCMP_ARCH_MIPSEL`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_MIPSEL"))]
    Mipsel,

    /// `SCMP_ARCH_MIPSEL64`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_MIPSEL64"))]
    Mipsel64,

    /// `SCMP_ARCH_MIPSEL64N32`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_MIPSEL64N32"))]
    Mipsel64n32,

    /// `SCMP_ARCH_PPC`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_PPC"))]
    Ppc,

    /// `SCMP_ARCH_PPC64`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_PPC64"))]
    Ppc64,

    /// `SCMP_ARCH_PPC64LE`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_PPC64LE"))]
    Ppc64le,

    /// `SCMP_ARCH_S390`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_S390"))]
    S390,

    /// `SCMP_ARCH_S390X`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_S390X"))]
    S390X,

    /// `SCMP_ARCH_PARISC`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_PARISC"))]
    Parisc,

    /// `SCMP_ARCH_PARISC64`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_ARCH_PARISC64"))]
    Parisc64,
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
///
/// When the feature `serde` is enabled, `SyscallCmp` can be serialized to / deserialized from an
/// comparator name as defined in libseccomp (e.g. `SCMP_CMP_NE`).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum SyscallCmp {
    /// `SCMP_CMP_NE`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_CMP_NE"))]
    Ne,

    /// `SCMP_CMP_LT`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_CMP_LT"))]
    Lt,

    /// `SCMP_CMP_LE`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_CMP_LE"))]
    Le,

    /// `SCMP_CMP_EQ`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_CMP_EQ"))]
    Eq,

    /// `SCMP_CMP_GE`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_CMP_GE"))]
    Ge,

    /// `SCMP_CMP_GT`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_CMP_GT"))]
    Gt,

    /// `SCMP_CMP_MASKED_EQ`
    #[cfg_attr(feature = "serde", serde(rename = "SCMP_CMP_MASKED_EQ"))]
    MaskedEq,
}
