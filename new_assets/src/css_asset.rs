use crate::non_image_asset::NonImageAsset;
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(PartialEq)]
pub struct CssAsset {
    pub path: PathBuf,
    pub contents: &'static str,
    pub load_time_budget: Duration,
}

impl NonImageAsset for CssAsset {
    fn path(&self) -> &Path {
        &self.path
    }

    fn bytes(&self) -> Vec<u8> {
        self.contents.as_bytes().to_vec()
    }

    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }

    fn content_type(&self) -> String {
        "text/css".to_string()
    }
}
