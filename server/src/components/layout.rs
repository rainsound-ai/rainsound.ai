use crate::assets::ASSETS;
use crate::components::*;
use crate::extensions::*;
use crate::routes::Route;
use chrono::{Datelike, Utc};
use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn layout(title: &'static str, content: Markup) -> Markup {
    let current_year = Utc::now().year();

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                link rel="icon" href=(ASSETS.favicon.url_path.to_string_lossy().to_string());
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http_equiv="X-UA-Compatible" content="ie=edge";
                meta name="description" content="rainsound.ai: Custom AI & Web App Development";
                (stylesheet(&ASSETS.css))
                title {
                    (title)
                }
            }

            body class="bg-slate text-white min-h-screen flex flex-col font-aurora-grotesk text-grid-4" {
                header class="h-grid-10 px-grid-7 flex justify-between items-center text-neutral bg-slate whitespace-nowrap z-20 text-grid-3" {
                    (Link::no_underline(Route::Home)
                        .class("flex items-center gap-grid-1 no-underline")
                        .slot(html! {
                            img src=(ASSETS.logo.url_path.to_string()) alt="rainsound.ai logo" class="h-grid-4 w-grid-4" {
                                "rainsound.ai"
                            }
                        })
                    )

                    div class="flex gap-grid-4" {
                        // (Link::no_underline(Route::Paurtfaurliaur)
                        //     .class("text-slate")
                        //     .slot("Paurtfaurliaur")
                        // )
                        (Link::no_underline(Route::Portfolio)
                            .slot("Portfolio")
                        )
                        (Link::no_underline(Route::Contact)
                            .slot("Contact")
                        )
                    }
                }

                main class="px-grid-7 grow flex flex-col items-center overflow-hidden" {
                    (content)
                }

                footer
                    class="h-grid-33 px-grid-10 w-full grid grid-cols-3 text-grid-2 items-center text-neutral whitespace-nowrap"
                {
                    a href="/" class="flex items-center gap-grid-1 text-grid-3" {
                        img src=(ASSETS.logo.url_path.to_string()) alt="rainsound.ai logo" class="h-grid-5 w-grid-5" {
                            "rainsound.ai"
                        }
                    }

                    div class="flex gap-grid-4 justify-between" {

                        (Link::no_underline(Route::Home)
                            .slot("Home")
                        )
                        (Link::no_underline(Route::Portfolio)
                            .slot("Portfolio")
                        )
                        (Link::no_underline(Route::Contact)
                            .slot("Contact")
                        )
                    }

                    div class="justify-self-end" {
                        "© rainsound.ai " (current_year) ". We love our clients!"
                    }
                }

                (main_js())
            }
        }
    }
}

fn main_js() -> Markup {
    let browser_js_path = ASSETS
        .browser_crate
        .js
        .url_path
        .to_string_lossy()
        .to_string();
    let contents =
        include_str!("../assets/main.js").replace("{browser_js_filename}", &browser_js_path);
    html! {
        script type="module" {
            (PreEscaped(contents))
        }
    }
}
