use std::path::{Path, PathBuf};

pub fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = std::env!("CARGO_WORKSPACE_DIR");
    Path::new(&cargo_workspace_dir).to_path_buf()
}

pub fn build_images_dir() -> PathBuf {
    workspace_root_dir().join("assets").join("build_images")
}

pub fn built_image_path(path_starting_from_images_dir: &Path) -> PathBuf {
    build_images_dir()
        .join("target")
        .join(path_starting_from_images_dir)
}
