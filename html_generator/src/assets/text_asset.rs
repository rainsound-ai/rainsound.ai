use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::prelude::*;

#[derive(PartialEq)]
pub struct TextAsset {
    pub path: PathBuf,
    pub content: String,
    pub load_time_budget: Duration,
}

impl NonImageAsset for TextAsset {
    fn path(&self) -> &Path {
        &self.path
    }

    fn bytes(&self) -> Vec<u8> {
        self.content.as_bytes().to_vec()
    }

    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }
}
