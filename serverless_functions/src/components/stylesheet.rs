use assets::{Asset, CssAsset};
use maud::{html, Markup};

pub fn stylesheet(asset: &CssAsset) -> Markup {
    html! {
        link rel="stylesheet" href={(asset.path().to_string_lossy())} type="text/css" media="screen";
    }
}
