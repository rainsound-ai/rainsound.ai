use std::fs;
use std::path::{Path, PathBuf};

use crate::ContentType;

pub trait CanSaveToDisk: Send + Sync {
    fn save_to_disk(&self);
}

pub trait Asset: CanSaveToDisk {
    fn file_name(&self) -> &Path;

    fn path(&self) -> PathBuf {
        PathBuf::from("built-assets").join(self.file_name())
    }

    fn bytes(&self) -> Vec<u8>;

    fn content_type(&self) -> ContentType;

    fn save_to_disk(&self) {
        println!("Saving asset: {:?}", self.path());
        let path = self.path_on_disk();

        let parent_dir = path.parent().unwrap();
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }

        if let Err(error) = fs::remove_file(&path) {
            println!("Error removing file: {}", error);
        }

        let bytes = self.bytes();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    fn path_on_disk(&self) -> PathBuf {
        crate::built_assets_dir().join(self.file_name())
    }
}
