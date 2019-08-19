//! Image information and configuration.
//!
//! See the [OCI image spec] for more information.
//!
//! [OCI image spec]: https://github.com/opencontainers/image-spec/blob/master/config.md

use std::{error::Error, fmt, str::FromStr};

use chrono::{DateTime, FixedOffset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::spec::{
    descriptor::{Architecture, Os},
    Annotations, Digest, GoSet,
};

/// Basic information about an image.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Image {
    /// Date and time on which the image was created (in RFC 3339 format).
    pub created: DateTime<FixedOffset>,

    /// Name and/or email address of the person or entity which created and is responsible for maintaining the image.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub author: Option<String>,

    /// CPU architecture on which the binaries in this image are built to run.
    pub architecture: Architecture,

    /// Operating system on which the image is built to run.
    pub os: Os,

    /// Execution parameters which SHOULD be used as a base when running a container using the image.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub config: Option<Config>,

    /// References layer content addresses used by the image.
    pub rootfs: RootFs,

    /// History of each layer.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub history: Vec<History>,
}

/// Image configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "PascalCase")
)]
pub struct Config {
    /// User name or UID (platform-specific structure) that allows specific control over which user
    /// the process run as.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub user: Option<String>,

    /// Set of ports to expose from a container running this image.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "GoSet::is_empty", default)
    )]
    pub exposed_ports: GoSet<Port>,

    /// List of default environment variables to be used in a container.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub env: Vec<EnvVar>,

    /// List of arguments to use as the command to execute when a container starts.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub entrypoint: Vec<String>,

    /// Default arguments to the entry point of a container.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub cmd: Vec<String>,

    /// Set of directories describing where the process is likely write data specific to a container instance.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "GoSet::is_empty", default)
    )]
    pub volumes: GoSet<String>,

    /// Working directory of the entry point process in a container.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub working_dir: Option<String>,

    /// Arbitrary metadata for a container.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Annotations::is_empty", default)
    )]
    pub labels: Annotations,

    /// System call signal that will be sent to a container to exit.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub stop_signal: Option<String>,
}

/// Exposed port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Port {
    /// UDP protocol.
    Udp {
        /// Port number.
        port: u16,
    },
    /// TCP protocol.
    Tcp {
        /// Port number.
        port: u16,
    },
}

/// Error type for parsing a string into a `Port`.
#[derive(Debug)]
pub struct ParsePortError {
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

/// Environment variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnvVar {
    /// Name of this environment variable.
    pub name: String,
    /// Value of this environment variable.
    pub value: String,
}

/// Error type for parsing a string into a `EnvVar`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEnvVarError;

/// Type of a rootfs.
pub const TYPE_LAYERS: &str = "layers";

/// References layer content addresses used by the image.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RootFs {
    /// MUST be set to "layers".
    pub r#type: String,

    /// Array of layer content hashes (DiffIDs), in order from first to last.
    pub diff_ids: Vec<Digest>,
}

/// History of each layer.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct History {
    /// Date and time on which the layer was created (in RFC 3339 format).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub created: Option<DateTime<FixedOffset>>,

    /// Author of the build point.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub author: Option<String>,

    /// Command which created the layer.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub created_by: Option<String>,

    /// Custom message set when creating the layer.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub comment: Option<String>,

    /// Marks if the history item created a filesystem diff.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub empty_layer: Option<bool>,
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Udp { port } => write!(f, "{}/udp", port),
            Self::Tcp { port } => write!(f, "{}/tcp", port),
        }
    }
}

impl FromStr for Port {
    type Err = ParsePortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (port, protocol) = {
            let mut slash_sp = s.split('/');
            let port = slash_sp.next().ok_or(ParsePortError { source: None })?;
            let protocol = slash_sp.next().ok_or(ParsePortError { source: None })?;
            (port, protocol)
        };

        let port = port.parse::<u16>().map_err(|e| ParsePortError {
            source: Some(Box::new(e)),
        })?;

        match protocol {
            "udp" => Ok(Self::Udp { port }),
            "tcp" => Ok(Self::Tcp { port }),
            _ => Err(ParsePortError { source: None }),
        }
    }
}

impl_serde_for_str_conv!(Port);

impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

impl FromStr for EnvVar {
    type Err = ParseEnvVarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut equal_sp = s.split('=');
        let name = equal_sp.next().ok_or(ParseEnvVarError)?;
        let val = equal_sp.next().ok_or(ParseEnvVarError)?;

        if name.is_empty() || equal_sp.next().is_some() {
            return Err(ParseEnvVarError);
        }

        Ok(EnvVar {
            name: name.to_string(),
            value: val.to_string(),
        })
    }
}

impl_serde_for_str_conv!(EnvVar);

