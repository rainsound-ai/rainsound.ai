use crate::image_asset::image_wrapper::RunTimeImageWrapper;
use crate::{asset::Asset, CanSaveToDisk};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(PartialEq, Clone)]
pub struct RunTimeResizedImageAsset {
    pub file_name: PathBuf,
    pub width: u32,
    pub image: Arc<RunTimeImageWrapper>,
}

impl CanSaveToDisk for RunTimeResizedImageAsset {
    fn save_to_disk(&self) {
        Asset::save_to_disk(self);
    }
}

impl Asset for RunTimeResizedImageAsset {
    fn file_name(&self) -> &Path {
        &self.file_name
    }

    fn bytes(&self) -> Vec<u8> {
        let path_to_resized_image_file = self.path_on_disk();
        let error_message = format!(
            "Expected resized image to exist at path: {:?}",
            &path_to_resized_image_file
        );
        fs::read(&path_to_resized_image_file).expect(&error_message)
    }

    fn content_type(&self) -> String {
        "image/jpeg".to_string()
    }
}
