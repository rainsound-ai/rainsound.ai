use crate::asset::Asset;
use crate::{CanSaveToDisk, HasPerformanceBudget};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(PartialEq)]
pub struct WasmAsset {
    pub file_name: PathBuf,
    pub bytes: &'static [u8],
    pub load_time_budget: Duration,
}

impl CanSaveToDisk for WasmAsset {
    fn save_to_disk(&self) {
        Asset::save_to_disk(self);
    }
}

impl Asset for WasmAsset {
    fn file_name(&self) -> &Path {
        &self.file_name
    }

    fn bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    fn content_type(&self) -> String {
        "application/wasm".to_string()
    }
}

impl HasPerformanceBudget for WasmAsset {
    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }
}
