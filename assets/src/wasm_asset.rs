use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::prelude::*;

#[derive(PartialEq)]
pub struct WasmAsset {
    pub path: PathBuf,
    pub bytes: &'static [u8],
    pub load_time_budget: Duration,
}

impl NonImageAsset for WasmAsset {
    fn path(&self) -> &Path {
        &self.path
    }

    fn bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }
}
