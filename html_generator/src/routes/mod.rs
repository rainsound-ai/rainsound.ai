use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod layout;
use layout::*;

mod contact;
use contact::*;

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
        p { "Hello, squirrel!" }
    }
}

fn horizontal_center_fixed() -> &'static str {
    "left-1/2 transform -translate-x-1/2"
}

// #[inline_props]
// fn Image<'a>(cx: Scope, asset: &'a ImageAsset, class: &'static str) -> Element<'a> {
//     dbg!("Image");
//     cx.render(rsx!(
//         div {
//             //
//             class: "select-none relative {class}",

//             img {
//                 alt: asset.alt,
//                 class: "shrink-0 min-w-full min-h-full object-cover",
//                 style: "image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;",
//                 src: "{asset.lqip}"
//             }

//             img {
//                 //
//                 alt: asset.alt,
//                 class: "absolute top-0 left-0 min-w-full min-h-full object-cover",
//                 src: asset.src(),
//                 srcset: asset.srcset()
//             }
//         }
//     ))
// }

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

#[derive(Props)]
struct LightDarkImageProps<'a> {
    asset: &'a LightDarkImageAsset,
    class: String,
    #[props(default = false)]
    above_the_fold: bool,
    #[props(default = false)]
    is_largest_contentful_paint: bool,
}

fn LightDarkImage<'a>(cx: Scope<'a, LightDarkImageProps<'a>>) -> Element<'a> {
    let class = &cx.props.class;
    let asset = cx.props.asset;
    let above_the_fold = cx.props.above_the_fold;
    let is_largest_contentful_paint = cx.props.is_largest_contentful_paint;

    // style: "image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;",
    cx.render(rsx!(
        div {
            //
            class: "select-none relative {class}",

            picture { class: "shrink-0 min-w-full min-h-full object-cover blur-lg",

                source {
                    //
                    "media": "(prefers-color-scheme: light)",
                    "srcset": "{asset.light_mode.lqip}",
                    "type": "image/jpeg"
                }

                source {
                    //
                    "media": "(prefers-color-scheme: dark)",
                    "srcset": "{asset.dark_mode.lqip}",
                    "type": "image/jpeg"
                }

                img {
                    //
                    alt: asset.alt,
                    class: "shrink-0 min-w-full min-h-full object-cover",
                    src: asset.light_mode.src()
                }
            }

            picture {
                //
                class: "absolute top-0 left-0 min-w-full min-h-full object-cover",

                source {
                    //
                    "media": "(prefers-color-scheme: light)",
                    "srcset": asset.light_mode.srcset(),
                    "type": asset.light_mode.mime_type()
                }

                source {
                    //
                    "media": "(prefers-color-scheme: dark)",
                    "srcset": asset.dark_mode.srcset(),
                    "type": asset.dark_mode.mime_type()
                }

                img {
                    //
                    "loading": if above_the_fold { "eager" } else { "lazy" },
                    "fetchpriority": if is_largest_contentful_paint { "high" } else { "auto" },
                    alt: asset.alt,
                    class: "absolute top-0 left-0 min-w-full min-h-full object-cover",
                    src: asset.light_mode.src()
                }
            }
        }
    ))
}
