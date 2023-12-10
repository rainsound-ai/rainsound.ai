use assets::{BrowserCrateAsset, CssAsset, ImageAsset};
use once_cell::sync::Lazy;

pub static all_assets: Lazy<ServerlessFunctionsAssets> = Lazy::new(ServerlessFunctionsAssets::new);

pub struct ServerlessFunctionsAssets {
    pub css: CssAsset,
    pub browser_crate: BrowserCrateAsset,
    pub hasui_hero: ImageAsset,
}

// We have to separate out the non-html assets because
// we want to reference them when generating html.
//
// If we didn't do this, we'd have a circular dependency.
// This causes problems. For example, it can lead to
// deadlocking if we're using a lazily initialized global variable.
impl ServerlessFunctionsAssets {
    pub fn new() -> Self {
        let css = assets::build_tailwind!(
            path_to_input_file: "serverless_functions/src/main.css",
            url_path: "built.css",
            performance_budget_millis: 1,
            minify: true,
            debug: true
        );

        let browser_crate = assets::build_browser_crate!(
            path_to_browser_crate: "browser",
            js_url_path: "browser.js",
            js_performance_budget_millis: 1,
            wasm_url_path: "browser_bg.wasm",
            wasm_performance_budget_millis: 1,
            production: true,
            debug: true,
        );

        let built_images =
            assets::build_images!(path_to_images_dir: "serverless_functions/src/images");

        ServerlessFunctionsAssets {
            css,
            browser_crate,
            hasui_hero: built_images.hasui_light,
        }
    }
}

impl Default for ServerlessFunctionsAssets {
    fn default() -> Self {
        Self::new()
    }
}
