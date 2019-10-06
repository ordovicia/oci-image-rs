use std::path::Path;

use spec::MediaType;

use crate::{
    convert_config::convert_config, layout::Layout, validate::*, Error, ErrorKind, Filter, Result,
};

pub fn unpack_index(
    index: &ValidatedIndex,
    layout: &Layout,
    bundle_dir: impl AsRef<Path>,
    filters: &[Filter],
) -> Result<()> {
    let mut descriptors = index.manifests().iter().filter(|d| {
        filters.iter().all(|f| descriptor_matches(d, f))
            && (d.media_type == MediaType::ImageIndex || d.media_type == MediaType::ImageManifest)
    });

    match (descriptors.next(), descriptors.next()) {
        (None, _) => Err(Error::new(ErrorKind::ManifestNotMatch)),
        (Some(desc), None) => match desc.media_type {
            MediaType::ImageIndex => {
                let index_nested: spec::Index = verify_descriptor(desc, layout)?.deser()?;
                let index_nested = validate_index(index_nested)?;

                unpack_index(&index_nested, layout, bundle_dir, filters)
            }
            MediaType::ImageManifest => {
                let manifest: spec::Manifest = verify_descriptor(desc, layout)?.deser()?;
                let manifest = validate_manifest(manifest)?;

                unpack_manifest(&manifest, layout, bundle_dir)
            }
            _ => unreachable!(),
        },
        (Some(_), Some(_)) => Err(Error::new(ErrorKind::ManifestNotUnique)),
    }
}

fn descriptor_matches(desc: &spec::Descriptor, filter: &Filter) -> bool {
    match filter {
        Filter::RefName(ref_name) => match desc.annotations.get(spec::annotation_keys::REF_NAME) {
            Some(n) if n != ref_name => false,
            _ => true,
        },
        Filter::Platform { os, arch } => match desc.platform {
            Some(ref platform) if platform.os != *os || platform.architecture != *arch => false,
            _ => true,
        },
    }
}

fn unpack_manifest(
    manifest: &ValidatedManifest,
    layout: &Layout,
    bundle_dir: impl AsRef<Path>,
) -> Result<()> {
    // Image config
    let image_cfg_desc = validate_image_config_descriptor(manifest.config())?;
    let image_cfg: spec::Image = verify_descriptor(image_cfg_desc.as_ref(), layout)?.deser()?;

    convert_config(&image_cfg)?;

    // Layers
    let bundle_dir = bundle_dir.as_ref();
    if bundle_dir.exists() && bundle_dir.read_dir()?.next().is_some() {
        return Err(Error::new(ErrorKind::BundleDirectoryNotEmpty));
    }

    let layers = manifest
        .layers()
        .iter()
        .map(|l| validate_layer_descriptor(l))
        .collect::<Result<Vec<_>>>()?;

    match expand_layers(&layers, layout, bundle_dir) {
        Ok(_) => Ok(()),
        Err(e) => {
            std::fs::remove_dir_all(bundle_dir)?;
            Err(e)
        }
    }
}

fn expand_layers(
    layers: &[ValidatedLayerDescriptor<'_>],
    layout: &Layout,
    _bundle_dir: impl AsRef<Path>,
) -> Result<()> {
    for layer in layers {
        let layer = verify_descriptor(layer.as_ref(), layout)?;
        dbg!(layer);
        // unimplemented!()
    }

    Ok(())
}
