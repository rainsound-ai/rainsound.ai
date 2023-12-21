use assets::*;
use once_cell::sync::Lazy;

pub static assets: Lazy<Assets> = Lazy::new(Assets::new);

pub struct Assets {
    pub css: CssAsset,
    pub browser_crate: BrowserCrateAsset,
    pub hasui_hero: ImageAsset,
    pub fugi: FontAsset,
    pub favicon: FileAsset,
}

// We have to separate out the non-html assets because
// we want to reference them when generating html.
//
// If we didn't do this, we'd have a circular dependency.
// This causes problems. For example, it can lead to
// deadlocking if we're using a lazily initialized global variable.
impl Assets {
    pub fn new() -> Self {
        let css = assets::include_tailwind!(
            path_to_input_file: "serverless_functions/src/main.css",
            url_path: "built-assets/built.css",
            performance_budget_millis: 150,
        );

        let browser_crate = assets::include_browser_crate!(
            path_to_browser_crate: "browser",
            js_url_path: "built-assets/browser.js",
            js_performance_budget_millis: 150,
            wasm_url_path: "built-assets/browser_bg.wasm",
            wasm_performance_budget_millis: 150,
        );

        let hasui_hero = assets::include_image!(
            path_to_image: "serverless_functions/src/images/hasui_light.jpeg",
            alt: "A woodblock print of mountains and a river by Kawase Hasui.",
            placeholder: lqip,
        );

        let fugi = assets::include_font!(
            path_to_input_file: "serverless_functions/src/fonts/Fugi.ttf",
            url_path: "built-assets/fonts/Fugi.ttf",
            performance_budget_millis: 150,
        );

        // favicon

        let favicon = assets::include_file!(
            path_to_input_file: "serverless_functions/src/images/favicon.ico",
            url_path: "built-assets/favicon.ico",
            performance_budget_millis: 150,
        );

        Self {
            css,
            browser_crate,
            hasui_hero,
            fugi,
            favicon,
        }
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}
