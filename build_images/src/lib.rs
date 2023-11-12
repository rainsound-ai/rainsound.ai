use std::path::PathBuf;

mod test;
pub use build_images_macro::build_images;

pub struct RunTimeBuiltImage {
    pub path_to_original_image: PathBuf,
    pub resized_copies: Vec<RunTimeResizedImage>,
    pub placeholder: RunTimePlaceholder,
    pub width: u32,
    pub height: u32,
}

pub struct RunTimeResizedImage {
    pub bytes: &'static [u8],
    pub mime_type: &'static str,
    pub width: u32,
    pub height: u32,
}

pub struct RunTimePlaceholder {
    pub lqip_data_uri: &'static str,
    pub automatically_detected_color: &'static str,
}
