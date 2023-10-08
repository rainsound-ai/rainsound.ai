use dioxus::prelude::*;

#[inline_props]
pub fn NotFound(cx: Scope, segments: Vec<String>) -> Element {
    render! {
        h1 { "404" }
        p { "Page not found." }
    }
}
