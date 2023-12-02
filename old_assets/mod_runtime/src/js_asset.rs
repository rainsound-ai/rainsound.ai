use crate::performance_budget::HasPerformanceBudget;
use crate::{asset_url_path, Asset, FileToSave};
use std::path::PathBuf;
use std::time::Duration;

#[derive(PartialEq)]
pub struct JsAsset {
    pub full_url_path: PathBuf, // Used for loading the asset in the browser.
    pub url_path_starting_from_built_assets_dir: PathBuf, // Used for saving the asset to disk.
    pub contents: &'static str,
    pub load_time_budget: Duration,
}

impl JsAsset {
    pub fn new(
        url_path_starting_from_built_assets_dir: PathBuf,
        contents: &'static str,
        load_time_budget: Duration,
    ) -> Self {
        let full_url_path = asset_url_path(&url_path_starting_from_built_assets_dir);
        Self {
            full_url_path,
            url_path_starting_from_built_assets_dir,
            contents,
            load_time_budget,
        }
    }
}

impl Asset for JsAsset {
    fn files_to_save(&self) -> Vec<FileToSave> {
        vec![FileToSave {
            path_starting_from_built_assets_dir: &self.url_path_starting_from_built_assets_dir,
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

    fn path_for_reporting_asset_over_budget(&self) -> &std::path::Path {
        &self.url_path_starting_from_built_assets_dir
    }
}
