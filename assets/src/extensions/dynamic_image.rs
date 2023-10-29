use base64::Engine;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::io::Cursor;

use crate::MimeType;

pub trait DynamicImageExtension {
    fn resize_to_width(&self, new_width: u32) -> Self;
    fn to_data_uri(&self) -> DataUriAndMimeType;
    fn into_bytes_with_format(&self, format: ImageFormat) -> Vec<u8>;
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
        let bytes = self.into_bytes_with_format(ImageFormat::Jpeg);
        let base64_encoded = base64::engine::general_purpose::STANDARD.encode(bytes);

        let mime_type = MimeType::ImageJpeg;

        let data_uri = format!(
            "data:{mime_type};base64,{base64}",
            mime_type = mime_type.to_string(),
            base64 = base64_encoded
        );

        DataUriAndMimeType {
            mime_type,
            data_uri,
        }
    }

    fn into_bytes_with_format(&self, format: ImageFormat) -> Vec<u8> {
        let mut bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        self.write_to(&mut bytes, format)
            .expect("Error encoding image.");
        bytes.into_inner()
    }
}

pub struct DataUriAndMimeType {
    pub mime_type: MimeType,
    pub data_uri: String,
}
