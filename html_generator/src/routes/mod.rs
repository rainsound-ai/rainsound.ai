use std::{path::PathBuf, str::FromStr};

use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_ssr::incremental::{DefaultRenderer, IncrementalRendererConfig};

pub async fn get_pages() -> Vec<HtmlAsset> {
    // create a VirtualDom with the app component
    // let mut app = VirtualDom::new(App);
    // rebuild the VirtualDom before rendering
    // let _ = app.rebuild();
    // render the VirtualDom to HTML
    // dioxus_ssr::render(&app)
    let temporary_asset_directory = manifest::dir().join("target").join("temp");

    let mut renderer = IncrementalRendererConfig::new()
        .static_dir(&temporary_asset_directory)
        .build();

    pre_cache_static_routes::<Route, _>(
        &mut renderer,
        &DefaultRenderer {
            before_body: r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width,
                initial-scale=1.0">
                <title>Dioxus Application</title>
            </head>
            <body>"#
                .to_string(),
            after_body: r#"</body>
            </html>"#
                .to_string(),
        },
    )
    .await
    .unwrap();

    // relative paths with out the file name
    let paths = Route::SITE_MAP
        .iter()
        .flat_map(|route| route.flatten().into_iter())
        .filter_map(|route| {
            let segments = &route
                .iter()
                .map(|segment| segment.to_string())
                .collect::<Vec<_>>()
                .join("")[1..];

            if segments == ":...segments" {
                return None;
            }

            Some(PathBuf::from_str(segments).unwrap())
        })
        .collect::<Vec<_>>();

    dbg!(&paths);

    let html_assets = paths
        .into_iter()
        .map(|cleaned_path| {
            let temp_path = temporary_asset_directory
                .join(&cleaned_path)
                .join("index.html");

            dbg!(&temp_path);

            let contents = fs::read_to_string(temp_path).unwrap();
            let path = cleaned_path.join("index.html");

            HtmlAsset {
                path,
                contents,
                size_budget: NumBytes(1),
            }
        })
        .collect::<Vec<_>>();

    dbg!(&html_assets);

    html_assets
}

// ANCHOR: router
#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq, Debug)]
enum Route {
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

// #[derive(Props)]
// struct LayoutProps<'a> {
//     title: &'static str,
//     children: Element<'a>,
// }

// fn Layout(title: &'static str, body: Element) -> Element {
#[inline_props]
fn Layout(cx: Scope) -> Element {
    render! {
        head {
            meta { charset: "UTF-8" }
            meta { content: "width=device-width, initial-scale=1.0", name: "viewport" }
            meta { http_equiv: "X-UA-Compatible", content: "ie=edge" }
            link { rel: "stylesheet", href: "main.css" }
        }

        body { class: "{bg_background()} dark:text-white flex flex-col items-center selection:bg-neutral-200/75 dark:selection:bg-neutral-700/75",
            nav { class: "flex gap-2 p-2 items-center justify-center w-full",
                Link { to: Route::Home {}, "Home" }
                Link { to: Route::Contact {}, "Contact" }
            }
            main { Outlet::<Route> {} }
            MainJs {}
        }
    }
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    dbg!("Body");
    render! {
        h1 { "Home" }
        p { "Hello, squirrel!" }
    }
}

#[inline_props]
fn Contact(cx: Scope) -> Element {
    render! {
        h1 { "Contact" }
        p { "This is the contact page." }
    }
}

#[inline_props]
fn NotFound(cx: Scope, segments: Vec<String>) -> Element {
    render! {
        h1 { "404" }
        p { "Page not found." }
    }
}

fn MainJs(cx: Scope) -> Element {
    dbg!("MainJs");
    let contents = include_str!("../main.js");
    render! { script { "type": "module", dangerous_inner_html: "{contents}" } }
}

fn horizontal_center_fixed() -> &'static str {
    "left-1/2 transform -translate-x-1/2"
}

fn bg_background() -> &'static str {
    "bg-neutral-50 dark:bg-neutral-900"
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
