use crate::prelude::*;

#[derive(PartialEq)]
pub struct CssAsset {
    pub asset_path: &'static str,
    pub contents: &'static str,
    pub size_budget: NumBytes,
}

impl HasSizeBudget for CssAsset {
    fn size_budget(&self) -> NumBytes {
        self.size_budget
    }

    fn check_size_budget(&self) -> HowCloseToBudget {
        NonImageAsset::check_size_budget(self)
    }
}

impl NonImageAsset for CssAsset {
    fn asset_path(&self) -> &str {
        self.asset_path
    }

    fn bytes(&self) -> Vec<u8> {
        self.contents.as_bytes().to_vec()
    }
}
