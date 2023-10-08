use maud::{html, Markup};
use strum_macros::EnumIter;

mod components;
use components::*;

pub mod contact;
use contact::*;

mod css_class_groups;
use css_class_groups::*;

// mod not_found;
// use not_found::*;

#[derive(EnumIter)]
pub enum RouteNames {
    HomePage,
    ContactPage,
    // SubmitContactForm,
}

impl RouteNames {
    pub fn route(&self) -> Route {
        match self {
            RouteNames::HomePage => Route::Page {
                path: "/".to_string(),
                html: home_page(),
            },

            RouteNames::ContactPage => Route::Page {
                path: "/contact".to_string(),
                html: contact_page(),
            },
            // Routes::SubmitContactForm => Route {
            //     verb: HttpVerb::Post,
            //     path: "/contact".to_string(),
            // },
        }
    }
}

pub enum HttpVerb {
    Get,
    Post,
    Put,
    Delete,
}

pub enum Route {
    Page { path: String, html: Markup },
}

pub fn home_page() -> Markup {
    dbg!("Body");
    layout(html! {
        h1 { "Home" }
        p { "Hello, squirreld!" }
        p { "WE MAEK THE SOFTWRE FOR YOU GIVE US MONEY NOM NOM NOM NOM 🧌" }
        // LightDarkImage { asset: &non_html_assets.images.hasui_hero }
    })
}
