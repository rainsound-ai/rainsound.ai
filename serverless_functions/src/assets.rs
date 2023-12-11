use assets::{BrowserCrateAsset, CssAsset, ImageAsset};
use once_cell::sync::Lazy;

pub static assets: Lazy<Assets> = Lazy::new(Assets::new);

pub struct Assets {
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
impl Assets {
    pub fn new() -> Self {
        let css = assets::build_tailwind!(
            path_to_input_file: "serverless_functions/src/main.css",
            url_path: "built.css",
            performance_budget_millis: 1,
            minify: true,
        );

        let browser_crate = assets::build_browser_crate!(
            path_to_browser_crate: "browser",
            js_url_path: "browser.js",
            js_performance_budget_millis: 1,
            wasm_url_path: "browser_bg.wasm",
            wasm_performance_budget_millis: 1,
            production: true,
        );

        // let built_images =
        //     assets::build_images!(path_to_images_dir: "serverless_functions/src/images");

        let hasui_hero = assets::build_image!(
            path_to_image: "serverless_functions/src/images/hasui_light.jpeg",
            alt: "Hasui's 'The Pond at Benten Shrine in Shiba'",
        );

        Self {
            css,
            browser_crate,
            hasui_hero,
        }
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}
