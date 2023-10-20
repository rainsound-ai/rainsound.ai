use std::env;
use std::path::{Path, PathBuf};

pub fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = env::var("CARGO_WORKSPACE_DIR").unwrap();
    Path::new(&cargo_workspace_dir).to_path_buf()
}

pub fn built_assets_dir() -> PathBuf {
    workspace_root_dir().join("built")
}

pub fn built_images_dir() -> PathBuf {
    built_assets_dir().join("images")
}
