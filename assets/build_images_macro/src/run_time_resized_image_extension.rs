use crate::built_image::*;
use build_images_runtime::RunTimeResizedImage;
use std::path::PathBuf;

pub trait RunTimeResizedImageExtension {
    fn from_built_resized_image(resized_image: &ResizedImage) -> Self;
}

impl RunTimeResizedImageExtension for RunTimeResizedImage {
    fn from_built_resized_image(resized_image: &ResizedImage) -> Self {
        eprintln!("Instantiating RunTimeResizedImage.");
        let file_name = PathBuf::from(resized_image.path.file_name().unwrap().to_str().unwrap());
        let mime_type = resized_image.mime_type.to_string();
        let width = resized_image.width;
        let height = resized_image.height;

        RunTimeResizedImage {
            file_name,
            bytes: resized_image.bytes.clone(),
            mime_type,
            width,
            height,
            original_file_path: resized_image.path.clone(),
        }
    }
}
