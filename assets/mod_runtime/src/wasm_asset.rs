use crate::performance_budget::HasPerformanceBudget;
use crate::{asset_url_path, Asset, FileToSave};
use std::path::PathBuf;
use std::time::Duration;

#[derive(PartialEq)]
pub struct WasmAsset {
    pub full_url_path: PathBuf, // Used for loading the asset in the browser.
    pub url_path_starting_from_built_assets_dir: PathBuf, // Used for saving the asset to disk.
    pub bytes: &'static [u8],
    pub load_time_budget: Duration,
}

impl WasmAsset {
    pub fn new(
        url_path_starting_from_built_assets_dir: PathBuf,
        bytes: &'static [u8],
        load_time_budget: Duration,
    ) -> Self {
        let full_url_path = asset_url_path(&url_path_starting_from_built_assets_dir);
        Self {
            full_url_path,
            url_path_starting_from_built_assets_dir,
            bytes,
            load_time_budget,
        }
    }
}

impl Asset for WasmAsset {
    fn files_to_save(&self) -> Vec<crate::FileToSave> {
        vec![FileToSave {
            path_starting_from_built_assets_dir: &self.url_path_starting_from_built_assets_dir,
            bytes: self.bytes,
            content_type: "application/wasm",
        }]
    }
}

impl HasPerformanceBudget for WasmAsset {
    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }

    fn bytes(&self) -> &[u8] {
        self.bytes
    }

    fn path_for_reporting_asset_over_budget(&self) -> &std::path::Path {
        &self.url_path_starting_from_built_assets_dir
    }
}
