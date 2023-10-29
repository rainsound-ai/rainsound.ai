use crate::components::*;
use maud::{html, Markup};
use new_assets::non_html_assets;

mod contact;
pub use self::contact::*;

mod not_found;
pub use self::not_found::*;

mod router;
pub use router::*;

pub fn home_page() -> Markup {
    dbg!("Body");
    layout(html! {
        h1 { "Home" }
        p { "WE MAEK THE SOFTWRE FOR YOU GIVE US MONEY NOM NOM NOM NOM 🧌" }
        (image("w-64 h-64", &non_html_assets.hasui_hero))
        // LightDarkImage { asset: &non_html_assets.images.hasui_hero }
    })
}
