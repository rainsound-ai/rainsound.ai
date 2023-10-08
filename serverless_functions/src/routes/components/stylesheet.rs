use maud::{html, Markup};
use new_assets::CssAsset;
use new_assets::NonImageAsset;

pub fn stylesheet(asset: &CssAsset) -> Markup {
    html! {
        link rel="stylesheet" href={(asset.path().to_string_lossy())} type="text/css" media="screen";
    }
}
