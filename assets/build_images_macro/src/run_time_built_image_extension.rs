use crate::built_image::*;
use crate::run_time_resized_image_extension::*;
use build_images_runtime::*;

pub trait RunTimeBuiltImageExtension {
    fn from_built_image(built_image: &BuiltImage) -> Self;
}

impl RunTimeBuiltImageExtension for RunTimeBuiltImage {
    fn from_built_image(built_image: &BuiltImage) -> Self {
        eprintln!("Extracting placeholder.");
        let placeholder = &built_image.placeholder;
        let lqip_data_uri = placeholder.lqip.to_string();
        let automatically_detected_color = placeholder.automatically_detected_color.to_string();

        eprintln!("Extracting width and height.");
        let width = built_image.width;
        let height = built_image.height;

        eprintln!("Extracting resized copies.");
        let resized_copies: Vec<_> = built_image
            .resized_copies
            .iter()
            .map(RunTimeResizedImage::from_built_resized_image)
            .collect();

        eprintln!("Creating RunTimeBuiltImage.");
        RunTimeBuiltImage {
            width,
            height,
            resized_copies,
            path_starting_from_images_dir: built_image.path_starting_from_images_dir.clone(),
            absolute_path_to_original_image: built_image.absolute_path_to_original_image.clone(),
            placeholder: RunTimePlaceholder {
                lqip_data_uri,
                automatically_detected_color,
            },
        }
    }
}
