use crate::{asset::Asset, CanSaveToDisk, HasPerformanceBudget};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(PartialEq)]
pub struct CssAsset {
    pub file_name: PathBuf,
    pub contents: &'static str,
    pub load_time_budget: Duration,
}

impl CanSaveToDisk for CssAsset {
    fn save_to_disk(&self) {
        Asset::save_to_disk(self);
    }
}

impl Asset for CssAsset {
    fn file_name(&self) -> &Path {
        &self.file_name
    }

    fn bytes(&self) -> Vec<u8> {
        self.contents.as_bytes().to_vec()
    }

    fn content_type(&self) -> String {
        "text/css".to_string()
    }
}

impl HasPerformanceBudget for CssAsset {
    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }
}
