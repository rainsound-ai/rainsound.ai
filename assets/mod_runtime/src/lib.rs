#![allow(non_upper_case_globals)]

use arraygen::Arraygen;
use build_browser::build_browser_crate;
use build_images::*;
use build_tailwind::build_tailwind;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

pub mod asset;
pub use self::asset::*;

pub mod css_asset;
pub use self::css_asset::*;

pub mod image_asset;
pub use self::image_asset::*;

pub mod js_asset;
pub use self::js_asset::*;

pub mod paths;
pub use self::paths::*;

pub mod performance_budget;
pub use self::performance_budget::*;

pub mod wasm_asset;
pub use self::wasm_asset::*;

pub static non_html_assets: Lazy<NonHtmlAssets> = Lazy::new(NonHtmlAssets::new);

#[derive(Arraygen)]
#[gen_array(pub fn all_assets: &dyn Asset, implicit_select_all: CssAsset, JsAsset, WasmAsset, ImageAsset, LightDarkImageAsset)]
pub struct NonHtmlAssets {
    pub built_css: CssAsset,
    pub browser_js: JsAsset,
    pub browser_bg_wasm: WasmAsset,
    pub hasui_hero: ImageAsset,
}

// We have to separate out the non-html assets because
// we want to reference them when generating html.
//
// If we didn't do this, we'd have a circular dependency.
// This causes problems. For example, it can lead to
// deadlocking if we're using a lazily initialized global variable.
impl NonHtmlAssets {
    pub fn new() -> Self {
        let built_css = build_tailwind!(
            path_to_input_file: "serverless_functions/src/main.css",
            minify: true,
            debug: true
        );
        let built_css = CssAsset {
            url_path: PathBuf::from_str("built.css").unwrap(),
            contents: built_css,
            load_time_budget: Duration::from_millis(1),
        };

        let browser_crate = build_browser_crate!(
            path_to_browser_crate: "browser",
            production: true,
            debug: true,
        );
        let browser_js = JsAsset {
            url_path: PathBuf::from_str("browser.js").unwrap(),
            contents: browser_crate.built_js,
            load_time_budget: Duration::from_millis(1),
        };
        let browser_bg_wasm = WasmAsset {
            url_path: PathBuf::from_str("browser_bg.wasm").unwrap(),
            bytes: browser_crate.built_wasm,
            load_time_budget: Duration::from_millis(1),
        };

        let built_images =
            build_images!(path_to_images_dir: "assets/mod_runtime/src/original_images");
        let hasui_hero = ImageAsset::from_built_image(
            "A woodblock print by Kawase Hasui",
            Placeholder::Lqip,
            built_images.hasui_light,
        );

        NonHtmlAssets {
            built_css,
            browser_js,
            browser_bg_wasm,
            hasui_hero,
        }
    }

    // fn non_image_assets(&self) -> Vec<&dyn Asset> {
    //     let css_assets = self.css_assets().into_iter().map(|css_asset| {
    //         let css_asset: &dyn Asset = css_asset;
    //         css_asset
    //     });
    //     let js_assets = self.js_assets().into_iter().map(|js_asset| {
    //         let js_asset: &dyn Asset = js_asset;
    //         js_asset
    //     });
    //     let wasm_assets = self.wasm_assets().into_iter().map(|wasm_asset| {
    //         let wasm_asset: &dyn Asset = wasm_asset;
    //         wasm_asset
    //     });
    //     css_assets.chain(js_assets).chain(wasm_assets).collect()
    // }
}

impl Default for NonHtmlAssets {
    fn default() -> Self {
        Self::new()
    }
}
