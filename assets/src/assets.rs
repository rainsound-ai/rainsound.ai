use crate::non_image_asset::NonImageAsset;
use arraygen::Arraygen;

#[derive(Arraygen)]
#[gen_array(fn html_assets_with_performance_budget: &dyn NonImageAsset, implicit_select_all: HtmlAsset)]
pub struct Assets {
    // html_assets: Vec<HtmlAsset>,
}
