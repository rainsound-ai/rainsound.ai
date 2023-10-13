use crate::asset::Asset;
use image::DynamicImage;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

pub type ResizedImageAsset = BuildTimeResizedImageAsset;

#[derive(PartialEq, Clone)]
pub struct BuildTimeResizedImageAsset {
    path: PathBuf,
    width: u32,
    image: Arc<DynamicImage>,
}

impl Asset for BuildTimeResizedImageAsset {
    fn path(&self) -> &Path {
        &self.path
    }

    fn bytes(&self) -> Vec<u8> {
        if self.needs_to_be_recreated(&path, paths_of_files_in_built_dir) {
            let parent_dir = path.parent().unwrap();
            if !parent_dir.exists() {
                std::fs::create_dir_all(parent_dir).unwrap();
            }

            println!("Saving resized image to disk: {:?}", &self.path);
            self.image
                .resize_to_width(self.width)
                .save_with_format(path, image::ImageFormat::Jpeg)
                .unwrap();

            return;
        }
    }
}

impl BuildTimeResizedImageAsset {
    pub fn save_to_disk(&self, built_dir: &Path, paths_of_files_in_built_dir: &HashSet<PathBuf>) {
        println!("Deciding whether to save resized image to disk.");
        if self.needs_to_be_recreated(&path, paths_of_files_in_built_dir) {
            let parent_dir = path.parent().unwrap();
            if !parent_dir.exists() {
                std::fs::create_dir_all(parent_dir).unwrap();
            }

            println!("Saving resized image to disk: {:?}", &self.path);
            self.image
                .resize_to_width(self.width)
                .save_with_format(path, image::ImageFormat::Jpeg)
                .unwrap();

            return;
        }

        println!(
            "Resized image {} already exists, so skipping saving it to disk.",
            &self.path.to_str().unwrap()
        );
    }

    pub fn needs_to_be_recreated(&self, paths_of_files_in_built_dir: &HashSet<PathBuf>) -> bool {
        let path_to_resized_image_file = self.path_on_disk();
        let already_exists = paths_of_files_in_built_dir.contains(&path_to_resized_image_file);
        !already_exists
    }
}
