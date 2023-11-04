#![allow(non_upper_case_globals)]

use arraygen::Arraygen;
use cfg_if::cfg_if;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

pub mod asset;
pub use self::asset::*;

pub mod css_asset;
pub use self::css_asset::*;

pub mod content_type;
pub use self::content_type::*;

pub mod extensions;
pub use self::extensions::*;

pub mod image_asset;
pub use self::image_asset::*;

pub mod js_asset;
pub use self::js_asset::*;

pub mod mime_type;
pub use self::mime_type::*;

pub mod paths;
pub use self::paths::*;

pub mod performance_budget;
pub use self::performance_budget::*;

mod prelude;
pub use self::prelude::*;

pub mod wasm_asset;
pub use self::wasm_asset::*;

// Desired order:
// 1. Build Tailwind + browser crate.
// 2. Build non_html_assets. Evaluate include_str!() and include_bytes!().
// 3. Save non_html_assets to disk.

pub static non_html_assets: Lazy<NonHtmlAssets> = Lazy::new(NonHtmlAssets::new);
pub static non_html_assets_by_path: Lazy<HashMap<String, (ContentType, Vec<u8>)>> =
    Lazy::new(|| non_html_assets.by_path());
type ContentType = String;

// #[gen_array(pub fn non_image_assets_as_can_save_to_disk: &dyn CanSaveToDisk, implicit_select_all: CssAsset, JsAsset, WasmAsset)]
#[derive(PartialEq, Arraygen)]
#[gen_array(pub fn image_assets: &ImageAsset, implicit_select_all: ImageAsset)]
#[gen_array(pub fn light_dark_image_assets: &LightDarkImageAsset, implicit_select_all: LightDarkImageAsset)]
#[gen_array(pub fn css_assets: &CssAsset, implicit_select_all: CssAsset)]
#[gen_array(pub fn js_assets: &JsAsset, implicit_select_all: JsAsset)]
#[gen_array(pub fn wasm_assets: &WasmAsset, implicit_select_all: WasmAsset)]
pub struct NonHtmlAssets {
    pub built_css: CssAsset,
    pub browser_js: JsAsset,
    pub browser_bg_wasm: WasmAsset,
    pub hasui_hero: ImageAsset,
    // pub build_time: TextAsset,
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
            file_name: PathBuf::from_str("built.css").unwrap(),
            contents: build_tailwind::built_css,
            load_time_budget: Duration::from_millis(1),
        };

        let browser_js = JsAsset {
            file_name: PathBuf::from_str("browser.js").unwrap(),
            contents: build_browser::built_js,
            load_time_budget: Duration::from_millis(1),
        };

        let browser_bg_wasm = WasmAsset {
            file_name: PathBuf::from_str("browser_bg.wasm").unwrap(),
            bytes: build_browser::built_wasm,
            load_time_budget: Duration::from_millis(1),
        };

        let hasui_hero = ImageAsset::new(
            PathBuf::from_str("hasui_hero.jpg").unwrap(),
            "A woodblock print by Kawase Hasui",
            include_bytes!("./original_images/hasui_light.jpeg"),
            Placeholder::Lqip,
        );

        NonHtmlAssets {
            built_css,
            browser_js,
            browser_bg_wasm,
            hasui_hero,
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

    fn all_assets(&self) -> Vec<&dyn Asset> {
        let non_image_assets = self.non_image_assets();
        let resized_image_assets =
            self.resized_image_assets()
                .into_iter()
                .map(|resized_image_asset| {
                    let resized_image_asset: &dyn Asset = resized_image_asset;
                    resized_image_asset
                });
        non_image_assets
            .into_iter()
            .chain(resized_image_assets)
            .collect()
    }

    fn resized_image_assets(&self) -> Vec<&ResizedImageAsset> {
        let resized_variants: Vec<&ResizedImageAsset> = non_html_assets
            .image_assets()
            .into_iter()
            .flat_map(|image_asset| &image_asset.resized_variants)
            .collect::<Vec<_>>();

        let light_dark_resized_variants: Vec<&ResizedImageAsset> = non_html_assets
            .light_dark_image_assets()
            .into_iter()
            .flat_map(|light_dark_image_asset| light_dark_image_asset.resized_variants())
            .collect::<Vec<_>>();

        resized_variants
            .into_iter()
            .chain(light_dark_resized_variants)
            .collect()
    }

    fn non_image_assets(&self) -> Vec<&dyn Asset> {
        let css_assets = self.css_assets().into_iter().map(|css_asset| {
            let css_asset: &dyn Asset = css_asset;
            css_asset
        });
        let js_assets = self.js_assets().into_iter().map(|js_asset| {
            let js_asset: &dyn Asset = js_asset;
            js_asset
        });
        let wasm_assets = self.wasm_assets().into_iter().map(|wasm_asset| {
            let wasm_asset: &dyn Asset = wasm_asset;
            wasm_asset
        });
        css_assets.chain(js_assets).chain(wasm_assets).collect()
    }
}

cfg_if! {
if #[cfg(feature = "build")] {
    use rayon::prelude::*;

    impl NonHtmlAssets {
        pub fn save_to_disk(&self) {
            self.all_assets_that_can_be_saved_to_disk()
                .into_iter()
                .par_bridge()
                .for_each(|asset| {
                    asset.save_to_disk();
                });
        }

        fn all_assets_that_can_be_saved_to_disk(&self) -> Vec<&dyn CanSaveToDisk> {
            let non_image_assets = self
                .non_image_assets_as_can_save_to_disk()
                .into_iter()
                .map(|asset| asset as &dyn CanSaveToDisk);
            let image_assets = self.image_assets().into_iter().map(|asset| asset as &dyn CanSaveToDisk);
            non_image_assets.chain(image_assets).collect()
        }

        fn all_image_assets(&self) -> Vec<&dyn CanSaveToDisk> {
            let images = non_html_assets.image_assets().into_iter().map(|image_asset| {
                let image_asset: &dyn CanSaveToDisk = image_asset;
                image_asset
            });

            let light_dark_images =
                non_html_assets
                    .light_dark_image_assets()
                    .into_iter()
                    .map(|light_dark_image_asset| {
                        let light_dark_image_asset: &dyn CanSaveToDisk = light_dark_image_asset;
                        light_dark_image_asset
                    });

            images.chain(light_dark_images).collect()
        }

        fn non_image_assets_as_can_save_to_disk(&self) -> Vec<&dyn CanSaveToDisk> {
            let css_assets = self.css_assets().into_iter().map(|asset| asset as &dyn CanSaveToDisk);
            let js_assets = self.js_assets().into_iter().map(|asset| asset as &dyn CanSaveToDisk);
            let wasm_assets = self.wasm_assets().into_iter().map(|asset| asset as &dyn CanSaveToDisk);
            css_assets.chain(js_assets).chain(wasm_assets).collect()
        }
    }
}
}

impl Default for NonHtmlAssets {
    fn default() -> Self {
        Self::new()
    }
}
