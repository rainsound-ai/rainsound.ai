use arraygen::Arraygen;
use new_assets::non_image_asset::NonImageAsset;
use new_assets::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use crate::extensions::StringExtension;

pub static non_html_assets: Lazy<NonHtmlAssets> = Lazy::new(NonHtmlAssets::new);
pub static non_html_assets_by_path: Lazy<HashMap<String, (ContentType, Vec<u8>)>> =
    Lazy::new(|| non_html_assets.by_path());

type ContentType = String;

#[derive(PartialEq, Arraygen)]
#[gen_array(pub fn all_assets: &dyn NonImageAsset, implicit_select_all: CssAsset, JsAsset, WasmAsset, TextAsset)]
pub struct NonHtmlAssets {
    pub built_css: CssAsset,
    pub browser_js: JsAsset,
    pub browser_bg_wasm: WasmAsset,
    // pub build_time: TextAsset,
    // pub images: ImageAssets,
}

// We have to separate out the non-html assets because
// we want to reference them when generating html.
//
// If we didn't do this, we'd have a circular dependency.
// This causes problems. For example, it can lead to
// deadlocking if we're using a lazily initialized global variable.
impl NonHtmlAssets {
    pub fn new() -> Self {
        let built_css = CssAsset {
            path: PathBuf::from_str("built.css").unwrap(),
            contents: include_str!("../../../target/tailwind/built.css"),
            load_time_budget: Duration::from_millis(1),
        };

        let browser_js = JsAsset {
            path: PathBuf::from_str("browser.js").unwrap(),
            contents: include_str!("../../../target/browser/browser.js"),
            load_time_budget: Duration::from_millis(1),
        };

        let browser_bg_wasm = WasmAsset {
            path: PathBuf::from_str("browser_bg.wasm").unwrap(),
            bytes: include_bytes!("../../../target/browser/browser_bg.wasm"),
            load_time_budget: Duration::from_millis(1),
        };

        NonHtmlAssets {
            built_css,
            browser_js,
            browser_bg_wasm,
        }
    }

    fn by_path(&self) -> HashMap<String, (ContentType, Vec<u8>)> {
        let all_assets = self.all_assets();
        let mut hashmap = HashMap::new();
        for asset in all_assets {
            let path = asset
                .path()
                .to_string_lossy()
                .to_string()
                .with_leading_slash();
            let content_type = asset.content_type().to_owned();
            let bytes = asset.bytes().to_owned();
            hashmap.insert(path, (content_type, bytes));
        }
        hashmap
    }
}
