use std::path::{Path, PathBuf};

use crate::prelude::*;

#[derive(PartialEq)]
pub struct TextAsset {
    pub path: PathBuf,
    pub content: String,
    pub size_budget: NumBytes,
}

impl NonImageAsset for TextAsset {
    fn path(&self) -> &Path {
        &self.path
    }

    fn bytes(&self) -> Vec<u8> {
        self.content.as_bytes().to_vec()
    }

    fn size_budget(&self) -> NumBytes {
        self.size_budget
    }
}
