use std::path::{Path, PathBuf};

use crate::{
    validate::{validate_image_layout, validate_index, ValidatedIndex},
    Error, ErrorKind, Result,
};

/// Image layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    index: ValidatedIndex,
    blobs: PathBuf,
}

/// Reads a root directory of an image and converts its into a `Layout`.
///
/// Validates its structure, the image layout version, and schema version of the image index.
/// Returns a `Layout` if the validation succeeded. If the validation failed, or other operation
/// (e.g. opening a file) failed, returns an error.
pub fn read_layout(path: impl AsRef<Path>) -> Result<Layout> {
    use spec::layout::{BLOBS, IMAGE_LAYOUT, INDEX_JSON};
    use std::ffi::OsStr;

    let mut layout_exists = false;
    let mut index = None;
    let mut blobs = None;

    for entry in path.as_ref().read_dir()? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_file() {
            let name = entry.file_name();

            if name == OsStr::new(IMAGE_LAYOUT) {
                let layout: spec::ImageLayout = deser(&entry)?;
                validate_image_layout(layout)?;

                layout_exists = true;
            } else if name == OsStr::new(INDEX_JSON) {
                let idx: spec::Index = deser(&entry)?;
                let idx = validate_index(idx)?;

                index = Some(idx);
            }
        } else if file_type.is_dir() && entry.file_name() == OsStr::new(BLOBS) {
            blobs = Some(entry.path());
        }
    }

    match (layout_exists, index, blobs) {
        (true, Some(index), Some(blobs)) => Ok(Layout::new(index, blobs)),
        _ => Err(Error::new(ErrorKind::InvalidLayout)),
    }
}

impl Layout {
    fn new(index: ValidatedIndex, blobs: PathBuf) -> Self {
        Layout { index, blobs }
    }

    /// Returns a reference to the top-level image index of this layout.
    pub fn index(&self) -> &ValidatedIndex {
        &self.index
    }

    /// Constructs the path to the content referenced by a descriptor.
    pub fn content_path(&self, descriptor: &spec::Descriptor) -> PathBuf {
        let digest = &descriptor.digest;
        self.blobs
            .join(digest.algorithm.to_string())
            .join(&digest.encoded)
    }
}

fn deser<T>(entry: &std::fs::DirEntry) -> Result<T>
where
    for<'de> T: serde::de::Deserialize<'de>,
{
    let file = std::fs::File::open(entry.path())?;
    serde_json::from_reader(&file).map_err(Error::deser)
}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO
}
