use crate::prelude::*;
use arraygen::Arraygen;
use std::any::Any;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

mod html_asset;
pub use self::html_asset::*;

#[derive(Arraygen)]
#[gen_array(fn html_assets_with_performance_budget: &dyn NonImageAsset, implicit_select_all: HtmlAsset)]
pub struct Assets {
    html_assets: Vec<HtmlAsset>,
}

pub trait NonImageAsset {
    fn path(&self) -> &Path;

    fn bytes(&self) -> Vec<u8>;

    fn save_to_disk(&self, built_dir: &Path) {
        println!("Saving asset: {:?}", self.path());
        let path = self.path_on_disk(built_dir);

        if let Err(error) = fs::remove_file(&path) {
            println!("Error removing file: {}", error);
        }

        let bytes = self.bytes();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    fn path_on_disk(&self, built_dir: &Path) -> PathBuf {
        Assets::path_on_disk(built_dir, self.path())
    }

    // Used for enforcing performance budgets.
    fn load_time_budget(&self) -> Duration;
}

impl Assets {
    pub async fn new() -> Assets {
        let html_assets = HtmlAsset::get_pages().await;
        Assets { html_assets }
    }

    pub fn save_to_disk(&self, built_dir: &Path) {
        dbg!(&self.html_assets);

        for html_asset in &self.html_assets {
            println!("Saving asset: {:?}", html_asset.path());
            html_asset.save_to_disk(built_dir);
        }
    }

    pub fn built_dir() -> PathBuf {
        crate::manifest::dir().join("built")
    }

    fn path_on_disk(built_dir: &Path, asset_path: &Path) -> PathBuf {
        built_dir.join(asset_path)
    }
}
