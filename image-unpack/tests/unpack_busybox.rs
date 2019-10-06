use oci_image_unpack::{unpack, Result};

#[test]
fn unpack_busybox() -> Result<()> {
    use std::path::PathBuf;

    let path = [env!("CARGO_MANIFEST_DIR"), "tests", "busybox-oci"]
        .iter()
        .collect::<PathBuf>();

    unpack(&path, "", &[])
}
