use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

mod light_dark_image_asset;
use crate::asset_path_from_file_name;
use crate::mime_type::MimeType;

pub use self::light_dark_image_asset::*;

pub mod image_wrapper;
use image_wrapper::*;

mod resized_image_asset;
pub use self::resized_image_asset::ResizedImageAsset;

#[derive(PartialEq)]
pub struct ImageAsset {
    pub file_name: PathBuf,
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
        file_name: PathBuf,
        alt: &'static str,
        bytes: &'static [u8],
        placeholder: Placeholder,
    ) -> ImageAsset {
        let image = ImageWrapper::new(bytes, file_name.clone());
        let image = Arc::new(image);

        let (width, height) = image.dimensions();
        let mime_type = image.mime_type();

        let srcset = Self::create_srcset(&file_name, width);

        let resized_variants = Self::resized_variants(&file_name, &image);

        ImageAsset {
            file_name,
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
            .file_name
            .to_str()
            .unwrap()
    }

    pub fn srcset(&self) -> &str {
        &self.srcset
    }

    pub fn mime_type(&self) -> MimeType {
        self.mime_type
    }

    fn resized_variants(
        file_name: &Path,
        original_image: &Arc<ImageWrapper>,
    ) -> Vec<ResizedImageAsset> {
        let original_width = original_image.width();

        Self::available_widths(original_width)
            .into_iter()
            .map(|target_width| ResizedImageAsset {
                file_name: Self::file_name_with_width(file_name, target_width),
                width: target_width,
                image: original_image.clone(),
            })
            .collect()
    }

    fn create_srcset(file_name: &Path, image_width: u32) -> String {
        Self::available_widths(image_width)
            .into_iter()
            .map(|width| {
                let path_with_width = Self::path_with_width(file_name, width);
                let file_name_string = path_with_width.to_str().unwrap();
                format!("{file_name_string} {width}w")
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

    fn file_name_with_width(file_name: &Path, width: u32) -> PathBuf {
        let old_file_stem = file_name.file_stem().unwrap().to_str().unwrap();
        let old_file_extension = file_name.extension().unwrap().to_str().unwrap();
        let new_file_name_string = format!("{}-{}w.{}", old_file_stem, width, old_file_extension);
        PathBuf::from_str(&new_file_name_string).unwrap()
    }

    fn path_with_width(file_name: &Path, width: u32) -> PathBuf {
        let file_name_with_width = Self::file_name_with_width(file_name, width);
        asset_path_from_file_name(&file_name_with_width)
    }
}

cfg_if! {
if #[cfg(feature = "build")] {
    use crate::CanSaveToDisk;

    impl CanSaveToDisk for ImageAsset {
        fn save_to_disk(&self) {
            self.serialized_image_wrapper().save_to_disk(&self.file_name);
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

cfg_if! {
if #[cfg(not(feature = "build"))] {
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
}
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
pub enum GeneratedPlaceholder {
    Lqip { data_uri: String },
    Color { css_string: String },
}
