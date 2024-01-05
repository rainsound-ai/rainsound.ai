use super::*;
use maud::{html, Markup};

pub fn not_found_page() -> Markup {
    layout(
        "rainsound.ai: Page Not Found",
        html! {
            "Couldn't find that thing you were looking for."
        },
    )
}
