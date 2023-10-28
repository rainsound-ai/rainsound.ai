use maud::{html, Markup};

mod components;
use components::*;

mod contact;
use contact::*;

mod css_class_groups;
use css_class_groups::*;

mod router;
pub use router::*;

mod not_found;
pub use not_found::*;

pub fn home_page() -> Markup {
    dbg!("Body");
    layout(html! {
        h1 { "Home" }
        p { "WE MAEK THE SOFTWRE FOR YOU GIVE US MONEY NOM NOM NOM NOM 🧌" }
        // LightDarkImage { asset: &non_html_assets.images.hasui_hero }
    })
}
