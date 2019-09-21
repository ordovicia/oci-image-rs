#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Seccomp {
    pub default_action: Action,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub architectures: Vec<Architecture>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub syscalls: Vec<Syscall>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum Action {
    ScmpActKill,
    ScmpActTrap,
    ScmpActErrno,
    ScmpActTrace,
    ScmpActAllow,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Syscall {
    pub names: Vec<String>,
    pub action: Action,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub args: Vec<SyscallArg>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct SyscallArg {
    pub index: usize,
    pub value: u64,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub value_two: Option<u64>,

    pub op: SyscallArgOp,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum SyscallArgOp {
    ScmpCmpNe,
    ScmpCmpLt,
    ScmpCmpLe,
    ScmpCmpEq,
    ScmpCmpGe,
    ScmpCmpGt,
    ScmpCmpMaskedEq,
}
