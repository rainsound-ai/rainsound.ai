use crate::routes::Route;
use maud::{html, Markup};

pub fn link<'class>(class: impl Into<&'class str>, route: Route, body: Markup) -> Markup {
    let class = class.into();
    html! {
        a href=(route.to_string()) class=(class) {
            (body)
        }
    }
}
