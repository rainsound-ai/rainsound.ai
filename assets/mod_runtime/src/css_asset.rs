use crate::asset::{Asset, FileToSave};
use crate::asset_url_path;
use crate::performance_budget::HasPerformanceBudget;
use std::{path::PathBuf, time::Duration};

#[derive(PartialEq)]
pub struct CssAsset {
    pub url_path: PathBuf,
    pub contents: &'static str,
    pub load_time_budget: Duration,
}

impl CssAsset {
    pub fn new(url_path: PathBuf, contents: &'static str, load_time_budget: Duration) -> Self {
        let url_path = asset_url_path(&url_path);
        Self {
            url_path,
            contents,
            load_time_budget,
        }
    }
}

impl Asset for CssAsset {
    fn files_to_save(&self) -> Vec<FileToSave> {
        vec![FileToSave {
            path_starting_from_built_assets_dir: &self.url_path,
            bytes: self.contents.as_bytes(),
            content_type: "text/css",
        }]
    }
}

impl HasPerformanceBudget for CssAsset {
    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }

    fn bytes(&self) -> &[u8] {
        self.contents.as_bytes()
    }

    fn path(&self) -> &std::path::Path {
        &self.url_path
    }
}
