use super::*;
use crate::non_html_assets;
use base64::Engine;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::fs;
use std::io::Cursor;

pub type ImageWrapper = BuildTimeImageWrapper;

#[derive(PartialEq)]
pub struct BuildTimeImageWrapper {
    dynamic_image: DynamicImage,
}

impl BuildTimeImageWrapper {}

impl ImageWrapperMethods for BuildTimeImageWrapper {
    fn new(bytes: &'static [u8]) -> Self {
        let dynamic_image = image::load_from_memory(bytes).unwrap();
        Self { dynamic_image }
    }

    fn dimensions(&self) -> (u32, u32) {
        self.dynamic_image.dimensions()
    }

    fn width(&self) -> u32 {
        self.dimensions().0
    }

    fn generate_placeholder(&self, placeholder: Placeholder) -> GeneratedPlaceholder {
        match placeholder {
            Placeholder::Lqip => {
                let DataUriAndMimeType {
                    data_uri,
                    mime_type,
                } = self.dynamic_image.resize_to_width(40).to_data_uri();

                GeneratedPlaceholder::Lqip {
                    data_uri,
                    mime_type,
                }
            }

            Placeholder::Color { css_string } => GeneratedPlaceholder::Color { css_string },

            Placeholder::AutomaticColor => {
                let [red, green, blue, alpha] = self
                    .dynamic_image
                    .resize_exact(1, 1, Lanczos3)
                    .get_pixel(0, 0)
                    .0;

                let css_string = format!(
                    "rgba({red}, {green}, {blue}, {alpha})",
                    red = red,
                    green = green,
                    blue = blue,
                    alpha = alpha
                );

                GeneratedPlaceholder::Color { css_string }
            }
        }
    }
}

pub trait DynamicImageExtension {
    fn resize_to_width(&self, new_width: u32) -> Self;
    fn to_data_uri(&self) -> DataUriAndMimeType;
}

impl DynamicImageExtension for DynamicImage {
    fn resize_to_width(&self, new_width: u32) -> Self {
        let (width, height) = self.dimensions();

        let new_width_f32 = new_width as f32;
        let height_f32 = height as f32;
        let width_f32 = width as f32;

        let scale = new_width_f32 / width_f32;

        let new_height_f32 = height_f32 * scale;
        let new_height = new_height_f32.ceil() as u32;

        self.resize(new_width, new_height, FilterType::Lanczos3)
    }

    fn to_data_uri(&self) -> DataUriAndMimeType {
        let mut bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        self.write_to(&mut bytes, ImageFormat::Jpeg)
            .expect("Error encoding low quality image placeholder.");
        let base64_encoded = base64::engine::general_purpose::STANDARD.encode(bytes.into_inner());

        let mime_type = "format/jpeg";

        let data_uri = format!(
            "data:{mime_type};base64,{base64}",
            mime_type = mime_type,
            base64 = base64_encoded
        );

        DataUriAndMimeType {
            mime_type,
            data_uri,
        }
    }
}

pub struct DataUriAndMimeType {
    pub mime_type: &'static str,
    pub data_uri: String,
}

pub type MimeType = &'static str;
pub type DataUri = String;

fn save_to_disk(built_dir: &Path) {
    let paths_of_images_in_built_dir = get_file_paths(built_dir);
    save_resized_image_assets_to_disk(built_dir, &paths_of_images_in_built_dir);
}

fn save_resized_image_assets_to_disk(
    built_dir: &Path,
    paths_of_images_in_built_dir: &HashSet<PathBuf>,
) {
    let image_resized_variants = non_html_assets
        .images()
        .iter()
        .map(|image_asset| image_asset.resized_variants.clone());

    let light_dark_resized_variants = non_html_assets
        .light_dark_images()
        .iter()
        .map(|light_dark_image_asset| light_dark_image_asset.resized_variants());

    let resized_image_assets = image_resized_variants
        .chain(light_dark_resized_variants)
        .collect::<Vec<_>>();

    resized_image_assets
        .into_iter()
        .par_bridge()
        .for_each(|resized_image_asset| {
            resized_image_asset.save_to_disk(built_dir, paths_of_images_in_built_dir);
        });
}

fn get_file_paths(built_dir: &Path) -> HashSet<PathBuf> {
    let images_dir = built_dir.join("images");

    fs::read_dir(&images_dir)
        .unwrap_or_else(|error| {
            println!(
                "Error reading directory {:?}. Error message: {}",
                images_dir, error
            );
            fs::create_dir_all(&images_dir).unwrap();
            fs::read_dir(&images_dir).unwrap()
        })
        .map(|entry| entry.unwrap().path())
        .collect::<HashSet<PathBuf>>()
}

#[derive(PartialEq, Clone)]
pub struct ResizedImageAsset {
    path: PathBuf,
    width: u32,
    image: Arc<ImageWrapper>,
}

impl ResizedImageAsset {
    pub fn save_to_disk(&self, paths_of_files_in_built_dir: &HashSet<PathBuf>) {
        println!("Deciding whether to save resized image to disk.");
        let path = crate::path_for_asset_on_disk(&self.path);
        if !self.needs_to_be_recreated(&path, paths_of_files_in_built_dir) {
            println!(
                "Resized image {} already exists, so skipping saving it to disk.",
                &self.path.to_str().unwrap()
            );
        }

        let parent_dir = path.parent().unwrap();
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }

        println!("Saving resized image to disk: {:?}", &self.path);
        self.image
            .resize_to_width(self.width)
            .save_with_format(path, image::ImageFormat::Jpeg)
            .unwrap();

        return;
    }

    pub fn needs_to_be_recreated(
        &self,
        path_to_resized_image: &Path,
        paths_of_files_in_built_dir: &HashSet<PathBuf>,
    ) -> bool {
        // let image_on_disk = image::open(path_to_resized_image).unwrap();
        // if image_on_disk.width() == self.width {
        //     return false;
        // }
        let already_exists = paths_of_files_in_built_dir.contains(path_to_resized_image);
        !already_exists
    }
}
