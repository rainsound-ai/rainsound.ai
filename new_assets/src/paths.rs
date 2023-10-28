use std::path::{Path, PathBuf};

#[cfg(feature = "build")]
pub fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = std::env!("CARGO_WORKSPACE_DIR");
    Path::new(&cargo_workspace_dir).to_path_buf()
}

#[cfg(not(feature = "build"))]
pub fn workspace_root_dir() -> PathBuf {
    // If you change this, you also need to change the files mount path in spin.toml.
    Path::new("/").to_path_buf()
}

pub fn built_assets_dir() -> PathBuf {
    workspace_root_dir().join("built_assets")
}
