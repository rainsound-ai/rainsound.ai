use crate::assets::assets;
use crate::components::*;
use maud::{html, Markup};

mod build_time;
pub use self::build_time::*;

mod contact;
pub use self::contact::*;

mod not_found;
pub use self::not_found::*;

mod router;
pub use router::*;

pub fn home_page() -> Markup {
    layout(html! {
        h1 { "Home" }
        p class="bg-red-500" { "WE MAEK THE SOFTWRE FOR YOU GIVE US MONEY NOM NOM NOM NOM 🧌" }
        (Image::new(&assets.hasui_hero))
        // LightDarkImage { asset: &non_html_assets.images.hasui_hero }
    })
}
