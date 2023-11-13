use build_images::{RunTimeBuiltImage, RunTimeResizedImage};
use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

mod light_dark_image_asset;
use crate::asset_path_from_file_name;
use crate::mime_type::MimeType;

pub use self::light_dark_image_asset::*;

pub struct ImageAsset {
    pub alt: &'static str,
    pub placeholder: BuiltPlaceholder,
    pub width: u32,
    pub height: u32,
    pub resized_copies: Vec<RunTimeResizedImage>,
    path_to_original_image: PathBuf,
    srcset: String,
}

impl ImageAsset {
    pub fn from_built_image(
        alt: &'static str,
        placeholder: Placeholder,
        built_image: RunTimeBuiltImage,
    ) -> ImageAsset {
        let srcset = Self::create_srcset(&file_name, width);

        let placeholder = match placeholder {
            Placeholder::Lqip => BuiltPlaceholder::Lqip {
                data_uri: built_image.placeholder.lqip_data_uri,
            },
            Placeholder::AutomaticColor => BuiltPlaceholder::Color {
                css_string: built_image.placeholder.automatically_detected_color,
            },
            Placeholder::Color { css_string } => BuiltPlaceholder::Color { css_string },
        };

        ImageAsset {
            alt,
            placeholder,
            width: built_image.width,
            height: built_image.height,
            resized_copies: built_image.resized_copies,
            path_to_original_image: built_image.path_to_original_image,
            srcset,
        }
    }

    pub fn src(&self) -> &str {
        // If their browser doesn't have support for the srcset attribute,
        // it's probably an old mobile browser. If that's the case, they
        // also probably don't have a lot of bandwidth so go with the smallest
        // image possible.
        self.resized_copies
            .first()
            .unwrap()
            .path
            .file_name()
            .to_str()
            .unwrap()
    }

    pub fn srcset(&self) -> &str {
        &self.srcset
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
}

cfg_if! {
if #[cfg(feature = "build")] {
    use crate::CanSaveToDisk;

    impl CanSaveToDisk for ImageAsset {
        fn save_to_disk(&self) {
            self.serialized_image_wrapper().save_to_disk(&self.file_name);
            for resized_variant in &self.resized_copies {
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
    Color { css_string: &'static str },
}

pub enum BuiltPlaceholder {
    Lqip { data_uri: &'static str },
    Color { css_string: &'static str },
}
