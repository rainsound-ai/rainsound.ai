use crate::assets::assets;
use crate::components::*;
use maud::{html, Markup};

pub fn portfolio_page() -> Markup {
    layout(html! {
        h1 class="!font-fugi" { "Portfolio" }
        p class="bg-red-500" { "WE MAEK THE SOFTWRE FOR YOU GIVE US MONEY NOM NOM NOM NOM 🧌" }
        (Image::new(&assets.hasui_hero))
    })
}
