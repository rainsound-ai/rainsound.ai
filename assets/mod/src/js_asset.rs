use crate::performance_budget::HasPerformanceBudget;
use crate::{Asset, FileToSave};
use std::path::PathBuf;
use std::time::Duration;

#[derive(PartialEq)]
pub struct JsAsset {
    pub file_name: PathBuf,
    pub contents: &'static str,
    pub load_time_budget: Duration,
}

impl Asset for JsAsset {
    fn files_to_save(&self) -> Vec<FileToSave> {
        vec![FileToSave {
            file_name: &self.file_name,
            bytes: self.contents.as_bytes(),
            content_type: "application/javascript",
        }]
    }
}

impl HasPerformanceBudget for JsAsset {
    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }

    fn bytes(&self) -> &[u8] {
        self.contents.as_bytes()
    }

    fn path(&self) -> &std::path::Path {
        &self.file_name
    }
}
