use crate::prelude::*;

#[derive(PartialEq)]
pub struct WasmAsset {
    pub asset_path: &'static str,
    pub bytes: &'static [u8],
}

impl NonImageAsset for WasmAsset {
    fn asset_path(&self) -> &str {
        self.asset_path
    }

    fn bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
}
