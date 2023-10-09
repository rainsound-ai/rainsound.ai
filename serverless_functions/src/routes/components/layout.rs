use crate::assets::non_html_assets;
use crate::routes::*;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use new_assets::*;

pub fn layout(content: Markup) -> Markup {
    html! {
    (DOCTYPE)
    html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http_equiv="X-UA-Compatible" content="ie=edge";
                (stylesheet(&non_html_assets.built_css))
            }

            body class={(bg_background()) " dark:text-white flex flex-col items-center selection:bg-neutral-200/75 dark:selection:bg-neutral-700/75"} {
                nav class="flex gap-2 p-2 items-center justify-start w-full font-semibold text-lg" {
                    a href="/" class="flex gap-2 items-center" {
                        span class="text-4xl font-bold" { "🌸" }
                        span { "Hyperbloom.studio" }
                    }
                    a href="/" { "Home" }
                    a href="/contact" { "Contact" }
                }
            }

            main {
                (content)
            }

            (main_js())
        }
    }
}

// body { class: "{bg_background()} dark:text-white flex flex-col items-center selection:bg-neutral-200/75 dark:selection:bg-neutral-700/75",
//     nav { class: " flex gap-2 p-2 items-center justify-start w-full font-semibold text-lg",
//         Link { class: "flex gap-2 items-center", to: Route::Home {},
//             span { class: "text-4xl font-bold", "🌸" }
//             span { "Hyperbloom.studio" }
//         }
//         Link { to: Route::Home {}, "Home" }
//         Link { to: Route::Contact {}, "Contact" }
//     }
//     main { Outlet::<Route> {} }
//     MainJs {}
// }

fn main_js() -> Markup {
    dbg!("MainJs");
    let contents = include_str!("../../main.js").replace(
        "{browser_js_filename}",
        non_html_assets.browser_js.path().to_str().unwrap(),
    );
    html! {
        script type="module" {
            (PreEscaped(contents))
        }
    }
}
