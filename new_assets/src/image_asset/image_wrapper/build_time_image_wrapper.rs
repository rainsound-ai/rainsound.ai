use super::*;
use base64::Engine;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::io::Cursor;

#[derive(PartialEq)]
pub struct BuildTimeImageWrapper {
    pub dynamic_image: DynamicImage,
}

impl BuildTimeImageWrapper {}

impl ImageWrapperMethods for BuildTimeImageWrapper {
    fn new(bytes: &'static [u8], _path: PathBuf) -> Self {
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
