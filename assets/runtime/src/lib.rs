#![allow(non_upper_case_globals)]

mod asset;
use self::asset::*;

mod browser_crate_asset;
pub use self::browser_crate_asset::BrowserCrateAsset;

mod css_asset;
pub use self::css_asset::CssAsset;

mod image_asset;
pub use self::image_asset::{ImageAsset, LightDarkImageAsset, Placeholder};

mod js_asset;
pub use self::js_asset::JsAsset;

pub mod paths;
pub use self::paths::*;

mod performance_budget;

mod wasm_asset;
pub use self::wasm_asset::WasmAsset;
