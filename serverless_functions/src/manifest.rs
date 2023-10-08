use std::env;
use std::path::{Path, PathBuf};

pub fn dir() -> PathBuf {
    let cargo_workspace_dir = env::var("CARGO_WORKSPACE_DIR").unwrap();
    Path::new(&cargo_workspace_dir).to_path_buf()
}
