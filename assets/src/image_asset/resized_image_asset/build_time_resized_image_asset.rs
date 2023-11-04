use crate::image_asset::image_wrapper::BuildTimeImageWrapper;
use crate::{asset::Asset, CanSaveToDisk, DynamicImageExtension};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(PartialEq, Clone)]
pub struct BuildTimeResizedImageAsset {
    pub file_name: PathBuf,
    pub width: u32,
    pub image: Arc<BuildTimeImageWrapper>,
}

impl CanSaveToDisk for BuildTimeResizedImageAsset {
    fn save_to_disk(&self) {
        if self.path_on_disk().exists() {
            return;
        }

        Asset::save_to_disk(self);
    }
}

impl Asset for BuildTimeResizedImageAsset {
    fn file_name(&self) -> &Path {
        &self.file_name
    }

    fn bytes(&self) -> Vec<u8> {
        let path_to_resized_image_file = self.path_on_disk();
        let maybe_bytes = fs::read(&path_to_resized_image_file);

        match maybe_bytes {
            Ok(bytes) => return bytes,
            Err(error) => {
                println!(
                    "Error reading resized image file {:?}. Error message: {}",
                    &path_to_resized_image_file, error
                );
            }
        }

        self.image
            .dynamic_image
            .resize_to_width(self.width)
            .to_bytes_with_format(resized_image_format)
    }

    fn content_type(&self) -> String {
        get_content_type(resized_image_format)
    }
}

static resized_image_format: image::ImageFormat = image::ImageFormat::Jpeg;

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
