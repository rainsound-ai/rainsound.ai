use assets::CssAsset;
use once_cell::sync::Lazy;

pub static all_assets: Lazy<ServerlessFunctionsAssets> = Lazy::new(ServerlessFunctionsAssets::new);

pub struct ServerlessFunctionsAssets {
    pub css: CssAsset,
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

        // let css = build_tailwind!(
        //     path_to_input_file: "serverless_functions/src/main.css",
        //     minify: true,
        //     debug: true
        // );
        // let css = CssAsset::new(
        //     PathBuf::from_str("built.css").unwrap(),
        //     built_css,
        //     Duration::from_millis(1),
        // );

        // let browser_crate = build_browser_crate!(
        //     path_to_browser_crate: "browser",
        //     production: true,
        //     debug: true,
        // );
        // let browser_js = JsAsset::new(
        //     PathBuf::from_str("browser.js").unwrap(),
        //     browser_crate.built_js,
        //     Duration::from_millis(1),
        // );
        // let browser_bg_wasm = WasmAsset::new(
        //     PathBuf::from_str("browser_bg.wasm").unwrap(),
        //     browser_crate.built_wasm,
        //     Duration::from_millis(1),
        // );

        // let built_images = build_images!(path_to_images_dir: "assets/runtime/src/original_images");
        // let hasui_hero = ImageAsset::from_built_image(
        //     "A woodblock print by Kawase Hasui",
        //     Placeholder::Lqip,
        //     built_images.hasui_light,
        // );

        ServerlessFunctionsAssets { css }
    }
}

impl Default for ServerlessFunctionsAssets {
    fn default() -> Self {
        Self::new()
    }
}