impl fmt::Display for ParsePortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid port")?;
        if let Some(ref s) = self.source {
            write!(f, ": {}", s)?;
        }
        Ok(())
    }
}

impl Error for ParsePortError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.source {
            Some(ref s) => Some(&**s),
            None => None,
        }
    }
}

impl fmt::Display for ParseEnvVarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid environment variable")
    }
}

impl Error for ParseEnvVarError {}

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;
    use crate::spec::descriptor::{Architecture, Os};
    use chrono::TimeZone;
    use std::string::ToString;

    // Example from https://github.com/opencontainers/image-spec/blob/master/config.md#example

    #[test]
    fn test_image_deser() {
        let json = r#"
{
    "created": "2015-10-31T22:22:56.015925234Z",
    "author": "Alyssa P. Hacker <alyspdev@example.com>",
    "architecture": "amd64",
    "os": "linux",
    "config": {
        "User": "alice",
        "ExposedPorts": {
            "8080/tcp": {}
        },
        "Env": [
            "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
            "FOO=oci_is_a",
            "BAR=well_written_spec"
        ],
        "Entrypoint": [
            "/bin/my-app-binary"
        ],
        "Cmd": [
            "--foreground",
            "--config",
            "/etc/my-app.d/default.cfg"
        ],
        "Volumes": {
            "/var/job-result-data": {},
            "/var/log/my-app-logs": {}
        },
        "WorkingDir": "/home/alice",
        "Labels": {
            "com.example.project.git.url": "https://example.com/project.git",
            "com.example.project.git.commit": "45a939b2999782a3f005621a8d0f29aa387e1d6b"
        }
    },
    "rootfs": {
      "diff_ids": [
        "sha256:c6f988f4874bb0add23a778f753c65efe992244e148a1d2ec2a8b664fb66bbd1",
        "sha256:5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef"
      ],
      "type": "layers"
    },
    "history": [
      {
        "created": "2015-10-31T22:22:54.690851953Z",
        "created_by": "/bin/sh -c #(nop) ADD file:a3bc1e842b69636f9df5256c49c5374fb4eef1e281fe3f282c65fb853ee171c5 in /"
      },
      {
        "created": "2015-10-31T22:22:55.613815829Z",
        "created_by": "/bin/sh -c #(nop) CMD [\"sh\"]",
        "empty_layer": true
      }
    ]
}
        "#;

        let image: Image = serde_json::from_str(&json).unwrap();

        assert_eq!(
            image,
            Image {
                created: FixedOffset::east(0).ymd(2015, 10, 31).and_hms_nano(22, 22, 56, 015925234),
                author: Some("Alyssa P. Hacker <alyspdev@example.com>".to_string()),
                architecture: Architecture::Amd64,
                os: Os::Linux,
                config: Some(Config {
                    user: Some("alice".to_string()),
                    exposed_ports: [Port::Tcp { port: 8080 }]
                        .iter().copied()
                        .collect(),
                    env: [
                        (
                            "PATH",
                            "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
                        ),
                        ("FOO", "oci_is_a"),
                        ("BAR", "well_written_spec"),
                    ]
                    .iter()
                    .map(|(n, v)| EnvVar {
                        name: n.to_string(),
                        value: v.to_string()
                    })
                    .collect(),
                    entrypoint: vec!["/bin/my-app-binary".to_string()],
                    cmd: ["--foreground", "--config", "/etc/my-app.d/default.cfg"]
                        .iter()
                        .map(ToString::to_string)
                        .collect(),
                    volumes: ["/var/job-result-data", "/var/log/my-app-logs"]
                        .iter()
                        .map(ToString::to_string)
                        .collect(),
                    working_dir: Some("/home/alice".to_string()),
                    labels: [
                        (
                            "com.example.project.git.url",
                            "https://example.com/project.git"
                        ),
                        (
                            "com.example.project.git.commit",
                            "45a939b2999782a3f005621a8d0f29aa387e1d6b"
                        ),
                    ]
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
                    stop_signal: None,
                }),
                rootfs: RootFs {
                    r#type: TYPE_LAYERS.to_string(),
                    diff_ids: [
                        "sha256:c6f988f4874bb0add23a778f753c65efe992244e148a1d2ec2a8b664fb66bbd1",
                        "sha256:5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef",
                    ].iter().map(|s| Digest::from_str(s).unwrap()).collect(),
                },
                history: vec![
                    History {
                        created: Some(FixedOffset::east(0).ymd(2015, 10, 31).and_hms_nano(22, 22, 54, 690851953)),
                        author: None,
                        created_by:  Some("/bin/sh -c #(nop) ADD file:a3bc1e842b69636f9df5256c49c5374fb4eef1e281fe3f282c65fb853ee171c5 in /".to_string()),
                        comment: None,
                        empty_layer: None,
                    },
                    History {
                        created: Some(FixedOffset::east(0).ymd(2015, 10, 31).and_hms_nano(22, 22, 55, 613815829)),
                        author: None,
                        created_by: Some("/bin/sh -c #(nop) CMD [\"sh\"]".to_string()),
                        comment: None,
                        empty_layer: Some(true),
                    }
                ],
            },
        );
    }

    #[test]
    fn test_image_ser() {
        // Not using zulu

        let image = Image {
                created: FixedOffset::east(0).ymd(2015, 10, 31).and_hms_nano(22, 22, 56, 15925234),
                author: Some("Alyssa P. Hacker <alyspdev@example.com>".to_string()),
                architecture: Architecture::Amd64,
                os: Os::Linux,
                config: Some(Config {
                    user: Some("alice".to_string()),
                    exposed_ports: [Port::Tcp { port: 8080 }]
                        .iter().copied()
                        .collect(),
                    env: [
                        (
                            "PATH",
                            "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
                        ),
                        ("FOO", "oci_is_a"),
                        ("BAR", "well_written_spec"),
                    ]
                    .iter()
                    .map(|(n, v)| EnvVar {
                        name: n.to_string(),
                        value: v.to_string()
                    })
                    .collect(),
                    entrypoint: vec!["/bin/my-app-binary".to_string()],
                    cmd: ["--foreground", "--config", "/etc/my-app.d/default.cfg"]
                        .iter()
                        .map(ToString::to_string)
                        .collect(),
                    volumes: ["/var/job-result-data", ] // "/var/log/my-app-logs"]
                        .iter()
                        .map(ToString::to_string)
                        .collect(),
                    working_dir: Some("/home/alice".to_string()),
                    labels: [
                        (
                            "com.example.project.git.url",
                            "https://example.com/project.git"
                        ),
                        // (
                        //     "com.example.project.git.commit",
                        //     "45a939b2999782a3f005621a8d0f29aa387e1d6b"
                        // ),
                    ]
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
                    stop_signal: None,
                }),
                rootfs: RootFs {
                    r#type: TYPE_LAYERS.to_string(),
                    diff_ids: [
                        "sha256:c6f988f4874bb0add23a778f753c65efe992244e148a1d2ec2a8b664fb66bbd1",
                        "sha256:5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef",
                    ].iter().map(|s| Digest::from_str(s).unwrap()).collect(),
                },
                history: vec![
                    History {
                        created: Some(FixedOffset::east(0).ymd(2015, 10, 31).and_hms_nano(22, 22, 54, 690851953)),
                        author: None,
                        created_by:  Some("/bin/sh -c #(nop) ADD file:a3bc1e842b69636f9df5256c49c5374fb4eef1e281fe3f282c65fb853ee171c5 in /".to_string()),
                        comment: None,
                        empty_layer: None,
                    },
                    History {
                        created: Some(FixedOffset::east(0).ymd(2015, 10, 31).and_hms_nano(22, 22, 55, 613815829)),
                        author: None,
                        created_by: Some("/bin/sh -c #(nop) CMD [\"sh\"]".to_string()),
                        comment: None,
                        empty_layer: Some(true),
                    }
                ],
            };

        assert_eq!(
            serde_json::to_string_pretty(&image).unwrap(),
            r#"{
  "created": "2015-10-31T22:22:56.015925234+00:00",
  "author": "Alyssa P. Hacker <alyspdev@example.com>",
  "architecture": "amd64",
  "os": "linux",
  "config": {
    "User": "alice",
    "ExposedPorts": {
      "8080/tcp": {}
    },
    "Env": [
      "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
      "FOO=oci_is_a",
      "BAR=well_written_spec"
    ],
    "Entrypoint": [
      "/bin/my-app-binary"
    ],
    "Cmd": [
      "--foreground",
      "--config",
      "/etc/my-app.d/default.cfg"
    ],
    "Volumes": {
      "/var/job-result-data": {}
    },
    "WorkingDir": "/home/alice",
    "Labels": {
      "com.example.project.git.url": "https://example.com/project.git"
    }
  },
  "rootfs": {
    "type": "layers",
    "diff_ids": [
      "sha256:c6f988f4874bb0add23a778f753c65efe992244e148a1d2ec2a8b664fb66bbd1",
      "sha256:5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef"
    ]
  },
  "history": [
    {
      "created": "2015-10-31T22:22:54.690851953+00:00",
      "created_by": "/bin/sh -c #(nop) ADD file:a3bc1e842b69636f9df5256c49c5374fb4eef1e281fe3f282c65fb853ee171c5 in /"
    },
    {
      "created": "2015-10-31T22:22:55.613815829+00:00",
      "created_by": "/bin/sh -c #(nop) CMD [\"sh\"]",
      "empty_layer": true
    }
  ]
}"#
        );
    }
}
