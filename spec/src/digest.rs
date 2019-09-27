//! Digest, as a content identifier.
//!
//! See the [OCI image spec] for more information.
//!
//! [OCI image spec]: https://github.com/opencontainers/image-spec/blob/v1.0.1/descriptor.md#digests

use std::{error::Error, fmt, io, str::FromStr};

/// Digest, as a content identifier.
///
/// `Digest` has two methods: [`validate`] and [`verify`]. `validate` validates the format of a
/// digest, and `verify` verifies a given content with a digest.
///
/// Currently, `validate` and `verify` support SHA-256 and SHA-512 hash algorithms.
///
/// [`validate`]: #method.validate
/// [`verify`]: #method.verify
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Digest {
    /// Hash algorithm.
    pub algorithm: Algorithm,
    /// Encoded result of the content by the hash algorithm.
    pub encoded: String,
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
///
/// In a future version, this struct may have fields that convey the cause of error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseError;

/// Error type that can be returned when failed to validate the format of a digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidateError {
    /// Digest algorithm is not supported.
    AlgorithmNotSupported,
}

/// Error type that can be returned when failed to verify a content with a digest.
#[derive(Debug)]
pub enum VerifyError {
    /// Failed to read the content.
    Read(io::Error),
    /// Digest algorithm is not supported.
    AlgorithmNotSupported,
}

impl Digest {
    /// Validates the format of this digest according to its digest algorithm.
    ///
    /// Returns `Ok(true)` if this digest has a valid format. Returns `Ok(false)` if not.
    ///
    /// # Errors
    ///
    /// If the verification cannot be performed, `Err(ValidateError)` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use oci_image_spec::{Digest, digest::Algorithm};
    ///
    /// let digest = Digest {
    ///     algorithm: Algorithm::Sha256,
    ///     encoded: "6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b".to_string(),
    /// };
    ///
    /// assert_eq!(digest.validate(), Ok(true));
    /// ```
    pub fn validate(&self) -> Result<bool, ValidateError> {
        use Algorithm::*;

        fn is_sha2_char(c: char) -> bool {
            match c as u8 {
                b'a'..=b'f' | b'0'..=b'9' => true,
                _ => false,
            }
        }

        match self.algorithm {
            Sha256 => {
                // ^[a-f0-9]{64}$
                let ok = self.encoded.len() == 64 && self.encoded.chars().all(is_sha2_char);
                Ok(ok)
            }
            Sha512 => {
                // ^[a-f0-9]{128}$
                let ok = self.encoded.len() == 128 && self.encoded.chars().all(is_sha2_char);
                Ok(ok)
            }
            Other(_) => Err(ValidateError::AlgorithmNotSupported),
        }
    }

    /// Verifies a content with this digest.
    ///
    /// Returns `Ok(true)` if the content is verified. Returns `Ok(false)` if not.
    ///
    /// # Errors
    ///
    /// If the verification cannot be performed, `Err(VerifyError)` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use oci_image_spec::{Digest, digest::Algorithm};
    ///
    /// let content = b"foo";
    /// let digest = Digest {
    ///     algorithm: Algorithm::Sha256,
    ///     encoded: "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae".to_string(),
    /// };
    ///
    /// assert_eq!(digest.verify(&content[..]).unwrap(), true);
    /// ```
    pub fn verify(&self, mut reader: impl io::Read) -> Result<bool, VerifyError> {
        use sha2::Digest;
        use Algorithm::*;

        match self.algorithm {
            Sha256 => {
                let mut hasher = sha2::Sha256::new();
                io::copy(&mut reader, &mut hasher).map_err(VerifyError::Read)?;
                let hash = hasher.result();
                Ok(hex::encode(hash) == self.encoded)
            }
            Sha512 => {
                let mut hasher = sha2::Sha512::new();
                io::copy(&mut reader, &mut hasher).map_err(VerifyError::Read)?;
                let hash = hasher.result();
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
        let mut colon_sp = s.split(':');
        let algorithm = colon_sp.next().ok_or(ParseError)?;
        let encoded = colon_sp.next().ok_or(ParseError)?;

        // ^[a-z0-9]+(?:[.+_-][a-z0-9]+)*:[a-zA-Z0-9=_-]+$
        let alg_valid = algorithm
            .split(|c| c == '+' || c == '.' || c == '_' || c == '-')
            .all(|alg| {
                !alg.is_empty()
                    && alg
                        .chars()
                        .all(|c| c.is_ascii_digit() || c.is_ascii_lowercase())
            });
        let enc_valid = !encoded.is_empty()
            && encoded
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '=' || c == '_' || c == '-');

        if !alg_valid || !enc_valid {
            return Err(ParseError);
        }

        Ok(Digest {
            algorithm: algorithm.parse::<Algorithm>().unwrap(),
            encoded: encoded.to_string(),
        })
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
        }
    }
}

impl Error for ValidateError {}

impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(e) => write!(f, "Read failed: {}", e),
            Self::AlgorithmNotSupported => f.write_str("Unsupported digest algorithm"),
        }
    }
}

