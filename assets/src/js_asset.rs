use crate::non_image_asset::NonImageAsset;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(PartialEq)]
pub struct JsAsset {
    pub path: PathBuf,
    pub contents: &'static str,
    pub load_time_budget: Duration,
}

impl JsAsset {
    pub fn minified_contents(&self) -> Vec<u8> {
        let session = minify_js::Session::new();
        let js_bytes = self.contents.as_bytes();
        let mut out = Vec::new();
        minify_js::minify(
            &session,
            minify_js::TopLevelMode::Module,
            js_bytes,
            &mut out,
        )
        .unwrap();
        out
    }
}

impl NonImageAsset for JsAsset {
    fn path(&self) -> &Path {
        &self.path
    }

    fn bytes(&self) -> Vec<u8> {
        // Unminified. You can comment this back in if you want to debug.
        // self.contents.as_bytes().to_vec()

        // Minified.
        self.minified_contents()
    }

    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }
}
