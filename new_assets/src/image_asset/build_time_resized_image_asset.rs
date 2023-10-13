use crate::{asset::Asset, extensions::DynamicImageExtension};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use super::build_time_image_wrapper::BuildTimeImageWrapper;

pub type ResizedImageAsset = BuildTimeResizedImageAsset;

static resized_image_format: image::ImageFormat = image::ImageFormat::Jpeg;

#[derive(PartialEq, Clone)]
pub struct BuildTimeResizedImageAsset {
    pub path: PathBuf,
    pub width: u32,
    pub image: Arc<BuildTimeImageWrapper>,
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

        self.image
            .dynamic_image
            .into_bytes_with_format(resized_image_format)
    }

    fn content_type(&self) -> String {
        get_content_type(resized_image_format)
    }
}

fn get_content_type(format: image::ImageFormat) -> String {
    match format {
        image::ImageFormat::Jpeg => "image/jpeg".to_string(),
        image::ImageFormat::Png => "image/png".to_string(),
        image::ImageFormat::Gif => "image/gif".to_string(),
        image::ImageFormat::WebP => "image/webp".to_string(),
        image::ImageFormat::Avif => "image/avif".to_string(),
        image::ImageFormat::Tiff => "image/tiff".to_string(),
        image::ImageFormat::Bmp => "image/bmp".to_string(),
        image::ImageFormat::Ico => "image/x-icon".to_string(),
        _ => panic!("Unsupported image format: {:?}", format),
    }
}
