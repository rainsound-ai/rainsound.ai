use crate::asset::Asset;
use crate::{CanSaveToDisk, HasPerformanceBudget};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(PartialEq)]
pub struct JsAsset {
    pub file_name: PathBuf,
    pub contents: &'static str,
    pub load_time_budget: Duration,
}

impl CanSaveToDisk for JsAsset {
    fn save_to_disk(&self) {
        Asset::save_to_disk(self);
    }
}

impl Asset for JsAsset {
    fn file_name(&self) -> &Path {
        &self.file_name
    }

    fn bytes(&self) -> Vec<u8> {
        // Unminified. You can comment this back in if you want to debug.
        self.contents.as_bytes().to_vec()

        // Minified.
        // self.minified_contents()
    }

    fn content_type(&self) -> String {
        "application/javascript".to_string()
    }
}

impl HasPerformanceBudget for JsAsset {
    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }
}
