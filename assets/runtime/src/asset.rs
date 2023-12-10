use std::fs;
use std::path::{Path, PathBuf};

pub trait Asset: Send + Sync {
    fn files_to_save(&self) -> Vec<FileToSave>;

    fn save_to_disk(&self) {
        let files_to_save = self.files_to_save();
        log::info!(
            "This asset has {} files to save to disk.",
            files_to_save.len()
        );
        for file_to_save in files_to_save {
            file_to_save.save_to_disk();
        }
    }
}

pub struct FileToSave<'a> {
    pub path_starting_from_built_assets_dir: &'a Path,
    pub bytes: &'a [u8],
    pub content_type: &'a str,
}

impl<'a> FileToSave<'a> {
    fn save_to_disk(&self) {
        let path = self.path_on_disk();
        log::info!("Saving file to disk: {:?}", path);

        let parent_dir = path.parent().unwrap();
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }

        if let Err(error) = fs::remove_file(&path) {
            println!("Error removing file: {}", error);
        }

        fs::write(&path, self.bytes).unwrap();
        log::info!("Done saving asset to disk: {:?}", &path);
    }

    fn path_on_disk(&self) -> PathBuf {
        crate::built_assets_dir().join(self.path_starting_from_built_assets_dir)
    }
}
