use crate::non_image_asset::NonImageAsset;
use arraygen::Arraygen;

// We have to separate out the non-html assets because
// we want to reference them when generating html.
//
// If we didn't do this, we'd have a circular dependency.
// This causes problems. For example, it can lead to
// deadlocking if we're using a lazily initialized global variable.
#[derive(PartialEq, Arraygen)]
#[gen_array(fn assets_with_performance_budget: &dyn NonImageAsset, implicit_select_all: CssAsset, JsAsset, WasmAsset, TextAsset)]
pub struct NonHtmlAssets {
    // pub main_css: CssAsset,
    // pub browser_js: JsAsset,
    // pub browser_bg_wasm: WasmAsset,
    // pub build_time: TextAsset,
    // pub images: ImageAssets,
}
