use maud::{html, Markup};
use new_assets::Asset;
use new_assets::CssAsset;

pub fn stylesheet(asset: &CssAsset) -> Markup {
    html! {
        link rel="stylesheet" href={(asset.path().to_string_lossy())} type="text/css" media="screen";
    }
}
