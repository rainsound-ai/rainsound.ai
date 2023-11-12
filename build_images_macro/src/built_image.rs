use crate::dynamic_image_extension::*;
use image::{DynamicImage, GenericImageView};
use mime::Mime;
use std::fs;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

pub struct BuiltImage {
    pub path_to_original_image: PathBuf,
    pub original_image: Arc<DynamicImage>,
    pub resized_copies: Vec<ResizedImage>,
    pub placeholder: Placeholder,
    pub width: u32,
    pub height: u32,
    pub name_in_source_code: String,
}

impl BuiltImage {
    pub fn new(path_to_original_image: PathBuf, original_image: DynamicImage) -> Self {
        let original_image = Arc::new(original_image);
        let (width, height) = original_image.dimensions();
        let resized_copies = Self::resized_copies(&path_to_original_image, &original_image);
        let placeholder = Placeholder::new(&original_image);

        let file_stem = path_to_original_image
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

        BuiltImage {
            path_to_original_image,
            original_image,
            resized_copies,
            placeholder,
            width,
            height,
            name_in_source_code,
        }
    }

    fn resized_copies(
        path_to_original_image: &Path,
        original_image: &Arc<DynamicImage>,
    ) -> Vec<ResizedImage> {
        let original_width = original_image.width();

        Self::available_widths(original_width)
            .into_iter()
            .map(|target_width| {
                ResizedImage::new(target_width, path_to_original_image, original_image.clone())
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

pub struct ResizedImage {
    // We store a copy of the original image so that we can resize it
    // to different widths if necessary.
    pub original_image: Arc<DynamicImage>,
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub mime_type: Mime,
}

impl ResizedImage {
    pub fn new(
        width: u32,
        path_to_original_image: &Path,
        original_image: Arc<DynamicImage>,
    ) -> Self {
        let file_name = path_to_original_image
            .file_name()
            .expect("Error parsing file name.");
        let file_name = PathBuf::from(file_name);

        let mime_type = mime::IMAGE_JPEG;

        let path = Self::path_with_width(&file_name, width, &mime_type);

        let height = original_image.height_if_resized_to_width(width);

        Self {
            path,
            original_image,
            width,
            height,
            mime_type,
        }
    }

    fn path_with_width(file_name: &Path, width: u32, mime_type: &Mime) -> PathBuf {
        let file_name_with_width = Self::file_name_with_width(file_name, width, mime_type);
        crate::paths::built_image_path_from_file_name(&file_name_with_width)
    }

    fn file_name_with_width(file_name: &Path, width: u32, mime_type: &Mime) -> PathBuf {
        let old_file_stem = file_name.file_stem().unwrap().to_str().unwrap();
        let new_file_extension = mime_type.subtype().as_str();
        let new_file_name_string = format!("{}_{}w.{}", old_file_stem, width, new_file_extension);
        PathBuf::from_str(&new_file_name_string).unwrap()
    }

    pub fn save_to_disk_if_necessary(&self) {
        if self.file_already_exists() {
            return;
        }

        let bytes = self.bytes();

        fs::create_dir_all(self.path.parent().unwrap()).expect("Error creating built images dir.");
        fs::write(&self.path, bytes).expect("Error writing resized image to disk.");
    }

    fn bytes(&self) -> Vec<u8> {
        let maybe_bytes = fs::read(&self.path);

        match maybe_bytes {
            Ok(bytes) => return bytes,
            Err(error) => {
                println!(
                    "Couldn't read resized image file {:?} so regenerating the resized image. Original error message: {}",
                    &self.path, error
                );
            }
        }

        self.original_image
            .resize_to_width(self.width)
            .to_bytes_with_format(image::ImageFormat::Jpeg)
    }

    fn file_already_exists(&self) -> bool {
        self.path.exists()
    }
}

// We don't know what placeholders users will want at runtime so we just
// generate all the options.
pub struct Placeholder {
    pub lqip: DataUriString,
    pub automatically_detected_color: RgbaString,
}

impl Placeholder {
    fn new(original_image: &DynamicImage) -> Self {
        let lqip = original_image.resize_to_width(40).to_data_uri();

        let [red, green, blue, alpha] = original_image
            .resize_exact(1, 1, image::imageops::Lanczos3)
            .get_pixel(0, 0)
            .0;

        let automatically_detected_color = format!(
            "rgba({red}, {green}, {blue}, {alpha})",
            red = red,
            green = green,
            blue = blue,
            alpha = alpha
        );

        Self {
            lqip,
            automatically_detected_color,
        }
    }
}

// Strings like "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAgGBgcGBQgHBwcJCQgKDBQNDAsLDBkS…".
pub type DataUriString = String;

// Strings like "rgba(255, 255, 255, 1.0)".
pub type RgbaString = String;
