use dioxus::prelude::*;

#[inline_props]
pub fn Contact(cx: Scope) -> Element {
    render! {
        h1 { "Contact" }
        p { "This is the contact page." }
    }
}
