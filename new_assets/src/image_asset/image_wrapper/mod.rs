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
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SerializedImageWrapper {
    pub dimensions: (u32, u32),
    pub generated_placeholder: GeneratedPlaceholder,
}

impl SerializedImageWrapper {
    pub fn load_from_disk(image_path: &Path) -> Self {
        let path_on_disk = SerializedImageWrapper::path_on_disk(image_path);
        let serialized = fs::read_to_string(&path_on_disk).unwrap();
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn save_to_disk(&self, image_path: &Path) {
        let path_on_disk = SerializedImageWrapper::path_on_disk(image_path);
        let serialized = serde_json::to_string_pretty(&self).unwrap();
        fs::write(&path_on_disk, &serialized).unwrap();
    }

    fn path_on_disk(image_path: &Path) -> PathBuf {
        let image_path_json = {
            let mut image_path_json = image_path.to_owned();
            image_path_json.push(".json");
            image_path_json
        };

        super::built_images_dir().join(image_path_json)
    }
}
