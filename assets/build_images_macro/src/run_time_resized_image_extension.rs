use crate::built_image::*;
use build_images_runtime::RunTimeResizedImage;

pub trait RunTimeResizedImageExtension {
    fn from_built_resized_image(resized_image: &ResizedImage) -> Self;
}

impl RunTimeResizedImageExtension for RunTimeResizedImage {
    fn from_built_resized_image(resized_image: &ResizedImage) -> Self {
        eprintln!("Instantiating RunTimeResizedImage.");
        let mime_type = resized_image.mime_type.to_string();
        let width = resized_image.width;
        let height = resized_image.height;

        RunTimeResizedImage {
            path_starting_from_images_dir: resized_image.path_starting_from_images_dir.clone(),
            bytes: resized_image.bytes.clone(),
            mime_type,
            width,
            height,
            absolute_path: resized_image.absolute_path.clone(),
        }
    }
}
