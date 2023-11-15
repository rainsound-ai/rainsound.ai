#![allow(non_upper_case_globals)]

use arraygen::Arraygen;
use build_browser::build_browser_crate;
use build_images::*;
use build_tailwind::build_tailwind;
use cfg_if::cfg_if;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

pub mod asset;
pub use self::asset::*;

pub mod css_asset;
pub use self::css_asset::*;

pub mod content_type;
pub use self::content_type::*;

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
type ContentType = String;

#[derive(Arraygen)]
#[gen_array(pub fn all_assets: &dyn Asset, implicit_select_all: dyn Asset)]
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
        let tailwind_output = build_tailwind!(
            path_to_input_file: "serverless_functions/src/main.css",
            minify: true
        );
        let built_css = CssAsset {
            file_name: PathBuf::from_str("built.css").unwrap(),
            contents: tailwind_output,
            load_time_budget: Duration::from_millis(1),
        };

        let browser_crate = build_browser_crate!(
            path_to_browser_crate: "browser",
            production: true
        );
        let browser_js = JsAsset {
            file_name: PathBuf::from_str("browser.js").unwrap(),
            contents: browser_crate.built_js,
            load_time_budget: Duration::from_millis(1),
        };
        let browser_bg_wasm = WasmAsset {
            file_name: PathBuf::from_str("browser_bg.wasm").unwrap(),
            bytes: browser_crate.built_wasm,
            load_time_budget: Duration::from_millis(1),
        };

        let built_images = build_images!(path_to_images_dir: "assets/mod/src/original_images");
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

cfg_if! {
if #[cfg(feature = "build")] {
    use rayon::prelude::*;

    impl NonHtmlAssets {
        pub fn save_to_disk(&self) {
            self.all_assets()
                .into_iter()
                .par_bridge()
                .for_each(|asset| {
                    asset.save_to_disk();
                });
        }

        // fn all_assets_that_can_be_saved_to_disk(&self) -> Vec<&dyn CanSaveToDisk> {
        //     let css_assets = self.css_assets().into_iter().map(|asset| asset as &dyn CanSaveToDisk);
        //     let js_assets = self.js_assets().into_iter().map(|asset| asset as &dyn CanSaveToDisk);
        //     let wasm_assets = self.wasm_assets().into_iter().map(|asset| asset as &dyn CanSaveToDisk);
        //     let non_image_assets = self
        //         .non_image_assets_as_can_save_to_disk()
        //         .into_iter()
        //         .map(|asset| asset as &dyn CanSaveToDisk);

        //     let image_assets = self.all_image_assets().into_iter().map(|asset| asset as &dyn CanSaveToDisk);
        //     non_image_assets.chain(image_assets).collect()
        // }

        // fn all_image_assets(&self) -> Vec<&dyn CanSaveToDisk> {
        //     let images = non_html_assets.image_assets().into_iter().map(|image_asset| {
        //         let image_asset: &dyn CanSaveToDisk = image_asset;
        //         image_asset
        //     });

        //     let light_dark_images =
        //         non_html_assets
        //             .light_dark_image_assets()
        //             .into_iter()
        //             .map(|light_dark_image_asset| {
        //                 let light_dark_image_asset: &dyn CanSaveToDisk = light_dark_image_asset;
        //                 light_dark_image_asset
        //             });

        //     images.chain(light_dark_images).collect()
        // }
    }
}
}

impl Default for NonHtmlAssets {
    fn default() -> Self {
        Self::new()
    }
}
