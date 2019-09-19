use std::path::PathBuf;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Linux {
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub namespaces: Vec<Namespace>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub uid_mappings: Vec<IdMapping>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub gid_mappings: Vec<IdMapping>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub devices: Vec<Device>,
    // pub cgroup_path: Option<PathBuf>,
    // pub resources: Option<Resources>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Namespace {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: String,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum NamespaceType {
    Pid,
    Network,
    Mount,
    Ipc,
    Uts,
    User,
    Cgroup,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct IdMapping {
    pub host_id: u32,
    pub container_id: u32,
    pub size: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Device {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: DeviceType,

    pub path: PathBuf,

    pub major: Option<i64>,
    pub minor: Option<i64>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub file_mode: Option<u32>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub uid: Option<u32>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub gid: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum DeviceType {
    C,
    B,
    U,
    P,
}
