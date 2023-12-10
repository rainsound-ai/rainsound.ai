use assets_runtime::Placeholder;
use image::{DynamicImage, GenericImageView};
use mime::Mime;
use std::fs;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use super::dynamic_image_extension::DynamicImageExtension;

#[derive(Clone)]
pub struct ImageToBuild {
    pub absolute_path_to_original_image: PathBuf,
    pub path_starting_from_images_dir: PathBuf,
    pub resized_copies: Vec<BuildTimeResizedImage>,
    pub placeholder: Placeholder,
    pub width: u32,
    pub height: u32,
    pub name_in_source_code: String,
    pub alt: String,
}

impl ImageToBuild {
    pub fn new(
        path_to_images_dir: &Path,
        absolute_path_to_original_image: PathBuf,
        original_image: DynamicImage,
        placeholder_to_generate: PlaceholderToGenerate,
        alt: String,
    ) -> Self {
        let path_starting_from_images_dir = absolute_path_to_original_image
            .strip_prefix(path_to_images_dir)
            .expect("Error stripping prefix from absolute path to original image.")
            .to_path_buf();
        let (width, height) = original_image.dimensions();
        let resized_copies = Self::resized_copies(
            path_to_images_dir,
            &absolute_path_to_original_image,
            &original_image,
        );
        let placeholder = placeholder_to_generate.to_placeholder(&original_image);

        let file_stem = absolute_path_to_original_image
            .file_stem()
            .expect("Error parsing file stem.")
            .to_string_lossy()
            .to_string();

        let name_in_source_code = file_stem.replace(
            |character: char| {
                let should_keep = character.is_alphanumeric() || character == '_';
                !should_keep
            },
            "_",
        );

        ImageToBuild {
            path_starting_from_images_dir,
            absolute_path_to_original_image,
            resized_copies,
            placeholder,
            width,
            height,
            name_in_source_code,
            alt,
        }
    }

    fn resized_copies(
        path_to_images_dir: &Path,
        absolute_path_to_original_image: &Path,
        original_image: &DynamicImage,
    ) -> Vec<BuildTimeResizedImage> {
        let original_width = original_image.width();

        Self::available_widths(original_width)
            .into_iter()
            .map(|target_width| {
                BuildTimeResizedImage::new(
                    target_width,
                    path_to_images_dir,
                    absolute_path_to_original_image,
                    original_image,
                )
            })
            .collect()
    }

    fn available_widths(image_width: u32) -> Vec<u32> {
        Self::possible_widths()
            .into_iter()
            .filter(|possible_width| possible_width <= &image_width)
            .collect()
    }

    fn possible_widths() -> Vec<u32> {
        (100..=4000).step_by(100).collect()
    }
}

#[derive(Clone)]
pub struct BuildTimeResizedImage {
    pub absolute_path: PathBuf,
    pub path_starting_from_images_dir: PathBuf,
    pub width: u32,
    pub height: u32,
    pub mime_type: Mime,
    pub bytes: Vec<u8>,
}

impl BuildTimeResizedImage {
    pub fn new(
        width: u32,
        path_to_images_dir: &Path,
        absolute_path_to_original_image: &Path,
        original_image: &DynamicImage,
    ) -> Self {
        let path_starting_from_images_dir_without_width = absolute_path_to_original_image
            .strip_prefix(path_to_images_dir)
            .expect("Error stripping prefix from absolute path to original image.")
            .to_path_buf();

        let mime_type = mime::IMAGE_JPEG;

        let path_starting_from_images_dir_with_width =
            Self::path_starting_from_images_dir_with_width(
                &path_starting_from_images_dir_without_width,
                width,
                &mime_type,
            );

        let absolute_path_with_width =
            assets_runtime::paths::built_image_path(&path_starting_from_images_dir_with_width);

        let height = original_image.height_if_resized_to_width(width);

        let bytes = Self::generate_bytes(width, &absolute_path_with_width, original_image);

        Self {
            absolute_path: absolute_path_with_width,
            path_starting_from_images_dir: path_starting_from_images_dir_with_width,
            width,
            height,
            mime_type,
            bytes,
        }
    }

    fn path_starting_from_images_dir_with_width(
        path_starting_from_images_dir_without_width: &Path,
        width: u32,
        mime_type: &Mime,
    ) -> PathBuf {
        let file_name = path_starting_from_images_dir_without_width
            .file_name()
            .expect("Error parsing file name.");
        let file_name = PathBuf::from(file_name);
        let file_name_with_width = Self::file_name_with_width(&file_name, width, mime_type);

        path_starting_from_images_dir_without_width
            .parent()
            .expect("Error getting parent of path starting from images dir.")
            .join(file_name_with_width)
    }

    fn file_name_with_width(file_name: &Path, width: u32, mime_type: &Mime) -> PathBuf {
        let old_file_stem = file_name.file_stem().unwrap().to_str().unwrap();
        let new_file_extension = mime_type.subtype().as_str();
        let new_file_name_string = format!("{}_{}w.{}", old_file_stem, width, new_file_extension);
        PathBuf::from_str(&new_file_name_string).unwrap()
    }

    fn generate_bytes(width: u32, path: &Path, original_image: &DynamicImage) -> Vec<u8> {
        fs::read(path).unwrap_or_else(|error| {
            println!(
                "Couldn't read resized image file {:?} so regenerating the resized image. Original error message: {}",
                &path, error
            );

            let bytes = original_image
                .resize_to_width(width)
                .to_bytes_with_format(image::ImageFormat::Jpeg);

            fs::create_dir_all(path.parent().unwrap()).expect("Error creating built images dir.");
            fs::write(path, &bytes).expect("Error writing resized image to disk.");

            bytes
        })
    }
}

pub enum PlaceholderToGenerate {
    Lqip,
    AutomaticallyDetectedColor,
}

impl PlaceholderToGenerate {
    pub fn to_placeholder(&self, original_image: &DynamicImage) -> Placeholder {
        match self {
            // Returns a string like "data:image/jpeg;base64,/9j/4AAQSk...".
            PlaceholderToGenerate::Lqip => {
                let data_uri = original_image.resize_to_width(40).to_data_uri();
                Placeholder::Lqip { data_uri }
            }

            // Returns a string like "rgba(255, 255, 255, 1.0)".
            PlaceholderToGenerate::AutomaticallyDetectedColor => {
                let [red, green, blue, alpha] = original_image
                    .resize_exact(1, 1, image::imageops::Lanczos3)
                    .get_pixel(0, 0)
                    .0;

                let css_string = format!(
                    "rgba({red}, {green}, {blue}, {alpha})",
                    red = red,
                    green = green,
                    blue = blue,
                    alpha = alpha
                );

                Placeholder::Color { css_string }
            }
        }
    }
}
