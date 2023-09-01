use crate::prelude::*;

#[derive(PartialEq)]
pub struct TextAsset {
    pub asset_path: &'static str,
    pub content: String,
    pub size_budget: NumBytes,
}

impl HasSizeBudget for TextAsset {
    fn size_budget(&self) -> NumBytes {
        self.size_budget
    }

    fn check_size_budget(&self) -> HowCloseToBudget {
        NonImageAsset::check_size_budget(self)
    }
}

impl NonImageAsset for TextAsset {
    fn asset_path(&self) -> &str {
        self.asset_path
    }

    fn bytes(&self) -> Vec<u8> {
        self.content.as_bytes().to_vec()
    }
}
