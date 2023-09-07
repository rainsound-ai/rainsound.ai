use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::prelude::*;

#[derive(Debug)]
pub struct HtmlAsset {
    pub path: PathBuf,
    pub contents: String,
    pub load_time_budget: Duration,
}

impl HtmlAsset {
    fn minified_contents(&self) -> Vec<u8> {
        let mut minify_html_config = minify_html::Cfg::new();
        minify_html_config.minify_js = true;
        minify_html::minify(self.contents.as_bytes(), &minify_html_config)
    }
}

impl NonImageAsset for HtmlAsset {
    fn path(&self) -> &Path {
        &self.path
    }

    fn bytes(&self) -> Vec<u8> {
        // Un-minified. You can comment this back in if you want to debug.
        // self.contents.as_bytes().to_vec()

        // Minified.
        self.minified_contents()
    }

    fn load_time_budget(&self) -> Duration {
        self.load_time_budget
    }
}
