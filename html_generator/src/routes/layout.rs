use super::Route;
use super::*;

#[inline_props]
pub fn Layout(cx: Scope) -> Element {
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

fn MainJs(cx: Scope) -> Element {
    dbg!("MainJs");
    let contents = include_str!("../main.js");
    render! { script { "type": "module", dangerous_inner_html: "{contents}" } }
}
