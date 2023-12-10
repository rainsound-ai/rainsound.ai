use crate::assets::assets;
use crate::components::*;
use crate::css_class_groups::*;
use crate::routes::Route;
use assets::*;
use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn layout(content: Markup) -> Markup {
    html! {
    (DOCTYPE)
    html lang="en" {
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            meta http_equiv="X-UA-Compatible" content="ie=edge";
            (stylesheet(&assets.css))
        }

        body class={(bg_background()) " dark:text-white flex flex-col items-center selection:bg-neutral-200/75 dark:selection:bg-neutral-700/75"} {
            (nav_links())

            main {
                (content)
            }

            (main_js())
        }
    }
    }
}

fn nav_links() -> Markup {
    html! {
        nav class="flex gap-2 p-2 items-center justify-start w-full font-semibold text-lg" {
            (link(
                "flex gap-2 items-center",
                Route::Home,
                html! {
                    span class="text-4xl font-bold" { "🌸" }
                    span { "Hyperbloom.studio" }
                }
            ))

            (link(
                "",
                Route::Home,
                html! {
                    "Home"
                }
            ))

            (link(
                "",
                Route::Contact,
                html! {
                    "Contact"
                }
            ))
        }
    }
}

fn main_js() -> Markup {
    let browser_js_path = assets
        .browser_crate
        .js
        .full_url_path
        .to_string_lossy()
        .to_string();
    dbg!(&browser_js_path);
    let contents = include_str!("../main.js").replace("{browser_js_filename}", &browser_js_path);
    dbg!(&contents);
    html! {
        script type="module" {
            (PreEscaped(contents))
        }
    }
}
