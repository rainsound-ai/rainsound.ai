use crate::non_image_asset::NonImageAsset;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(PartialEq)]
pub struct JsAsset {
    pub path: PathBuf,
    pub contents: &'static str,
    pub load_time_budget: Duration,
}

impl JsAsset {}

impl NonImageAsset for JsAsset {
    fn path(&self) -> &Path {
        &self.path
    }

    fn bytes(&self) -> Vec<u8> {
        // Unminified. You can comment this back in if you want to debug.
        self.contents.as_bytes().to_vec()

        // Minified.
        // self.minified_contents()
    }

    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }

    fn content_type(&self) -> String {
        "application/javascript".to_string()
    }
}
