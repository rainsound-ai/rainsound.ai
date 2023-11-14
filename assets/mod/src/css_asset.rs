use crate::{Asset, FileToSave, HasPerformanceBudget};
use std::{path::PathBuf, time::Duration};

#[derive(PartialEq)]
pub struct CssAsset {
    pub file_name: PathBuf,
    pub contents: &'static str,
    pub load_time_budget: Duration,
}

impl Asset for CssAsset {
    fn files_to_save(&self) -> Vec<FileToSave> {
        vec![FileToSave {
            file_name: &self.file_name,
            bytes: &self.contents.as_bytes(),
            content_type: "text/css",
        }]
    }
}

impl HasPerformanceBudget for CssAsset {
    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }
}
