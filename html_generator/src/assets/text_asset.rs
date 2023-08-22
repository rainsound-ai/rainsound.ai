use crate::prelude::*;

#[derive(PartialEq)]
pub struct TextAsset {
    pub asset_path: &'static str,
    pub content: &'static str,
}

impl NonImageAsset for TextAsset {
    fn asset_path(&self) -> &str {
        self.asset_path
    }

    fn bytes(&self) -> Vec<u8> {
        self.content.as_bytes().to_vec()
    }
}
