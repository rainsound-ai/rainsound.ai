use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "build")] {
    mod build_time_image_wrapper;
    pub use build_time_image_wrapper::BuildTimeImageWrapper;
    pub type ImageWrapper = BuildTimeImageWrapper;
} else {
    mod run_time_image_wrapper;
    pub use run_time_image_wrapper::RunTimeImageWrapper;
    pub type ImageWrapper = RunTimeImageWrapper;
}
}

use super::*;

pub trait ImageWrapperMethods: Sync + Send + PartialEq {
    fn new(bytes: &'static [u8], path: PathBuf) -> Self
    where
        Self: Sized;
    fn dimensions(&self) -> (u32, u32);
    fn generate_placeholder(&self, placeholder: Placeholder) -> GeneratedPlaceholder;
    fn width(&self) -> u32;
    fn mime_type(&self) -> MimeType;
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SerializedImageWrapper {
    pub dimensions: (u32, u32),
    pub generated_placeholder: GeneratedPlaceholder,
    pub mime_type: MimeType,
}

impl SerializedImageWrapper {
    pub fn load_from_disk(image_path: &Path) -> Self {
        let path_on_disk = SerializedImageWrapper::path_on_disk(image_path);
        let serialized = fs::read_to_string(path_on_disk).unwrap();
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn save_to_disk(&self, image_path: &Path) {
        let path_on_disk = SerializedImageWrapper::path_on_disk(image_path);
        let parent_dir = path_on_disk.parent().unwrap();
        if !parent_dir.exists() {
            println!("Creating directory: {:?}", parent_dir);
            std::fs::create_dir_all(parent_dir).unwrap();
        }

        let serialized = serde_json::to_string_pretty(&self).unwrap();
        fs::write(path_on_disk, serialized).unwrap();
    }

    fn path_on_disk(image_path: &Path) -> PathBuf {
        let original_file_name = image_path
            .file_name()
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();
        let file_name = original_file_name + ".json";
        crate::built_images_dir().join(file_name)
    }
}
