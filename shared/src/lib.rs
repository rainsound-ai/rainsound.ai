#![allow(non_snake_case, non_upper_case_globals)]

use maud::{html, Markup};

// pub mod components;
// pub mod extensions;
// pub mod prelude;

// pub use prelude::*;

pub static message: &str = "Hello, shared!";

pub fn shared_component() -> Markup {
    html! {
        p { "Hello, shared component!" }
    }
}

pub fn another_shared_component() -> Markup {
    html! {
        p { "Hello, another shared component!" }
    }
}
