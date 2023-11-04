use crate::components::*;
use assets::non_html_assets;
use maud::{html, Markup};

mod contact;
pub use self::contact::*;

mod not_found;
pub use self::not_found::*;

mod router;
pub use router::*;

pub fn home_page() -> Markup {
    layout(html! {
        h1 { "Home" }
        p class="bg-green-500" { "WE MAEK THE SOFTWRE FOR YOU GIVE US MONEY NOM NOM NOM NOM 🧌" }
        (image("", &non_html_assets.hasui_hero))
        // LightDarkImage { asset: &non_html_assets.images.hasui_hero }
    })
}
