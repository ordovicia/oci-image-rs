use std::{error::Error, fmt, str::FromStr};

/// Environment variable.
///
/// `EnvVar` implements `Display` and `FromStr`. You can convert an `EnvVar` into a string and vice
/// versa.
///
/// ```
/// # use environ_str::EnvVar;
/// let env_var = EnvVar { name: "PATH".to_string(), value: "/usr/local/bin".to_string() };
/// let environ_str = env_var.to_string();
///
/// assert_eq!(environ_str, "PATH=/usr/local/bin");
/// assert_eq!(environ_str.parse::<EnvVar>().unwrap(), env_var);
/// ```
///
/// With `serde` feature enabled (default), `EnvVar` also implements `Serialize` and `Deserialize`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnvVar {
    /// Name of this environment variable.
    pub name: String,
    /// Value of this environment variable.
    pub value: String,
}

/// Error type for parsing a string into an `EnvVar`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParseEnvVarError;

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

#[cfg(feature = "serde")]
impl serde::Serialize for EnvVar {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EnvVar {
    fn deserialize<D: serde::Deserializer<'de>>(deser: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deser)?;
        Ok(s.parse::<Self>().unwrap())
    }
}

impl fmt::Display for ParseEnvVarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Failed to parse environment variable")
    }
}

impl Error for ParseEnvVarError {}

/// Parses an `environ` string into `EnvVar`s.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), environ_str::ParseEnvVarError> {
/// use environ_str::{EnvVar, parse_environ};
///
/// let mut env_vars = parse_environ("PATH=/usr/local/bin HOME=/home/name");
///
/// assert_eq!(
///     env_vars.next().unwrap(),
///     Ok(EnvVar { name: "PATH".to_string(), value: "/usr/local/bin".to_string() }),
/// );
/// assert_eq!(
///     env_vars.next().unwrap(),
///     Ok(EnvVar { name: "HOME".to_string(), value: "/home/name".to_string() }),
/// );
/// assert!(env_vars.next().is_none());
/// # Ok(())
/// # }
/// ```
pub fn parse_environ<'a>(
    environ: &'a str,
) -> impl 'a + Iterator<Item = Result<EnvVar, ParseEnvVarError>> {
    environ.split_whitespace().map(|s| s.parse::<EnvVar>())
}

#[cfg(all(feature = "serde", test))]
mod tests_serde {
    use super::*;

    #[test]
    fn test_serde() {
        let env_var = EnvVar {
            name: "key".to_string(),
            value: "val".to_string(),
        };
        assert_eq!(serde_json::to_string(&env_var).unwrap(), r#""key=val""#);

        let env_var: EnvVar = serde_json::from_str(r#""key=val""#).unwrap();
        assert_eq!(
            env_var,
            EnvVar {
                name: "key".to_string(),
                value: "val".to_string()
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_value() {
        let env_var = EnvVar::from_str("key=").unwrap();
        assert_eq!(
            env_var,
            EnvVar {
                name: "key".to_string(),
                value: String::new()
            }
        );

        let env_var = EnvVar {
            name: "key".to_string(),
            value: String::new(),
        };
        assert_eq!(env_var.to_string(), "key=");
    }

    #[test]
    fn err_parse() {
        let test_cases = &["", "=", "=val", "key-val", "key-"];

        for case in test_cases {
            assert_eq!(EnvVar::from_str(case).unwrap_err(), ParseEnvVarError);
        }
    }
}
