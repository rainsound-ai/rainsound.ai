use super::*;
use maud::{html, Markup};

pub fn not_found_page() -> Markup {
    layout(html! {
        "Couldn't find that thing you were looking for."
    })
}
