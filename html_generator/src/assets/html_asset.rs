use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use strum::IntoEnumIterator;

use crate::prelude::*;
use crate::routes::{Route, RouteNames};

#[derive(Debug)]
pub struct HtmlAsset {
    pub path: PathBuf,
    pub contents: String,
    pub load_time_budget: Duration,
}

impl HtmlAsset {
    pub fn get_pages() -> Vec<HtmlAsset> {
        RouteNames::iter()
            .map(|route_name| {
                let route = route_name.route();

                match route {
                    Route::Page { path, html } => {
                        let path_without_leading_slash = path.strip_prefix('/').unwrap();
                        let path_buf = PathBuf::from_str(path_without_leading_slash)
                            .unwrap()
                            .join("index.html");

                        HtmlAsset {
                            path: path_buf,
                            contents: html.into_string(),
                            load_time_budget: Duration::from_millis(1),
                        }
                    }
                }
            })
            .collect()
    }

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
