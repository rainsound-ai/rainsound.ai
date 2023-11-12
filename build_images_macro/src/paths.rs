use std::path::{Path, PathBuf};

pub fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = std::env!("CARGO_WORKSPACE_DIR");
    Path::new(&cargo_workspace_dir).to_path_buf()
}

pub fn build_images_dir() -> PathBuf {
    workspace_root_dir().join("build_images")
}

pub fn built_image_path_from_file_name(file_name: &Path) -> PathBuf {
    crate::paths::build_images_dir()
        .join("target")
        .join(file_name)
}
