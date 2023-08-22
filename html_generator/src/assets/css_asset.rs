use crate::prelude::*;

#[derive(PartialEq)]
pub struct CssAsset {
    pub asset_path: &'static str,
    pub contents: &'static str,
}

impl NonImageAsset for CssAsset {
    fn asset_path(&self) -> &str {
        self.asset_path
    }

    fn bytes(&self) -> Vec<u8> {
        self.contents.as_bytes().to_vec()
    }
}
