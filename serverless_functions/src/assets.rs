use assets::{BrowserCrateAsset, CssAsset, FontAsset, ImageAsset};
use once_cell::sync::Lazy;

pub static assets: Lazy<Assets> = Lazy::new(Assets::new);

pub struct Assets {
    pub css: CssAsset,
    pub browser_crate: BrowserCrateAsset,
    pub hasui_hero: ImageAsset,
    pub my_font: FontAsset,
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
            performance_budget_millis: 150,
        );

        let browser_crate = assets::build_browser_crate!(
            path_to_browser_crate: "browser",
            js_url_path: "browser.js",
            js_performance_budget_millis: 150,
            wasm_url_path: "browser_bg.wasm",
            wasm_performance_budget_millis: 150,
        );

        let hasui_hero = assets::build_image!(
            path_to_image: "serverless_functions/src/images/hasui_light.jpeg",
            alt: "A woodblock print of mountains and a river by Kawase Hasui.",
            placeholder: lqip,
        );

        // let my_font = assets::build_font!(
        //     path_to_input_file: "serverless_functions/src/fonts/MyFont.otf",
        //     url_path: "fonts/MyFont.otf",
        //     performance_budget_millis: 150,
        // );

        // favicon

        Self {
            css,
            browser_crate,
            hasui_hero,
            my_font,
        }
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}
