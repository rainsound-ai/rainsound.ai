use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

mod light_dark_image_asset;
use crate::mime_type::MimeType;
use crate::CanSaveToDisk;

pub use self::light_dark_image_asset::*;

pub mod image_wrapper;
use image_wrapper::*;

mod resized_image_asset;
pub use self::resized_image_asset::ResizedImageAsset;

// Things that have to happen at build time:
// - Generating placeholders.
//   - LQIPs.
//   - Automatic placeholder colors.
// - Image resizing.
// - Mime type detection?
//
// Things that have to happen at runtime:
// - Serving images.
// - Generating HTML.
//
// When instantiating an image asset:
// - If it's runtime, read the generated placeholder from the file system.
// - If it's build time, generate the placeholder and save it to the file system.

// pub static paths_of_images_in_built_dir: Lazy<HashSet<PathBuf>> =
//     Lazy::new(get_paths_of_images_in_built_dir);

// fn get_paths_of_images_in_built_dir() -> HashSet<PathBuf> {
//     let images_dir = crate::built_images_dir();

//     fs::read_dir(&images_dir)
//         .unwrap_or_else(|error| {
//             println!(
//                 "Error reading directory {:?}. Error message: {}",
//                 images_dir, error
//             );
//             fs::create_dir_all(&images_dir).unwrap();
//             fs::read_dir(&images_dir).unwrap()
//         })
//         .map(|entry| entry.unwrap().path())
//         .collect::<HashSet<PathBuf>>()
// }

#[derive(PartialEq)]
pub struct ImageAsset {
    // For runtime.
    pub path: PathBuf,
    pub alt: &'static str,
    srcset: String,
    mime_type: MimeType,
    pub placeholder: GeneratedPlaceholder,

    pub bytes: &'static [u8],
    pub width: u32,
    pub height: u32,
    pub resized_variants: Vec<ResizedImageAsset>,

    image: Arc<ImageWrapper>,
}

impl ImageAsset {
    pub fn new(
        path: PathBuf,
        alt: &'static str,
        bytes: &'static [u8],
        placeholder: Placeholder,
    ) -> ImageAsset {
        println!("Creating image wrapper.");
        let image = ImageWrapper::new(bytes, path.clone());
        let image = Arc::new(image);

        println!("Getting image dimensions and mime type");
        let (width, height) = image.dimensions();
        let mime_type = image.mime_type();

        println!("Creating srcset.");
        let path = PathBuf::from_str("images/").unwrap().join(path);
        let srcset = Self::create_srcset(&path, width);

        println!("Creating resized variants.");
        let resized_variants = Self::resized_variants(&path, &image);

        ImageAsset {
            path,
            alt,
            bytes,
            placeholder: image.generate_placeholder(placeholder),
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
        self.resized_variants
            .first()
            .unwrap()
            .path
            .to_str()
            .unwrap()
    }

    pub fn srcset(&self) -> &str {
        &self.srcset
    }

    pub fn mime_type(&self) -> MimeType {
        self.mime_type
    }

    fn resized_variants(path: &Path, original_image: &Arc<ImageWrapper>) -> Vec<ResizedImageAsset> {
        let original_width = original_image.width();

        Self::available_widths(original_width)
            .into_iter()
            .map(|target_width| ResizedImageAsset {
                path: Self::path_with_width(path, target_width),
                width: target_width,
                image: original_image.clone(),
            })
            .collect()
    }

    fn create_srcset(path: &Path, image_width: u32) -> String {
        Self::available_widths(image_width)
            .into_iter()
            .map(|width| {
                let path_with_width = Self::path_with_width(path, width);
                let path_string = path_with_width.to_str().unwrap();
                format!("{path_string} {width}w")
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

    fn path_with_width(path: &Path, width: u32) -> PathBuf {
        let old_file_stem = path.file_stem().unwrap().to_str().unwrap();
        let old_file_extension = path.extension().unwrap().to_str().unwrap();
        let new_file_name = format!("{}-{}w.{}", old_file_stem, width, old_file_extension);
        path.with_file_name(new_file_name)
    }
}

cfg_if! {
if #[cfg(feature = "build")] {
    impl CanSaveToDisk for ImageAsset {
        fn save_to_disk(&self) {
            self.serialized_image_wrapper().save_to_disk(&self.path);
            for resized_variant in &self.resized_variants {
                resized_variant.save_to_disk();
            }
        }
    }

    impl ImageAsset {
        fn serialized_image_wrapper(&self) -> SerializedImageWrapper {
            SerializedImageWrapper {
                dimensions: (self.width, self.height),
                generated_placeholder: self.placeholder.clone(),
                mime_type: self.mime_type,
            }
        }
    }
}
}

#[derive(Debug)]
pub enum Placeholder {
    Lqip,
    AutomaticColor,
    Color { css_string: String },
}

impl Placeholder {
    fn matches(&self, generated_placeholder: &GeneratedPlaceholder) -> bool {
        matches!(
            (self, generated_placeholder),
            (Placeholder::Lqip, GeneratedPlaceholder::Lqip { .. })
                | (
                    Placeholder::Color { .. },
                    GeneratedPlaceholder::Color { .. }
                )
                | (
                    Placeholder::AutomaticColor,
                    GeneratedPlaceholder::Color { .. }
                )
        )
    }
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
pub enum GeneratedPlaceholder {
    Lqip { data_uri: String },
    Color { css_string: String },
}
