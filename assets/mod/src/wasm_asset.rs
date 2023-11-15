use crate::performance_budget::HasPerformanceBudget;
use crate::{Asset, FileToSave};
use std::path::PathBuf;
use std::time::Duration;

#[derive(PartialEq)]
pub struct WasmAsset {
    pub file_name: PathBuf,
    pub bytes: &'static [u8],
    pub load_time_budget: Duration,
}

impl Asset for WasmAsset {
    fn files_to_save(&self) -> Vec<crate::FileToSave> {
        vec![FileToSave {
            file_name: &self.file_name,
            bytes: &self.bytes,
            content_type: "application/wasm",
        }]
    }
}

impl HasPerformanceBudget for WasmAsset {
    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }
}