impl Error for VerifyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read(ref e) => Some(e),
            Self::AlgorithmNotSupported => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Algorithm::*;

    #[test]
    fn test_digest_validate() {
        // SHA-256 valid case is tested in doc

        let digest = Digest {
            algorithm: Sha512,
            encoded: "f7fbba6e0636f890e56fbbf3283e524c6fa3204ae298382d624741d0dc6638326e282c41be5e4254d8820772c5518a2c5a8c0c7f7eda19594a7eb539453e1ed7".to_string(),
        };
        assert_eq!(digest.validate(), Ok(true));

        // Length not match
        let digest = Digest {
            algorithm: Sha256,
            encoded: "b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270".to_string(),
        };
        assert_eq!(digest.validate(), Ok(false));

        // Invalid character
        let digest = Digest {
            algorithm: Sha512,
            encoded: "g7fbba6e0636f890e56fbbf3283e524c6fa3204ae298382d624741d0dc6638326e282c41be5e4254d8820772c5518a2c5a8c0c7f7eda19594a7eb539453e1ed7".to_string(),
        };
        assert_eq!(digest.validate(), Ok(false));
    }

    #[test]
    fn err_digest_validate() {
        // Unsupported algorithm
        let digest = Digest {
            algorithm: Other("unsupported".to_string()),
            encoded: "6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b".to_string(),
        };
        assert_eq!(
            digest.validate().unwrap_err(),
            ValidateError::AlgorithmNotSupported
        );
    }

    #[test]
    fn test_digest_verify() {
        let content = &b"foo"[..];

        // SHA-256 verified case is tested in doc

        let digest = Digest {
            algorithm: Sha512,
            encoded: "f7fbba6e0636f890e56fbbf3283e524c6fa3204ae298382d624741d0dc6638326e282c41be5e4254d8820772c5518a2c5a8c0c7f7eda19594a7eb539453e1ed7".to_string(),
        };
        assert_eq!(digest.verify(content).unwrap(), true);

        let digest = Digest {
            algorithm: Sha256,
            encoded: "1c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae".to_string(),
        };
        assert_eq!(digest.verify(content).unwrap(), false);

        let digest = Digest {
            algorithm: Sha512,
            encoded: "g7fbba6e0636f890e56fbbf3283e524c6fa3204ae298382d624741d0dc6638326e282c41be5e4254d8820772c5518a2c5a8c0c7f7eda19594a7eb539453e1ed7".to_string(),
        };
        assert_eq!(digest.verify(content).unwrap(), false);
    }

    #[test]
    fn err_digest_verify() {
        let content = &b"foo"[..];

        // Unsupported algorithm
        let digest = Digest {
            algorithm: Other("unsupported".to_string()),
            encoded: "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae".to_string(),
        };
        assert_eq!(
            std::mem::discriminant(&digest.verify(content).unwrap_err()),
            std::mem::discriminant(&VerifyError::AlgorithmNotSupported)
        );
    }

    #[test]
    fn test_digest_parse() {
        let digest = Digest::from_str(
            "sha256:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b",
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

        // Encoded part has invalid length, but parsing passes
        let digest =
            Digest::from_str("sha512:401b09eab3c013d4ca54922bb802bec8fd5318192b0a75f201d8b372742")
                .unwrap();
        assert_eq!(
            digest,
            Digest {
                algorithm: Sha512,
                encoded: "401b09eab3c013d4ca54922bb802bec8fd5318192b0a75f201d8b372742".to_string(),
            }
        );

        let digest =
            Digest::from_str("multihash+base58:QmRZxt2b1FVZPNqd8hsiykDL3TdBDeTSPX9Kv46HmX4Gx8")
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
    fn err_digest_from_str() {
        let test_cases = &[
            // missing parts
            "",
            "sha256:",
            ":6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b",
            // invalid algorithm-encoded separator
            "sha256+6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b",
            // invalid algorithm char
            "X:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b",
            // invalid algorithm separator
            "sha256/sha512:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b",
            // leading/trailing algorithm separator
            "-sha256:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b",
            "sha256-:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b",
            // invalid encoded char
            "sha256:/c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b",
        ];

        for case in test_cases {
            assert_eq!(Digest::from_str(case).unwrap_err(), ParseError);
        }
    }

    #[test]
    fn test_digest_display() {
        let digest = Digest {
            algorithm: Sha256,
            encoded: "6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b".to_string(),
        };
        assert_eq!(
            digest.to_string(),
            "sha256:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b"
        );

        // Encoded part has invalid length, but to_string passes
        let digest = Digest {
            algorithm: Sha512,
            encoded: "401b09eab3c013d4ca54922bb802bec8fd5318192b0a75f201d8b372742".to_string(),
        };
        assert_eq!(
            digest.to_string(),
            "sha512:401b09eab3c013d4ca54922bb802bec8fd5318192b0a75f201d8b372742"
        );

        let digest = Digest {
            algorithm: Other("sha256+b64u".to_string()),
            encoded: "LCa0a2j_xo_5m0U8HTBBNBNCLXBkg7-g-YpeiGJm564".to_string(),
        };
        assert_eq!(
            digest.to_string(),
            "sha256+b64u:LCa0a2j_xo_5m0U8HTBBNBNCLXBkg7-g-YpeiGJm564"
        );
    }
}
