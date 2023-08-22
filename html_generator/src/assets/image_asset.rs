use crate::prelude::*;
use base64::prelude::*;
use image::{DynamicImage, GenericImageView, ImageFormat};
use rayon::prelude::*;
use std::collections::HashSet;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(PartialEq)]
pub struct LightDarkImageAsset {
    pub alt: &'static str,
    pub light_mode: ImageAsset,
    pub dark_mode: ImageAsset,
}

#[derive(PartialEq)]
pub struct ImageAsset {
    pub asset_path: String,
    pub alt: &'static str,
    pub bytes: &'static [u8],
    pub lqip: String,

    mime_type: String,

    pub width: u32,
    pub height: u32,
    pub resized_variants: Vec<ResizedImageAsset>,

    srcset: String,
    image: Arc<DynamicImage>,
}

impl ImageAsset {
    pub fn new(
        asset_path: &'static str,
        alt: &'static str,
        bytes: &'static [u8],
        // lqip: &'static str,
    ) -> ImageAsset {
        let asset_path = "images/".to_string() + asset_path;
        let image = image::load_from_memory(bytes).unwrap();
        let image = Arc::new(image);
        let (width, height) = image.dimensions();
        let srcset = Self::create_srcset(&asset_path, width);
        let resized_variants = Self::resized_variants(&asset_path, &image);
        let mime_type = tree_magic::from_u8(bytes);
        let lqip = Self::create_lqip(image.clone(), &mime_type);

        ImageAsset {
            asset_path,
            alt,
            bytes,
            lqip,
            width,
            height,
            srcset,
            image,
            resized_variants,
            mime_type,
        }
    }

    pub fn src(&self) -> &str {
        // If their browser doesn't have support for the srcset attribute,
        // it's probably an old mobile browser. If that's the case, they
        // also probably don't have a lot of bandwidth so go with the smallest
        // image possible.
        self.resized_variants.first().unwrap().asset_path.as_str()
    }

    pub fn srcset(&self) -> &str {
        &self.srcset
    }

    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    fn resized_variants(
        asset_path: &str,
        original_image: &Arc<DynamicImage>,
    ) -> Vec<ResizedImageAsset> {
        let original_width = original_image.width();

        Self::available_widths(original_width)
            .into_par_iter()
            .map(|target_width| ResizedImageAsset {
                asset_path: Self::asset_path_with_width(asset_path, target_width),
                width: target_width,
                image: original_image.clone(),
            })
            .collect()
    }

    fn create_srcset(asset_path: &str, image_width: u32) -> String {
        Self::available_widths(image_width)
            .into_iter()
            .map(|width| {
                let asset_path_with_width = Self::asset_path_with_width(asset_path, width);
                format!("{asset_path_with_width} {width}w")
            })
            .collect::<Vec<String>>()
            .join(", ")
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

    fn asset_path_with_width(asset_path: &str, width: u32) -> String {
        map_filename_without_extension(asset_path, |filename_without_extension| {
            format!("{filename_without_extension}-{width}w")
        })
    }

    fn create_lqip(image: Arc<DynamicImage>, mime_type: &str) -> String {
        let resized = image.resize_to_width(40);
        let mut bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        resized
            .write_to(&mut bytes, ImageFormat::Jpeg)
            .expect("Error encoding low quality image placeholder.");
        let base64_encoded = base64::engine::general_purpose::STANDARD.encode(bytes.into_inner());

        format!(
            "data:{mime_type};base64,{base64}",
            mime_type = mime_type,
            base64 = base64_encoded
        )
    }
}

#[derive(PartialEq)]
pub struct ResizedImageAsset {
    asset_path: String,
    width: u32,
    image: Arc<DynamicImage>,
}

impl ResizedImageAsset {
    pub fn save_to_disk(&self, built_dir: &Path, paths_of_files_in_built_dir: &HashSet<PathBuf>) {
        println!("Deciding whether to save resized image to disk");
        let path = Assets::path_on_disk(built_dir, &self.asset_path);
        if self.needs_to_be_recreated(&path, paths_of_files_in_built_dir) {
            let parent_dir = path.parent().unwrap();
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir).unwrap();
            }

            println!("Saving resized image to disk: {:?}", &self.asset_path);
            self.image
                .resize_to_width(self.width)
                .save_with_format(path, image::ImageFormat::Jpeg)
                .unwrap();

            return;
        }

        println!(
            "Resized image {} already exists, so skipping saving it to disk.",
            &self.asset_path
        );
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
