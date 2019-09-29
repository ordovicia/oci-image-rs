//! Resource limits for the container forced by cgroups.
//!
//! For more information about cgroups, see the [kernel docs].
//!
//! [kernel docs]: https://www.kernel.org/doc/Documentation/cgroup-v1/cgroups.txt

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Resource limits for a container forced by cgroups.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Resources {
    /// Device whitelist.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub devices: Vec<Device>,

    /// Limits on the container's memory usage.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub memory: Option<Memory>,

    /// Limits on the container's CPU usage.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cpu: Option<Cpu>,

    /// Represents a cgroup `blkio` subsystems for the container.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "blockIO", skip_serializing_if = "Option::is_none")
    )]
    pub block_io: Option<BlockIo>,

    /// Limits on the container's hugepage TLB usage.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub hugepage_limits: Vec<HugepageLimit>,

    /// Represents a cgroup subsystems `net_cls` and `net_prio` for the container.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub network: Option<Network>,

    /// Represents a cgroup `pids` subsystems for the container.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pids: Option<Pids>,
}

/// Device whitelist.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Device {
    /// Whether the entry is allowed or denied.
    pub allow: bool,

    /// Type of device. Unset values mean "all", mapping to `a`.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "type", skip_serializing_if = "Option::is_none")
    )]
    pub type_: Option<DeviceType>,

    /// Major number for the device. Unset values mean "all", mapping to `*` in the filesystem API.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub major: Option<i64>,

    /// Minor number for the device. Unset values mean "all", mapping to `*` in the filesystem API.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub minor: Option<i64>,

    /// Permission for the device. Composition of `r` (read), `w` (write), and `m` (mknod).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub access: Option<String>, // TODO: User proper type?
}

/// Types of devices.
///
/// When the feature `serde` is enabled, `DeviceType` can be serialized to / deserialized from a
/// single character representing the device type (i.e. `a`, `c`, or `b`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeviceType {
    /// Both character device and block device (type `a`).
    #[cfg_attr(feature = "serde", serde(rename = "a"))]
    All,

    /// Character device (type `c`).
    #[cfg_attr(feature = "serde", serde(rename = "c"))]
    Char,

    /// Block device (type `b`).
    #[cfg_attr(feature = "serde", serde(rename = "b"))]
    Block,
}

/// Limits on a container's memory usage.
///
/// Values for memory specify the limit in bytes, or `-1` for unlimited memory usage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Memory {
    /// Limit on memory usage.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub limit: Option<i64>,

    /// Soft limit on memory usage.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub reservation: Option<i64>,

    /// Limit on memory + swap usage.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub swap: Option<i64>,

    /// Hard limit on usage of kernel memory.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub kernel: Option<i64>,

    /// Hard limit on usage of kernel TCP buffer memory.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "kernelTCP", skip_serializing_if = "Option::is_none")
    )]
    pub kernel_tcp: Option<i64>,

    /// Swappiness parameter. Values are from 0 to 100.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub swappiness: Option<u64>,

    /// Whether to disable the OOM killer for the container.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "disableOOMKiller", skip_serializing_if = "Option::is_none")
    )]
    pub disable_oom_killer: Option<bool>,
}

/// Limits on a container's CPU usage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Cpu {
    /// Relative share of CPU time available to the cgroup.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub shares: Option<u64>,

    /// Total amount of time in microseconds for which the cgroup can run during one `period`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub quota: Option<i64>,

    /// Period of time in microseconds for how regularly the cgroup's access to CPU resources
    /// should be reallocated.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub period: Option<u64>,

    /// period of time in microseconds for the longest continuous period in which the cgroup have
    /// access to CPU resources.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub realtime_runtime: Option<i64>,

    /// Same as `period` but applies to realtime scheduler only.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub realtime_period: Option<u64>,

    /// List of CPUs the container will run on.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cpus: Option<String>, // TODO: Use proper type?

    /// List of memory nodes the container will use.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mems: Option<String>, // TODO: Use proper type?
}

/// Represents a cgroup `blkio` subsystems for a container.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct BlockIo {
    /// Per-cgroup weight.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub weight: Option<u16>,

    /// How much weight the cgroup has while competing with its child cgroups.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub leaf_weight: Option<u16>,

    /// Per-device weight.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub weight_device: Vec<DeviceWeight>,

    /// Per-device bandwidth rate limits for reading block devices in terms of bps.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub throttle_read_bps_device: Vec<DeviceThrottle>,

    /// Per-device bandwidth rate limits for writing to block devices in terms of bps.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub throttle_write_bps_device: Vec<DeviceThrottle>,

    /// Per-device I/O rate limits for reading block devices.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "throttleReadIOPSDevice",
            skip_serializing_if = "Vec::is_empty",
            default
        )
    )]
    pub throttle_read_iops_device: Vec<DeviceThrottle>,

    /// Per-device I/O rate limits for writing to block devices.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "throttleWriteIOPSDevice",
            skip_serializing_if = "Vec::is_empty",
            default
        )
    )]
    pub throttle_write_iops_device: Vec<DeviceThrottle>,
}

/// Per-device weight for block I/O.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct DeviceWeight {
    /// Major number for the device.
    pub major: i64,

    /// Minor number for the device.
    pub minor: i64,

    /// Bandwidth weight for the device.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub weight: Option<u16>,

    /// Bandwidth weight for the device while competing with the cgroup's child cgroups.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub leaf_weight: Option<u16>,
}

/// Per-device bandwidth or I/O rate limits.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceThrottle {
    /// Major number for the device.
    pub major: i64,

    /// Minor number for the device.
    pub minor: i64,

    /// Rate limit. The unit is bps for bandwidth rate, or ops for I/O rate.
    pub rate: u64,
}

/// Limits on a container's hugepage TLB usage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct HugepageLimit {
    /// Hugepage size (e.g. `2MB`).
    pub page_size: String, // TODO: use proper type?

    /// Limit in bytes.
    pub limit: u64,
}

/// Represents a cgroup subsystems `net_cls` and `net_prio` for a container.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Network {
    /// Network class ID with which the cgroup's network packets will be tagged.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "classID", skip_serializing_if = "Option::is_none")
    )]
    pub class_id: Option<u32>,

    /// List of priorities assigned to traffic originating from the cgroup.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub priorities: Vec<NetworkPriority>,
}

/// Network priority assigned to traffic originating from a cgroup.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NetworkPriority {
    /// Interface name in the runtime network namespace.
    pub name: String,

    /// Priority applied to the interface.
    pub priority: u32,
}

/// Represents a cgroup `pids` subsystems for a container.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pids {
    /// Maximum number of tasks in the cgroup.
    pub limit: i64,
}
