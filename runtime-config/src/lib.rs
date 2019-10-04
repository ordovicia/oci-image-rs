//! OCI runtime config schema in Rust.
//!
//! Schema in this crate is based on the v1.0.1 release of the OCI runtime config spec. See the
//! [OCI runtime spec] for more information about the spec.
//!
//! [OCI runtime spec]: https://github.com/opencontainers/runtime-spec/releases/tag/v1.0.1

#![warn(
    future_incompatible,
    missing_docs,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused
)]

pub mod config;
pub mod linux;

pub use config::Config;
pub use linux::LinuxConfig;

/// Version of OCI runtime spec on which this crate is based.
pub const OCI_VERSION: &str = "1.0.1";

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_config_ser() {
        #![allow(clippy::unreadable_literal)]

        let config = Config {
            oci_version: String::from("0.5.0-dev"),
            root: Some(config::Root {
                path: PathBuf::from("rootfs"),
                readonly: Some(true),
            }),
            mounts: vec![
                config::Mount {
                    destination: PathBuf::from("/proc"),
                    type_: Some(String::from("proc")),
                    source: Some(PathBuf::from("proc")),
                    options: vec![],
                },
                config::Mount {
                    destination: PathBuf::from("/dev"),
                    type_: Some(String::from("tmpfs")),
                    source: Some(PathBuf::from("tmpfs")),
                    options: vec![
                        String::from("nosuid"),
                        String::from("strictatime"),
                        String::from("mode=755"),
                        String::from("size=65536k"),
                    ],
                },
                config::Mount {
                    destination: PathBuf::from("/dev/pts"),
                    type_: Some(String::from("devpts")),
                    source: Some(PathBuf::from("devpts")),
                    options: vec![
                        String::from("nosuid"),
                        String::from("noexec"),
                        String::from("newinstance"),
                        String::from("ptmxmode=0666"),
                        String::from("mode=0620"),
                        String::from("gid=5"),
                    ],
                },
                config::Mount {
                    destination: PathBuf::from("/dev/shm"),
                    type_: Some(String::from("tmpfs")),
                    source: Some(PathBuf::from("shm")),
                    options: vec![
                        String::from("nosuid"),
                        String::from("noexec"),
                        String::from("nodev"),
                        String::from("mode=1777"),
                        String::from("size=65536k"),
                    ],
                },
                config::Mount {
                    destination: PathBuf::from("/dev/mqueue"),
                    type_: Some(String::from("mqueue")),
                    source: Some(PathBuf::from("mqueue")),
                    options: vec![
                        String::from("nosuid"),
                        String::from("noexec"),
                        String::from("nodev"),
                    ],
                },
                config::Mount {
                    destination: PathBuf::from("/sys"),
                    type_: Some(String::from("sysfs")),
                    source: Some(PathBuf::from("sysfs")),
                    options: vec![
                        String::from("nosuid"),
                        String::from("noexec"),
                        String::from("nodev"),
                    ],
                },
                config::Mount {
                    destination: PathBuf::from("/sys/fs/cgroup"),
                    type_: Some(String::from("cgroup")),
                    source: Some(PathBuf::from("cgroup")),
                    options: vec![
                        String::from("nosuid"),
                        String::from("noexec"),
                        String::from("nodev"),
                        String::from("relatime"),
                        String::from("ro"),
                    ],
                },
            ],
            process: Some(config::Process {
                terminal: Some(true),
                console_size: None,
                user: config::User {
                    uid: 1,
                    gid: 1,
                    additional_gids: vec![5, 6],
                    username: None,
                },
                cwd: PathBuf::from("/"),
                env: vec![
                    String::from(
                        "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                    ),
                    String::from("TERM=xterm"),
                ],
                args: vec![String::from("sh")],
                rlimits: vec![
                    config::Rlimit {
                        type_: String::from("RLIMIT_CORE"),
                        hard: 1024,
                        soft: 1024,
                    },
                    config::Rlimit {
                        type_: String::from("RLIMIT_NOFILE"),
                        hard: 1024,
                        soft: 1024,
                    },
                ],
                apparmor_profile: Some(String::from("acme_secure_profile")),
                capabilities: Some(config::Capabilities {
                    bounding: vec![
                        config::Capability::AuditWrite,
                        config::Capability::Kill,
                        config::Capability::NetBindService,
                    ],
                    permitted: vec![
                        config::Capability::AuditWrite,
                        config::Capability::Kill,
                        config::Capability::NetBindService,
                    ],
                    inheritable: vec![
                        config::Capability::AuditWrite,
                        config::Capability::Kill,
                        config::Capability::NetBindService,
                    ],
                    effective: vec![config::Capability::AuditWrite, config::Capability::Kill],
                    ambient: vec![config::Capability::NetBindService],
                }),
                no_new_privileges: Some(true),
                oom_score_adj: Some(100),
                selinux_label: Some(String::from(
                    "system_u:system_r:svirt_lxc_net_t:s0:c124,c675",
                )),
            }),
            hostname: Some(String::from("slartibartfast")),
            hooks: Some(config::Hooks {
                prestart: vec![
                    config::Hook {
                        path: PathBuf::from("/usr/bin/fix-mounts"),
                        args: vec![
                            String::from("fix-mounts"),
                            String::from("arg1"),
                            String::from("arg2"),
                        ],
                        env: vec![String::from("key1=value1")],
                        timeout: None,
                    },
                    config::Hook {
                        path: PathBuf::from("/usr/bin/setup-network"),
                        args: vec![],
                        env: vec![],
                        timeout: None,
                    },
                ],
                poststart: vec![config::Hook {
                    path: PathBuf::from("/usr/bin/notify-start"),
                    args: vec![],
                    env: vec![],
                    timeout: Some(5),
                }],
                poststop: vec![config::Hook {
                    path: PathBuf::from("/usr/sbin/cleanup.sh"),
                    args: vec![String::from("cleanup.sh"), String::from("-f")],
                    env: vec![],
                    timeout: None,
                }],
            }),
            annotations: [
                (String::from("com.example.key1"), String::from("value1")),
                // (String::from("com.example.key2"), String::from("value2")),
            ]
            .iter()
            .cloned()
            .collect(),
            linux: Some(linux::LinuxConfig {
                namespaces: vec![
                    linux::Namespace {
                        type_: linux::NamespaceType::Pid,
                        path: None,
                    },
                    linux::Namespace {
                        type_: linux::NamespaceType::Network,
                        path: None,
                    },
                    linux::Namespace {
                        type_: linux::NamespaceType::Ipc,
                        path: None,
                    },
                    linux::Namespace {
                        type_: linux::NamespaceType::Uts,
                        path: None,
                    },
                    linux::Namespace {
                        type_: linux::NamespaceType::Mount,
                        path: None,
                    },
                    linux::Namespace {
                        type_: linux::NamespaceType::User,
                        path: None,
                    },
                    linux::Namespace {
                        type_: linux::NamespaceType::Cgroup,
                        path: None,
                    },
                ],
                uid_mappings: vec![linux::UserNamespaceMappings {
                    host_id: 1000,
                    container_id: 0,
                    size: 32000,
                }],
                gid_mappings: vec![linux::UserNamespaceMappings {
                    host_id: 1000,
                    container_id: 0,
                    size: 32000,
                }],
                devices: vec![
                    linux::Device {
                        type_: linux::DeviceType::Character,
                        path: PathBuf::from("/dev/fuse"),
                        major: Some(10),
                        minor: Some(229),
                        file_mode: Some(438),
                        uid: Some(0),
                        gid: Some(0),
                    },
                    linux::Device {
                        type_: linux::DeviceType::Block,
                        path: PathBuf::from("/dev/sda"),
                        major: Some(8),
                        minor: Some(0),
                        file_mode: Some(432),
                        uid: Some(0),
                        gid: Some(0),
                    },
                ],
                cgroups_path: Some(PathBuf::from("/myRuntime/myContainer")),
                resources: Some(linux::Resources {
                    devices: vec![
                        linux::resources::Device {
                            allow: false,
                            type_: None,
                            major: None,
                            minor: None,
                            access: Some(String::from("rwm")),
                        },
                        linux::resources::Device {
                            allow: true,
                            type_: Some(linux::resources::DeviceType::Character),
                            major: Some(10),
                            minor: Some(229),
                            access: Some(String::from("rw")),
                        },
                        linux::resources::Device {
                            allow: true,
                            type_: Some(linux::resources::DeviceType::Block),
                            major: Some(8),
                            minor: Some(0),
                            access: Some(String::from("r")),
                        },
                    ],
                    memory: Some(linux::resources::Memory {
                        limit: Some(536870912),
                        reservation: Some(536870912),
                        swap: Some(536870912),
                        kernel: Some(-1),
                        kernel_tcp: Some(-1),
                        swappiness: Some(0),
                        disable_oom_killer: Some(false),
                    }),
                    cpu: Some(linux::resources::Cpu {
                        shares: Some(1024),
                        quota: Some(1000000),
                        period: Some(500000),
                        realtime_runtime: Some(950000),
                        realtime_period: Some(1000000),
                        cpus: Some(String::from("2-3")),
                        mems: Some(String::from("0-7")),
                    }),
                    block_io: Some(linux::resources::BlockIo {
                        weight: Some(10),
                        leaf_weight: Some(10),
                        weight_device: vec![
                            linux::resources::DeviceWeight {
                                major: 8,
                                minor: 0,
                                weight: Some(500),
                                leaf_weight: Some(300),
                            },
                            linux::resources::DeviceWeight {
                                major: 8,
                                minor: 16,
                                weight: Some(500),
                                leaf_weight: None,
                            },
                        ],
                        throttle_read_bps_device: vec![linux::resources::DeviceThrottle {
                            major: 8,
                            minor: 0,
                            rate: 600,
                        }],
                        throttle_write_bps_device: vec![],
                        throttle_read_iops_device: vec![],
                        throttle_write_iops_device: vec![linux::resources::DeviceThrottle {
                            major: 8,
                            minor: 16,
                            rate: 300,
                        }],
                    }),
                    hugepage_limits: vec![linux::resources::HugepageLimit {
                        page_size: String::from("2MB"),
                        limit: 9223372036854772000,
                    }],
                    network: Some(linux::resources::Network {
                        class_id: Some(1048577),
                        priorities: vec![
                            linux::resources::NetworkPriority {
                                name: String::from("eth0"),
                                priority: 500,
                            },
                            linux::resources::NetworkPriority {
                                name: String::from("eth1"),
                                priority: 1000,
                            },
                        ],
                    }),
                    pids: Some(linux::resources::Pids { limit: 32771 }),
                }),
                intel_rdt: None,
                sysctl: [
                    (String::from("net.ipv4.ip_forward"), String::from("1")),
                    // (String::from("net.core.somaxconn"), String::from("256")),
                ]
                .iter()
                .cloned()
                .collect(),
                seccomp: Some(linux::Seccomp {
                    default_action: linux::seccomp::Action::Allow,
                    architectures: vec![
                        linux::seccomp::Architecture::X86,
                        linux::seccomp::Architecture::X32,
                    ],
                    syscalls: vec![linux::seccomp::Syscall {
                        names: vec![String::from("getcwd"), String::from("chmod")],
                        action: linux::seccomp::Action::Errno,
                        args: vec![],
                    }],
                }),
                rootfs_propagation: Some(linux::RootfsPropagation::Slave),
                masked_paths: vec![
                    PathBuf::from("/proc/kcore"),
                    PathBuf::from("/proc/latency_stats"),
                    PathBuf::from("/proc/timer_stats"),
                    PathBuf::from("/proc/sched_debug"),
                ],
                readonly_paths: vec![
                    PathBuf::from("/proc/asound"),
                    PathBuf::from("/proc/bus"),
                    PathBuf::from("/proc/fs"),
                    PathBuf::from("/proc/irq"),
                    PathBuf::from("/proc/sys"),
                    PathBuf::from("/proc/sysrq-trigger"),
                ],
                mount_label: Some(String::from(
                    "system_u:object_r:svirt_sandbox_file_t:s0:c715,c811",
                )),
            }),
        };

        assert_eq!(serde_json::to_string_pretty(&config).unwrap(), JSON_SER);
    }

    #[test]
    fn test_config_deser() {
        #![allow(clippy::unreadable_literal)]

        let config: Config = serde_json::from_str(JSON_DESER).unwrap();

        assert_eq!(
            config,
            Config {
                oci_version: String::from("0.5.0-dev"),
                root: Some(config::Root {
                    path: PathBuf::from("rootfs"),
                    readonly: Some(true),
                }),
                mounts: vec![
                    config::Mount {
                        destination: PathBuf::from("/proc"),
                        type_: Some(String::from("proc")),
                        source: Some(PathBuf::from("proc")),
                        options: vec![],
                    },
                    config::Mount {
                        destination: PathBuf::from("/dev"),
                        type_: Some(String::from("tmpfs")),
                        source: Some(PathBuf::from("tmpfs")),
                        options: vec![
                            String::from("nosuid"),
                            String::from("strictatime"),
                            String::from("mode=755"),
                            String::from("size=65536k"),
                        ],
                    },
                    config::Mount {
                        destination: PathBuf::from("/dev/pts"),
                        type_: Some(String::from("devpts")),
                        source: Some(PathBuf::from("devpts")),
                        options: vec![
                            String::from("nosuid"),
                            String::from("noexec"),
                            String::from("newinstance"),
                            String::from("ptmxmode=0666"),
                            String::from("mode=0620"),
                            String::from("gid=5"),
                        ],
                    },
                    config::Mount {
                        destination: PathBuf::from("/dev/shm"),
                        type_: Some(String::from("tmpfs")),
                        source: Some(PathBuf::from("shm")),
                        options: vec![
                            String::from("nosuid"),
                            String::from("noexec"),
                            String::from("nodev"),
                            String::from("mode=1777"),
                            String::from("size=65536k"),
                        ],
                    },
                    config::Mount {
                        destination: PathBuf::from("/dev/mqueue"),
                        type_: Some(String::from("mqueue")),
                        source: Some(PathBuf::from("mqueue")),
                        options: vec![
                            String::from("nosuid"),
                            String::from("noexec"),
                            String::from("nodev"),
                        ],
                    },
                    config::Mount {
                        destination: PathBuf::from("/sys"),
                        type_: Some(String::from("sysfs")),
                        source: Some(PathBuf::from("sysfs")),
                        options: vec![
                            String::from("nosuid"),
                            String::from("noexec"),
                            String::from("nodev"),
                        ],
                    },
                    config::Mount {
                        destination: PathBuf::from("/sys/fs/cgroup"),
                        type_: Some(String::from("cgroup")),
                        source: Some(PathBuf::from("cgroup")),
                        options: vec![
                            String::from("nosuid"),
                            String::from("noexec"),
                            String::from("nodev"),
                            String::from("relatime"),
                            String::from("ro"),
                        ],
                    },
                ],
                process: Some(config::Process {
                    terminal: Some(true),
                    console_size: None,
                    user: config::User {
                        uid: 1,
                        gid: 1,
                        additional_gids: vec![5, 6],
                        username: None,
                    },
                    cwd: PathBuf::from("/"),
                    env: vec![
                        String::from(
                            "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                        ),
                        String::from("TERM=xterm"),
                    ],
                    args: vec![String::from("sh")],
                    rlimits: vec![
                        config::Rlimit {
                            type_: String::from("RLIMIT_CORE"),
                            hard: 1024,
                            soft: 1024,
                        },
                        config::Rlimit {
                            type_: String::from("RLIMIT_NOFILE"),
                            hard: 1024,
                            soft: 1024,
                        },
                    ],
                    apparmor_profile: Some(String::from("acme_secure_profile")),
                    capabilities: Some(config::Capabilities {
                        bounding: vec![
                            config::Capability::AuditWrite,
                            config::Capability::Kill,
                            config::Capability::NetBindService,
                        ],
                        permitted: vec![
                            config::Capability::AuditWrite,
                            config::Capability::Kill,
                            config::Capability::NetBindService,
                        ],
                        inheritable: vec![
                            config::Capability::AuditWrite,
                            config::Capability::Kill,
                            config::Capability::NetBindService,
                        ],
                        effective: vec![config::Capability::AuditWrite, config::Capability::Kill],
                        ambient: vec![config::Capability::NetBindService],
                    }),
                    no_new_privileges: Some(true),
                    oom_score_adj: Some(100),
                    selinux_label: Some(String::from(
                        "system_u:system_r:svirt_lxc_net_t:s0:c124,c675",
                    )),
                }),
                hostname: Some(String::from("slartibartfast")),
                hooks: Some(config::Hooks {
                    prestart: vec![
                        config::Hook {
                            path: PathBuf::from("/usr/bin/fix-mounts"),
                            args: vec![
                                String::from("fix-mounts"),
                                String::from("arg1"),
                                String::from("arg2"),
                            ],
                            env: vec![String::from("key1=value1")],
                            timeout: None,
                        },
                        config::Hook {
                            path: PathBuf::from("/usr/bin/setup-network"),
                            args: vec![],
                            env: vec![],
                            timeout: None,
                        },
                    ],
                    poststart: vec![config::Hook {
                        path: PathBuf::from("/usr/bin/notify-start"),
                        args: vec![],
                        env: vec![],
                        timeout: Some(5),
                    }],
                    poststop: vec![config::Hook {
                        path: PathBuf::from("/usr/sbin/cleanup.sh"),
                        args: vec![String::from("cleanup.sh"), String::from("-f")],
                        env: vec![],
                        timeout: None,
                    }],
                }),
                annotations: [
                    (String::from("com.example.key1"), String::from("value1")),
                    (String::from("com.example.key2"), String::from("value2")),
                ]
                .iter()
                .cloned()
                .collect(),
                linux: Some(linux::LinuxConfig {
                    namespaces: vec![
                        linux::Namespace {
                            type_: linux::NamespaceType::Pid,
                            path: None,
                        },
                        linux::Namespace {
                            type_: linux::NamespaceType::Network,
                            path: None,
                        },
                        linux::Namespace {
                            type_: linux::NamespaceType::Ipc,
                            path: None,
                        },
                        linux::Namespace {
                            type_: linux::NamespaceType::Uts,
                            path: None,
                        },
                        linux::Namespace {
                            type_: linux::NamespaceType::Mount,
                            path: None,
                        },
                        linux::Namespace {
                            type_: linux::NamespaceType::User,
                            path: None,
                        },
                        linux::Namespace {
                            type_: linux::NamespaceType::Cgroup,
                            path: None,
                        },
                    ],
                    uid_mappings: vec![linux::UserNamespaceMappings {
                        host_id: 1000,
                        container_id: 0,
                        size: 32000,
                    }],
                    gid_mappings: vec![linux::UserNamespaceMappings {
                        host_id: 1000,
                        container_id: 0,
                        size: 32000,
                    }],
                    devices: vec![
                        linux::Device {
                            type_: linux::DeviceType::Character,
                            path: PathBuf::from("/dev/fuse"),
                            major: Some(10),
                            minor: Some(229),
                            file_mode: Some(438),
                            uid: Some(0),
                            gid: Some(0),
                        },
                        linux::Device {
                            type_: linux::DeviceType::Block,
                            path: PathBuf::from("/dev/sda"),
                            major: Some(8),
                            minor: Some(0),
                            file_mode: Some(432),
                            uid: Some(0),
                            gid: Some(0),
                        },
                    ],
                    cgroups_path: Some(PathBuf::from("/myRuntime/myContainer")),
                    resources: Some(linux::Resources {
                        devices: vec![
                            linux::resources::Device {
                                allow: false,
                                type_: None,
                                major: None,
                                minor: None,
                                access: Some(String::from("rwm")),
                            },
                            linux::resources::Device {
                                allow: true,
                                type_: Some(linux::resources::DeviceType::Character),
                                major: Some(10),
                                minor: Some(229),
                                access: Some(String::from("rw")),
                            },
                            linux::resources::Device {
                                allow: true,
                                type_: Some(linux::resources::DeviceType::Block),
                                major: Some(8),
                                minor: Some(0),
                                access: Some(String::from("r")),
                            },
                        ],
                        memory: Some(linux::resources::Memory {
                            limit: Some(536870912),
                            reservation: Some(536870912),
                            swap: Some(536870912),
                            kernel: Some(-1),
                            kernel_tcp: Some(-1),
                            swappiness: Some(0),
                            disable_oom_killer: Some(false),
                        }),
                        cpu: Some(linux::resources::Cpu {
                            shares: Some(1024),
                            quota: Some(1000000),
                            period: Some(500000),
                            realtime_runtime: Some(950000),
                            realtime_period: Some(1000000),
                            cpus: Some(String::from("2-3")),
                            mems: Some(String::from("0-7")),
                        }),
                        block_io: Some(linux::resources::BlockIo {
                            weight: Some(10),
                            leaf_weight: Some(10),
                            weight_device: vec![
                                linux::resources::DeviceWeight {
                                    major: 8,
                                    minor: 0,
                                    weight: Some(500),
                                    leaf_weight: Some(300),
                                },
                                linux::resources::DeviceWeight {
                                    major: 8,
                                    minor: 16,
                                    weight: Some(500),
                                    leaf_weight: None,
                                },
                            ],
                            throttle_read_bps_device: vec![linux::resources::DeviceThrottle {
                                major: 8,
                                minor: 0,
                                rate: 600,
                            }],
                            throttle_write_bps_device: vec![],
                            throttle_read_iops_device: vec![],
                            throttle_write_iops_device: vec![linux::resources::DeviceThrottle {
                                major: 8,
                                minor: 16,
                                rate: 300,
                            }],
                        }),
                        hugepage_limits: vec![linux::resources::HugepageLimit {
                            page_size: String::from("2MB"),
                            limit: 9223372036854772000,
                        }],
                        network: Some(linux::resources::Network {
                            class_id: Some(1048577),
                            priorities: vec![
                                linux::resources::NetworkPriority {
                                    name: String::from("eth0"),
                                    priority: 500,
                                },
                                linux::resources::NetworkPriority {
                                    name: String::from("eth1"),
                                    priority: 1000,
                                },
                            ],
                        }),
                        pids: Some(linux::resources::Pids { limit: 32771 }),
                    }),
                    intel_rdt: None,
                    sysctl: [
                        (String::from("net.ipv4.ip_forward"), String::from("1")),
                        (String::from("net.core.somaxconn"), String::from("256")),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                    seccomp: Some(linux::Seccomp {
                        default_action: linux::seccomp::Action::Allow,
                        architectures: vec![
                            linux::seccomp::Architecture::X86,
                            linux::seccomp::Architecture::X32,
                        ],
                        syscalls: vec![linux::seccomp::Syscall {
                            names: vec![String::from("getcwd"), String::from("chmod")],
                            action: linux::seccomp::Action::Errno,
                            args: vec![],
                        }],
                    }),
                    rootfs_propagation: Some(linux::RootfsPropagation::Slave),
                    masked_paths: vec![
                        PathBuf::from("/proc/kcore"),
                        PathBuf::from("/proc/latency_stats"),
                        PathBuf::from("/proc/timer_stats"),
                        PathBuf::from("/proc/sched_debug"),
                    ],
                    readonly_paths: vec![
                        PathBuf::from("/proc/asound"),
                        PathBuf::from("/proc/bus"),
                        PathBuf::from("/proc/fs"),
                        PathBuf::from("/proc/irq"),
                        PathBuf::from("/proc/sys"),
                        PathBuf::from("/proc/sysrq-trigger"),
                    ],
                    mount_label: Some(String::from(
                        "system_u:object_r:svirt_sandbox_file_t:s0:c715,c811",
                    )),
                }),
            }
        );
    }

    // Example from https://github.com/opencontainers/runtime-spec/blob/v1.0.1/config.md

    const JSON_SER: &str = r#"{
  "ociVersion": "0.5.0-dev",
  "root": {
    "path": "rootfs",
    "readonly": true
  },
  "mounts": [
    {
      "destination": "/proc",
      "type": "proc",
      "source": "proc"
    },
    {
      "destination": "/dev",
      "type": "tmpfs",
      "source": "tmpfs",
      "options": [
        "nosuid",
        "strictatime",
        "mode=755",
        "size=65536k"
      ]
    },
    {
      "destination": "/dev/pts",
      "type": "devpts",
      "source": "devpts",
      "options": [
        "nosuid",
        "noexec",
        "newinstance",
        "ptmxmode=0666",
        "mode=0620",
        "gid=5"
      ]
    },
    {
      "destination": "/dev/shm",
      "type": "tmpfs",
      "source": "shm",
      "options": [
        "nosuid",
        "noexec",
        "nodev",
        "mode=1777",
        "size=65536k"
      ]
    },
    {
      "destination": "/dev/mqueue",
      "type": "mqueue",
      "source": "mqueue",
      "options": [
        "nosuid",
        "noexec",
        "nodev"
      ]
    },
    {
      "destination": "/sys",
      "type": "sysfs",
      "source": "sysfs",
      "options": [
        "nosuid",
        "noexec",
        "nodev"
      ]
    },
    {
      "destination": "/sys/fs/cgroup",
      "type": "cgroup",
      "source": "cgroup",
      "options": [
        "nosuid",
        "noexec",
        "nodev",
        "relatime",
        "ro"
      ]
    }
  ],
  "process": {
    "terminal": true,
    "user": {
      "uid": 1,
      "gid": 1,
      "additionalGids": [
        5,
        6
      ]
    },
    "cwd": "/",
    "env": [
      "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
      "TERM=xterm"
    ],
    "args": [
      "sh"
    ],
    "rlimits": [
      {
        "type": "RLIMIT_CORE",
        "soft": 1024,
        "hard": 1024
      },
      {
        "type": "RLIMIT_NOFILE",
        "soft": 1024,
        "hard": 1024
      }
    ],
    "apparmorProfile": "acme_secure_profile",
    "capabilities": {
      "effective": [
        "CAP_AUDIT_WRITE",
        "CAP_KILL"
      ],
      "bounding": [
        "CAP_AUDIT_WRITE",
        "CAP_KILL",
        "CAP_NET_BIND_SERVICE"
      ],
      "inheritable": [
        "CAP_AUDIT_WRITE",
        "CAP_KILL",
        "CAP_NET_BIND_SERVICE"
      ],
      "permitted": [
        "CAP_AUDIT_WRITE",
        "CAP_KILL",
        "CAP_NET_BIND_SERVICE"
      ],
      "ambient": [
        "CAP_NET_BIND_SERVICE"
      ]
    },
    "noNewPrivileges": true,
    "oomScoreAdj": 100,
    "selinuxLabel": "system_u:system_r:svirt_lxc_net_t:s0:c124,c675"
  },
  "hostname": "slartibartfast",
  "hooks": {
    "prestart": [
      {
        "path": "/usr/bin/fix-mounts",
        "args": [
          "fix-mounts",
          "arg1",
          "arg2"
        ],
        "env": [
          "key1=value1"
        ]
      },
      {
        "path": "/usr/bin/setup-network"
      }
    ],
    "poststart": [
      {
        "path": "/usr/bin/notify-start",
        "timeout": 5
      }
    ],
    "poststop": [
      {
        "path": "/usr/sbin/cleanup.sh",
        "args": [
          "cleanup.sh",
          "-f"
        ]
      }
    ]
  },
  "annotations": {
    "com.example.key1": "value1"
  },
  "linux": {
    "namespaces": [
      {
        "type": "pid"
      },
      {
        "type": "network"
      },
      {
        "type": "ipc"
      },
      {
        "type": "uts"
      },
      {
        "type": "mount"
      },
      {
        "type": "user"
      },
      {
        "type": "cgroup"
      }
    ],
    "uidMappings": [
      {
        "hostID": 1000,
        "containerID": 0,
        "size": 32000
      }
    ],
    "gidMappings": [
      {
        "hostID": 1000,
        "containerID": 0,
        "size": 32000
      }
    ],
    "devices": [
      {
        "type": "c",
        "path": "/dev/fuse",
        "major": 10,
        "minor": 229,
        "fileMode": 438,
        "uid": 0,
        "gid": 0
      },
      {
        "type": "b",
        "path": "/dev/sda",
        "major": 8,
        "minor": 0,
        "fileMode": 432,
        "uid": 0,
        "gid": 0
      }
    ],
    "cgroupsPath": "/myRuntime/myContainer",
    "resources": {
      "devices": [
        {
          "allow": false,
          "access": "rwm"
        },
        {
          "allow": true,
          "type": "c",
          "major": 10,
          "minor": 229,
          "access": "rw"
        },
        {
          "allow": true,
          "type": "b",
          "major": 8,
          "minor": 0,
          "access": "r"
        }
      ],
      "memory": {
        "limit": 536870912,
        "reservation": 536870912,
        "swap": 536870912,
        "kernel": -1,
        "kernelTCP": -1,
        "swappiness": 0,
        "disableOOMKiller": false
      },
      "cpu": {
        "shares": 1024,
        "quota": 1000000,
        "period": 500000,
        "realtimeRuntime": 950000,
        "realtimePeriod": 1000000,
        "cpus": "2-3",
        "mems": "0-7"
      },
      "blockIO": {
        "weight": 10,
        "leafWeight": 10,
        "weightDevice": [
          {
            "major": 8,
            "minor": 0,
            "weight": 500,
            "leafWeight": 300
          },
          {
            "major": 8,
            "minor": 16,
            "weight": 500
          }
        ],
        "throttleReadBpsDevice": [
          {
            "major": 8,
            "minor": 0,
            "rate": 600
          }
        ],
        "throttleWriteIOPSDevice": [
          {
            "major": 8,
            "minor": 16,
            "rate": 300
          }
        ]
      },
      "hugepageLimits": [
        {
          "pageSize": "2MB",
          "limit": 9223372036854772000
        }
      ],
      "network": {
        "classID": 1048577,
        "priorities": [
          {
            "name": "eth0",
            "priority": 500
          },
          {
            "name": "eth1",
            "priority": 1000
          }
        ]
      },
      "pids": {
        "limit": 32771
      }
    },
    "sysctl": {
      "net.ipv4.ip_forward": "1"
    },
    "seccomp": {
      "defaultAction": "SCMP_ACT_ALLOW",
      "architectures": [
        "SCMP_ARCH_X86",
        "SCMP_ARCH_X32"
      ],
      "syscalls": [
        {
          "names": [
            "getcwd",
            "chmod"
          ],
          "action": "SCMP_ACT_ERRNO"
        }
      ]
    },
    "rootfsPropagation": "slave",
    "maskedPaths": [
      "/proc/kcore",
      "/proc/latency_stats",
      "/proc/timer_stats",
      "/proc/sched_debug"
    ],
    "readonlyPaths": [
      "/proc/asound",
      "/proc/bus",
      "/proc/fs",
      "/proc/irq",
      "/proc/sys",
      "/proc/sysrq-trigger"
    ],
    "mountLabel": "system_u:object_r:svirt_sandbox_file_t:s0:c715,c811"
  }
}"#;

    const JSON_DESER: &str = r#"{
    "ociVersion": "0.5.0-dev",
    "process": {
        "terminal": true,
        "user": {
            "uid": 1,
            "gid": 1,
            "additionalGids": [
                5,
                6
            ]
        },
        "args": [
            "sh"
        ],
        "env": [
            "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
            "TERM=xterm"
        ],
        "cwd": "/",
        "capabilities": {
            "bounding": [
                "CAP_AUDIT_WRITE",
                "CAP_KILL",
                "CAP_NET_BIND_SERVICE"
            ],
            "permitted": [
                "CAP_AUDIT_WRITE",
                "CAP_KILL",
                "CAP_NET_BIND_SERVICE"
            ],
            "inheritable": [
                "CAP_AUDIT_WRITE",
                "CAP_KILL",
                "CAP_NET_BIND_SERVICE"
            ],
            "effective": [
                "CAP_AUDIT_WRITE",
                "CAP_KILL"
            ],
            "ambient": [
                "CAP_NET_BIND_SERVICE"
            ]
        },
        "rlimits": [
            {
                "type": "RLIMIT_CORE",
                "hard": 1024,
                "soft": 1024
            },
            {
                "type": "RLIMIT_NOFILE",
                "hard": 1024,
                "soft": 1024
            }
        ],
        "apparmorProfile": "acme_secure_profile",
        "oomScoreAdj": 100,
        "selinuxLabel": "system_u:system_r:svirt_lxc_net_t:s0:c124,c675",
        "noNewPrivileges": true
    },
    "root": {
        "path": "rootfs",
        "readonly": true
    },
    "hostname": "slartibartfast",
    "mounts": [
        {
            "destination": "/proc",
            "type": "proc",
            "source": "proc"
        },
        {
            "destination": "/dev",
            "type": "tmpfs",
            "source": "tmpfs",
            "options": [
                "nosuid",
                "strictatime",
                "mode=755",
                "size=65536k"
            ]
        },
        {
            "destination": "/dev/pts",
            "type": "devpts",
            "source": "devpts",
            "options": [
                "nosuid",
                "noexec",
                "newinstance",
                "ptmxmode=0666",
                "mode=0620",
                "gid=5"
            ]
        },
        {
            "destination": "/dev/shm",
            "type": "tmpfs",
            "source": "shm",
            "options": [
                "nosuid",
                "noexec",
                "nodev",
                "mode=1777",
                "size=65536k"
            ]
        },
        {
            "destination": "/dev/mqueue",
            "type": "mqueue",
            "source": "mqueue",
            "options": [
                "nosuid",
                "noexec",
                "nodev"
            ]
        },
        {
            "destination": "/sys",
            "type": "sysfs",
            "source": "sysfs",
            "options": [
                "nosuid",
                "noexec",
                "nodev"
            ]
        },
        {
            "destination": "/sys/fs/cgroup",
            "type": "cgroup",
            "source": "cgroup",
            "options": [
                "nosuid",
                "noexec",
                "nodev",
                "relatime",
                "ro"
            ]
        }
    ],
    "hooks": {
        "prestart": [
            {
                "path": "/usr/bin/fix-mounts",
                "args": [
                    "fix-mounts",
                    "arg1",
                    "arg2"
                ],
                "env": [
                    "key1=value1"
                ]
            },
            {
                "path": "/usr/bin/setup-network"
            }
        ],
        "poststart": [
            {
                "path": "/usr/bin/notify-start",
                "timeout": 5
            }
        ],
        "poststop": [
            {
                "path": "/usr/sbin/cleanup.sh",
                "args": [
                    "cleanup.sh",
                    "-f"
                ]
            }
        ]
    },
    "linux": {
        "devices": [
            {
                "path": "/dev/fuse",
                "type": "c",
                "major": 10,
                "minor": 229,
                "fileMode": 438,
                "uid": 0,
                "gid": 0
            },
            {
                "path": "/dev/sda",
                "type": "b",
                "major": 8,
                "minor": 0,
                "fileMode": 432,
                "uid": 0,
                "gid": 0
            }
        ],
        "uidMappings": [
            {
                "hostID": 1000,
                "containerID": 0,
                "size": 32000
            }
        ],
        "gidMappings": [
            {
                "hostID": 1000,
                "containerID": 0,
                "size": 32000
            }
        ],
        "sysctl": {
            "net.ipv4.ip_forward": "1",
            "net.core.somaxconn": "256"
        },
        "cgroupsPath": "/myRuntime/myContainer",
        "resources": {
            "network": {
                "classID": 1048577,
                "priorities": [
                    {
                        "name": "eth0",
                        "priority": 500
                    },
                    {
                        "name": "eth1",
                        "priority": 1000
                    }
                ]
            },
            "pids": {
                "limit": 32771
            },
            "hugepageLimits": [
                {
                    "pageSize": "2MB",
                    "limit": 9223372036854772000
                }
            ],
            "memory": {
                "limit": 536870912,
                "reservation": 536870912,
                "swap": 536870912,
                "kernel": -1,
                "kernelTCP": -1,
                "swappiness": 0,
                "disableOOMKiller": false
            },
            "cpu": {
                "shares": 1024,
                "quota": 1000000,
                "period": 500000,
                "realtimeRuntime": 950000,
                "realtimePeriod": 1000000,
                "cpus": "2-3",
                "mems": "0-7"
            },
            "devices": [
                {
                    "allow": false,
                    "access": "rwm"
                },
                {
                    "allow": true,
                    "type": "c",
                    "major": 10,
                    "minor": 229,
                    "access": "rw"
                },
                {
                    "allow": true,
                    "type": "b",
                    "major": 8,
                    "minor": 0,
                    "access": "r"
                }
            ],
            "blockIO": {
                "weight": 10,
                "leafWeight": 10,
                "weightDevice": [
                    {
                        "major": 8,
                        "minor": 0,
                        "weight": 500,
                        "leafWeight": 300
                    },
                    {
                        "major": 8,
                        "minor": 16,
                        "weight": 500
                    }
                ],
                "throttleReadBpsDevice": [
                    {
                        "major": 8,
                        "minor": 0,
                        "rate": 600
                    }
                ],
                "throttleWriteIOPSDevice": [
                    {
                        "major": 8,
                        "minor": 16,
                        "rate": 300
                    }
                ]
            }
        },
        "rootfsPropagation": "slave",
        "seccomp": {
            "defaultAction": "SCMP_ACT_ALLOW",
            "architectures": [
                "SCMP_ARCH_X86",
                "SCMP_ARCH_X32"
            ],
            "syscalls": [
                {
                    "names": [
                        "getcwd",
                        "chmod"
                    ],
                    "action": "SCMP_ACT_ERRNO"
                }
            ]
        },
        "namespaces": [
            {
                "type": "pid"
            },
            {
                "type": "network"
            },
            {
                "type": "ipc"
            },
            {
                "type": "uts"
            },
            {
                "type": "mount"
            },
            {
                "type": "user"
            },
            {
                "type": "cgroup"
            }
        ],
        "maskedPaths": [
            "/proc/kcore",
            "/proc/latency_stats",
            "/proc/timer_stats",
            "/proc/sched_debug"
        ],
        "readonlyPaths": [
            "/proc/asound",
            "/proc/bus",
            "/proc/fs",
            "/proc/irq",
            "/proc/sys",
            "/proc/sysrq-trigger"
        ],
        "mountLabel": "system_u:object_r:svirt_sandbox_file_t:s0:c715,c811"
    },
    "annotations": {
        "com.example.key1": "value1",
        "com.example.key2": "value2"
    }
}
"#;
}
