use std::fs;
use std::path::{Path, PathBuf};

/// Most assets like CSS stylesheets have a single corresponding file.
/// In those cases, they can implement this trait and it'll take care of
/// saving the file in the right spot automatically.
pub trait Asset: Send + Sync {
    fn files_to_save(&self) -> Vec<FileToSave>;

    fn save_to_disk(&self) {
        for file_to_save in self.files_to_save() {
            file_to_save.save_to_disk();
        }
    }
}

pub struct FileToSave<'a> {
    pub file_name: &'a Path,
    pub bytes: &'a [u8],
    pub content_type: &'a str,
}

impl<'a> FileToSave<'a> {
    fn save_to_disk(&self) {
        let path = self.path_on_disk();

        let parent_dir = path.parent().unwrap();
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }

        if let Err(error) = fs::remove_file(&path) {
            println!("Error removing file: {}", error);
        }

        fs::write(path, self.bytes).unwrap();
    }

    fn path_on_disk(&self) -> PathBuf {
        crate::built_assets_dir().join(self.file_name)
    }

    /// The path used when loading assets in the browser.
    fn path(&self) -> PathBuf {
        path_for_asset_in_browser(self.file_name)
    }
}

pub fn path_for_asset_in_browser(file_name: &Path) -> PathBuf {
    crate::built_assets_browser_prefix().join(file_name)
}
