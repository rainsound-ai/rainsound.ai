use crate::Asset;
use crate::FileToSave;
use build_images::{RunTimeBuiltImage, RunTimeResizedImage};
use std::path::PathBuf;

mod light_dark_image_asset;

pub use self::light_dark_image_asset::*;

pub struct ImageAsset {
    pub alt: &'static str,
    pub placeholder: BuiltPlaceholder,

    pub width: u32,
    pub height: u32,

    pub resized_copies: Vec<RunTimeResizedImage>,
    pub srcset: String,
    pub src: String,

    path_to_original_image: PathBuf,
}

impl ImageAsset {
    pub fn from_built_image(
        alt: &'static str,
        placeholder: Placeholder,
        built_image: RunTimeBuiltImage,
    ) -> ImageAsset {
        let srcset = Self::generate_srcset(&built_image.resized_copies);
        let src = Self::generate_src(&built_image);
        let placeholder = Self::get_placeholder(&built_image, placeholder);

        ImageAsset {
            alt,
            placeholder,

            width: built_image.width,
            height: built_image.height,

            resized_copies: built_image.resized_copies,
            srcset,
            src,

            path_to_original_image: built_image.path_to_original_image,
        }
    }

    pub fn generate_src(built_image: &RunTimeBuiltImage) -> String {
        // If their browser doesn't have support for the srcset attribute,
        // it's probably an old mobile browser. If that's the case, they
        // also probably don't have a lot of bandwidth so go with the smallest
        // image possible.
        let narrowest = built_image
            .resized_copies
            .iter()
            .min_by_key(|resized_copy| resized_copy.width)
            .expect("Received a built image with no resized copies.");

        crate::path_for_asset_in_browser(&narrowest.file_name)
            .to_string_lossy()
            .to_string()
    }

    fn generate_srcset(resized_copies: &[RunTimeResizedImage]) -> String {
        resized_copies
            .iter()
            .map(|resized_copy| {
                let width = resized_copy.width;
                let path = crate::path_for_asset_in_browser(&resized_copy.file_name);
                let path_str = path.to_str().unwrap();
                format!("{path_str} {width}w")
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn get_placeholder(
        built_image: &RunTimeBuiltImage,
        placeholder: Placeholder,
    ) -> BuiltPlaceholder {
        match placeholder {
            Placeholder::Lqip => BuiltPlaceholder::Lqip {
                data_uri: built_image.placeholder.lqip_data_uri,
            },
            Placeholder::AutomaticColor => BuiltPlaceholder::Color {
                css_string: built_image.placeholder.automatically_detected_color,
            },
            Placeholder::Color { css_string } => BuiltPlaceholder::Color { css_string },
        }
    }
}

#[derive(Debug)]
pub enum Placeholder {
    Lqip,
    AutomaticColor,
    Color { css_string: String },
}

pub enum BuiltPlaceholder {
    Lqip { data_uri: String },
    Color { css_string: String },
}

impl Asset for ImageAsset {
    fn files_to_save(&self) -> Vec<crate::FileToSave> {
        self.resized_copies
            .iter()
            .map(|resized_copy| FileToSave {
                file_name: &resized_copy.file_name,
                bytes: &resized_copy.bytes,
                content_type: &resized_copy.mime_type,
            })
            .collect()
    }
}
