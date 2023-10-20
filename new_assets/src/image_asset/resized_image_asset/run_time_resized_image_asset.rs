use crate::image_asset::image_wrapper::RunTimeImageWrapper;
use crate::{asset::Asset, CanSaveToDisk};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(PartialEq, Clone)]
pub struct RunTimeResizedImageAsset {
    pub path: PathBuf,
    pub width: u32,
    pub image: Arc<RunTimeImageWrapper>,
}

impl CanSaveToDisk for RunTimeResizedImageAsset {
    fn save_to_disk(&self) {
        Asset::save_to_disk(self);
    }
}

impl Asset for RunTimeResizedImageAsset {
    fn path(&self) -> &Path {
        &self.path
    }

    fn bytes(&self) -> Vec<u8> {
        let path_to_resized_image_file = self.path_on_disk();
        return fs::read(&path_to_resized_image_file)
            .expect("Expected resized image to exist at path: {:?}", &self.path);
    }

    fn content_type(&self) -> String {
        "image/jpeg".to_string()
    }
}
