use crate::prelude::*;

#[derive(PartialEq)]
pub struct WasmAsset {
    pub asset_path: &'static str,
    pub bytes: &'static [u8],
    pub size_budget: NumBytes,
}

impl HasSizeBudget for WasmAsset {
    fn size_budget(&self) -> NumBytes {
        self.size_budget
    }

    fn check_size_budget(&self) -> HowCloseToBudget {
        NonImageAsset::check_size_budget(self)
    }
}

impl NonImageAsset for WasmAsset {
    fn asset_path(&self) -> &str {
        self.asset_path
    }

    fn bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
}
