use std::{error::Error as StdErr, fmt};

use spec::digest::VerifyError;

/// Alias of the result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type.
///
/// The kind and lower-level source of an error can be obtained via `kind` and `source` method.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn StdErr + Send + Sync + 'static>>,
}

/// Kinds of errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// Failed to do an I/O operation on a file system.
    Io,

    /// Failed to deserialize a JSON into a value.
    Deserialize,

    /// Root directory of an image has invalid layout. i.e. `oci_layout` file, `index.json` file, or
    /// `blobs` directory is missing.
    InvalidLayout,

    /// Version of image layout is not supported.
    LayoutVersionNotSupported,

    /// Schema version of an JSON is not supported.
    SchemaVersionNotSupported,

    /// No manifest matches with filters.
    ManifestNotMatch,

    /// Multiple (not unique) manifests match with filters.
    ManifestNotUnique,

    /// Descriptor has a different media type than expected.
    UnexpectedMediaType,

    /// Directory on where a runtime bundle is expanded exists but not empty.
    BundleDirectoryNotEmpty,

    /// Digest algorithm is not supported.
    DigestAlgorithmNotSupported,

    /// Failed to verify a content with a digest.
    VerifyContent,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind, source: None }
    }

    pub(crate) fn with_source<E>(kind: ErrorKind, source: E) -> Self
    where
        E: StdErr + Send + Sync + 'static,
    {
        Self {
            kind,
            source: Some(Box::new(source)),
        }
    }

    pub(crate) fn deser(e: serde_json::Error) -> Self {
        Self::with_source(ErrorKind::Deserialize, e)
    }

    /// Returns the kind of this error.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl StdErr for Error {
    fn source(&self) -> Option<&(dyn StdErr + 'static)> {
        match self.source {
            Some(ref x) => Some(&**x),
            None => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;

        f.write_str(match self.kind {
            Io => "I/O failed",
            InvalidLayout => "Invalid directory layout",
            Deserialize => "Deserialization failed",
            LayoutVersionNotSupported => "Unsupported image layout version",
            SchemaVersionNotSupported => "Unsupported schema version",
            ManifestNotMatch => "no manifest matches with filters",
            ManifestNotUnique => "multiple manifests match with filters",
            UnexpectedMediaType => "descriptor has unexpected media type",
            BundleDirectoryNotEmpty => "bundle directory exists but not empty",
            DigestAlgorithmNotSupported => "Unsupported digest algorithm",
            VerifyContent => "Content not matches with digest",
        })?;

        if let Some(ref source) = self.source {
            write!(f, ": {}", source)?;
        }

        Ok(())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::with_source(ErrorKind::Io, e)
    }
}

impl From<VerifyError> for Error {
    fn from(e: VerifyError) -> Self {
        match e {
            VerifyError::Read(io_err) => Self {
                kind: ErrorKind::Io,
                source: Some(Box::new(io_err)),
            },
            VerifyError::AlgorithmNotSupported => Self {
                kind: ErrorKind::DigestAlgorithmNotSupported,
                source: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unreachable_code, dead_code)]
    fn test_error_impl_sync_send(e: Error) {
        let _: &dyn Sync = &e;
        let _: &dyn Send = &e;
    }
}
