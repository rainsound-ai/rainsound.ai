#![allow(non_upper_case_globals)]
use std::path::{Path, PathBuf};

pub mod css_asset;
pub use self::css_asset::*;

pub mod extensions;
pub use self::extensions::*;

pub mod non_image_asset;
pub use self::non_image_asset::*;

pub mod performance_budget;
pub use self::performance_budget::*;

mod prelude;
pub use self::prelude::*;

pub mod tailwind;
pub use self::tailwind::*;

pub mod workspace_root;
pub use self::workspace_root::*;

pub fn built_assets_dir() -> PathBuf {
    workspace_root::dir().join("built")
}

fn path_for_asset_on_disk(asset_path: &Path) -> PathBuf {
    built_assets_dir().join(asset_path)
}
