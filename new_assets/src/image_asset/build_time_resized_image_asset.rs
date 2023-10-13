use crate::extensions::dynamic_image::DynamicImageExtension;
use crate::{asset::Asset, non_html_assets};
use image::DynamicImage;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

pub type ResizedImageAsset = BuildTimeResizedImageAsset;

#[derive(PartialEq, Clone)]
pub struct BuildTimeResizedImageAsset {
    path: PathBuf,
    width: u32,
    image: Arc<DynamicImage>,
}

impl Asset for BuildTimeResizedImageAsset {
    fn path(&self) -> &Path {
        &self.path
    }

    fn bytes(&self) -> Vec<u8> {
        let path_to_resized_image_file = self.path_on_disk();
        let already_exists =
            super::paths_of_images_in_built_dir.contains(&path_to_resized_image_file);

        if already_exists {
            return fs::read(&path_to_resized_image_file).unwrap();
        }

        println!("Resizing image: {:?}", &self.path);

        let mut bytes = Vec::new();

        let resized_image = self.image.resize_to_width(self.width);
        resized_image.write_to(&mut bytes, image::ImageFormat::Jpeg);

        bytes
    }
}

fn save_resized_image_assets_to_disk(
    built_dir: &Path,
    paths_of_images_in_built_dir: &HashSet<PathBuf>,
) {
    let image_resized_variants = non_html_assets
        .images()
        .iter()
        .flat_map(|image_asset| image_asset.resized_variants.clone());

    let light_dark_resized_variants = non_html_assets
        .light_dark_images()
        .iter()
        .flat_map(|light_dark_image_asset| light_dark_image_asset.resized_variants());

    let resized_image_assets = image_resized_variants
        .chain(light_dark_resized_variants)
        .collect::<Vec<_>>();

    resized_image_assets
        .into_iter()
        .par_bridge()
        .for_each(|resized_image_asset| {
            resized_image_asset.save_to_disk();
        });
}
