use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};

pub trait DynamicImageExtension {
    fn resize_to_width(&self, new_width: u32) -> Self;
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
}
