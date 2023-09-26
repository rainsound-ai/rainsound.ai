use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod components;
use components::*;

mod contact;
use contact::*;

mod css_class_groups;
use css_class_groups::*;

mod layout;
use layout::*;

mod not_found;
use not_found::*;

// ANCHOR: router
#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},  
        #[route("/contact")]
        Contact {},  
    #[end_layout]

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
// ANCHOR_END: router

#[inline_props]
fn Home(cx: Scope) -> Element {
    dbg!("Body");
    render! {
        h1 { "Home" }
        p { "Hello, squirreld!" }
        p { "WE MAEK THE SOFTWRE FOR YOU GIVE US MONEY NOM NOM NOM NOM 🧌" }
        LightDarkImage { asset: &non_html_assets.images.hasui_hero }
    }
}

// #[derive(Props)]
// struct RenderBrowserComponentProps<BrowserComponentProps>
// where
//     BrowserComponentProps: Serialize,
// {
//     class: Option<&'static str>,
//     component: BrowserComponent<BrowserComponentProps>,
//     children: Option<Element<'static>>,
// }

// fn RenderBrowserComponent<BrowserComponentProps>(
//     cx: Scope<'static, RenderBrowserComponentProps<BrowserComponentProps>>,
// ) -> Element<'static>
// where
//     BrowserComponentProps: Serialize,
// {
//     let component = &cx.props.component;
//     let serialized_props = serde_json::to_string(&component.props).unwrap();

//     cx.render(rsx!(
//         div {
//             class: cx.props.class,
//             "data-browser-component-name": component.name,
//             "data-browser-component-props": "{serialized_props}",
//             if let Some(children) = &cx.props.children {
//                 children
//             }
//         }
//     ))
// }
