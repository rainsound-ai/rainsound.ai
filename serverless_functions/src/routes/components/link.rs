use std::fmt::Display;

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

pub struct Link {
    pub class: &'static str,
    pub route: Route,
    pub body: Markup,
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        link(self.class, self.route.clone(), self.body.clone())
            .into_string()
            .fmt(f)
    }
}
