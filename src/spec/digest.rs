//! Digest, as a content identifier.
//!
//! See the [OCI image spec] for more information.
//!
//! [OCI image spec]: https://github.com/opencontainers/image-spec/blob/master/descriptor.md#digests

use std::{error::Error, fmt, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

/// Digest, as a content identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Digest {
    algorithm: Algorithm,
    encoded: String,
}

/// Registered and other hash algorithms.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Algorithm {
    /// SHA-256.
    Sha256,
    /// SHA-512.
    Sha512,
    /// Other (not registered) hash algorithm.
    Other(String),
}

/// Error type for parsing a string into a `Digest`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseError;

/// Error type for validating the format of a `Digest`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidateError {
    /// Digest algorithm is not supported.
    AlgorithmNotSupported,
    /// Digest is invalid.
    InvalidForm,
}

/// Error type for verifying a content with a digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyError {
    /// Digest algorithm is not supported.
    AlgorithmNotSupported,
}

impl Digest {
    /// Returns the hash algorithm.
    pub fn algorithm(&self) -> &Algorithm {
        &self.algorithm
    }

    /// Returns the encoded result of the content by the hash algorithm.
    pub fn encoded(&self) -> &str {
        &self.encoded
    }

    /// Validates the format of this digest.
    ///
    /// Returns `Ok(())` if this digest has valid format. Otherwise, returns an
    /// `Err(ValidateError)`. The reason why verification failed can be obtained via its variant.
    pub fn validate(&self) -> Result<(), ValidateError> {
        use Algorithm::*;
        use ValidateError::*;

        match self.algorithm {
            Sha256 => {
                lazy_static! {
                    static ref RE: Regex = Regex::new("^[a-f0-9]{64}$").unwrap();
                }

                if !RE.is_match(&self.encoded) {
                    return Err(InvalidForm);
                }
            }
            Sha512 => {
                lazy_static! {
                    static ref RE: Regex = Regex::new("^[a-f0-9]{128}$").unwrap();
                }

                if !RE.is_match(&self.encoded) {
                    return Err(InvalidForm);
                }
            }
            Other(_) => {
                return Err(AlgorithmNotSupported);
            }
        }

        Ok(())
    }

    /// Verifies a content with this digest.
    ///
    /// Returns `Ok(true)` if the content is verified. Returns `Ok(false)` if not verified.
    /// If the verification cannot be performed, `Err(VerifyError)` is returned.
    pub fn verify(&self, content: &[u8]) -> Result<bool, VerifyError> {
        use sha2::Digest;
        use Algorithm::*;

        match self.algorithm {
            Sha256 => {
                let hash = sha2::Sha256::digest(content);
                Ok(hex::encode(hash) == self.encoded)
            }
            Sha512 => {
                let hash = sha2::Sha512::digest(content);
                Ok(hex::encode(hash) == self.encoded)
            }
            Other(_) => Err(VerifyError::AlgorithmNotSupported),
        }
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.algorithm, self.encoded)
    }
}

impl FromStr for Digest {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref DIGEST_RE: Regex =
                Regex::new("^[a-z0-9]+(?:[.+_-][a-z0-9]+)*:[a-zA-Z0-9=_-]+$").unwrap();
        }

        if !DIGEST_RE.is_match(s) {
            return Err(ParseError);
        }

        let mut colon_sp = s.split(':');
        let algorithm = colon_sp
            .next()
            .ok_or(ParseError)?
            .parse::<Algorithm>()
            .unwrap();
        let encoded = colon_sp.next().ok_or(ParseError)?.to_string();

        Ok(Digest { algorithm, encoded })
    }
}

impl_serde_for_str_conv!(Digest);

impl_str_conv! {
    Algorithm,
    (Sha256, "sha256"),
    (Sha512, "sha512")
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Failed to parse digest")
    }
}

impl Error for ParseError {}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlgorithmNotSupported => f.write_str("Unsupported digest algorithm"),
            Self::InvalidForm => f.write_str("Invalid digest form"),
        }
    }
}

impl Error for ValidateError {}

impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlgorithmNotSupported => f.write_str("Unsupported digest algorithm"),
        }
    }
}

impl Error for VerifyError {}

#[cfg(test)]
mod tests {
    use super::*;
    use Algorithm::*;

    #[test]
    fn test_digest_validate() {
        let digest = Digest {
            algorithm: Sha256,
            encoded: "6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b".to_string(),
        };
        assert_eq!(digest.validate(), Ok(()));

        let digest = Digest {
            algorithm: Sha512,
            encoded: "401b09eab3c013d4ca54922bb802bec8fd5318192b0a75f201d8b372742".to_string(),
        };
        assert_eq!(digest.validate().unwrap_err(), ValidateError::InvalidForm);

        let digest = Digest {
            algorithm: Other("foo".to_string()),
            encoded: "6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b".to_string(),
        };
        assert_eq!(
            digest.validate().unwrap_err(),
            ValidateError::AlgorithmNotSupported
        );
    }

    #[test]
    fn test_digest_verify() {
        let digest = Digest {
            algorithm: Sha256,
            encoded: "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae".to_string(),
        };
        assert_eq!(digest.verify(b"foo"), Ok(true));

        let digest = Digest {
            algorithm: Sha256,
            encoded: "1c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae".to_string(),
        };
        assert_eq!(digest.verify(b"foo"), Ok(false));

        let digest = Digest {
            algorithm: Other("foo".to_string()),
            encoded: "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae".to_string(),
        };
        assert_eq!(
            digest.verify(b"foo").unwrap_err(),
            VerifyError::AlgorithmNotSupported
        );
    }
}

#[cfg(all(feature = "serde", test))]
mod tests_serde {
    use super::*;
    use Algorithm::*;

    #[test]
    fn test_digest_deser() {
        let digest: Digest = serde_json::from_str(
            r#""sha256:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b""#,
        )
        .unwrap();
        assert_eq!(
            digest,
            Digest {
                algorithm: Sha256,
                encoded: "6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b"
                    .to_string(),
            }
        );

        let digest: Digest = serde_json::from_str(
            r#""sha512:401b09eab3c013d4ca54922bb802bec8fd5318192b0a75f201d8b372742""#, // encoded part has invalid length
        )
        .unwrap();
        assert_eq!(
            digest,
            Digest {
                algorithm: Sha512,
                encoded: "401b09eab3c013d4ca54922bb802bec8fd5318192b0a75f201d8b372742".to_string(),
            }
        );

        let digest: Digest = serde_json::from_str(
            r#""multihash+base58:QmRZxt2b1FVZPNqd8hsiykDL3TdBDeTSPX9Kv46HmX4Gx8""#,
        )
        .unwrap();
        assert_eq!(
            digest,
            Digest {
                algorithm: Other("multihash+base58".to_string()),
                encoded: "QmRZxt2b1FVZPNqd8hsiykDL3TdBDeTSPX9Kv46HmX4Gx8".to_string(),
            }
        );
    }

    #[test]
    fn test_digest_ser() {
        let digest = Digest {
            algorithm: Sha256,
            encoded: "6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b".to_string(),
        };
        assert_eq!(
            serde_json::to_string(&digest).unwrap(),
            r#""sha256:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b""#
        );

        let digest = Digest {
            algorithm: Sha512,
            encoded: "401b09eab3c013d4ca54922bb802bec8fd5318192b0a75f201d8b372742".to_string(),
        };
        assert_eq!(
            serde_json::to_string(&digest).unwrap(),
            r#""sha512:401b09eab3c013d4ca54922bb802bec8fd5318192b0a75f201d8b372742""#, // encoded part has invalid length
        );

        let digest = Digest {
            algorithm: Other("sha256+b64u".to_string()),
            encoded: "LCa0a2j_xo_5m0U8HTBBNBNCLXBkg7-g-YpeiGJm564".to_string(),
        };
        assert_eq!(
            serde_json::to_string(&digest).unwrap(),
            r#""sha256+b64u:LCa0a2j_xo_5m0U8HTBBNBNCLXBkg7-g-YpeiGJm564""#,
        );
    }
}
