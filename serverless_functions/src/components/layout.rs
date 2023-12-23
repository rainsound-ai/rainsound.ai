use crate::assets::assets;
use crate::components::*;
use crate::extensions::*;
use crate::routes::Route;
use chrono::{Datelike, Utc};
use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn layout(content: Markup) -> Markup {
    let current_year = Utc::now().year();

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                link rel="icon" href=(assets.favicon.url_path.to_string_lossy().to_string());
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http_equiv="X-UA-Compatible" content="ie=edge";
                meta name="description" content="rainsound.ai - An AI Technology Studio";
                (stylesheet(&assets.css))
            }

            body class="bg-slate text-white min-h-screen flex flex-col font-aurora-grotesk text-grid-4" {
                header class="h-grid-10 px-grid-7 flex justify-between items-center text-neutral bg-slate whitespace-nowrap z-20 text-grid-3" {
                    (Link::new(Route::Home)
                        .class("flex items-center gap-grid-1 no-underline")
                        .variant(LinkVariant::NoUnderline)
                        .slot(html! {
                            img src=(assets.logo.url_path.to_string()) alt="rainsound.ai logo" class="h-grid-4 w-grid-4" {
                                "rainsound.ai"
                            }
                        })
                    )

                    div class="flex gap-grid-4" {
                        (Link::new(Route::Paurtfaurliaur)
                            .class("text-slate")
                            .variant(LinkVariant::NoUnderline)
                            .slot("Paurtfaurliaur")
                        )
                        (Link::new(Route::Portfolio)
                            .variant(LinkVariant::NoUnderline)
                            .slot("Portfolio")
                        )
                        (Link::new(Route::Contact)
                            .variant(LinkVariant::NoUnderline)
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
                        img src=(assets.logo.url_path.to_string()) alt="rainsound.ai logo" class="h-grid-5 w-grid-5" {
                            "rainsound.ai"
                        }
                    }

                    div class="flex gap-grid-4 justify-between" {

                        (Link::new(Route::Home)
                            .variant(LinkVariant::NoUnderline)
                            .slot("Home")
                        )
                        (Link::new(Route::Portfolio)
                            .variant(LinkVariant::NoUnderline)
                            .slot("Portfolio")
                        )
                        (Link::new(Route::Contact)
                            .variant(LinkVariant::NoUnderline)
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
    let browser_js_path = assets
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
