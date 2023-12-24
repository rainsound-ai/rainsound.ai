use crate::routes::Route;
use maud::{html, Markup, Render};

pub struct Link<'a> {
    pub route: Route,
    pub slot: Markup,
    pub class: &'a str,
    pub variant: LinkVariant,
}

impl<'a> Link<'a> {
    pub fn underline(route: Route) -> Self {
        Self {
            route,
            slot: html! { "" },
            class: "",
            variant: LinkVariant::Underline,
        }
    }

    pub fn no_underline(route: Route) -> Self {
        Self {
            route,
            slot: html! { "" },
            class: "",
            variant: LinkVariant::NoUnderline,
        }
    }

    pub fn button(route: Route) -> Self {
        Self {
            route,
            slot: html! { "" },
            class: "",
            variant: LinkVariant::Button,
        }
    }

    pub fn slot(mut self, slot: impl Render) -> Self {
        self.slot = slot.render();
        self
    }

    pub fn class(mut self, class: impl Into<&'a str>) -> Self {
        self.class = class.into();
        self
    }
}

impl Render for Link<'_> {
    fn render(&self) -> Markup {
        let class = match self.variant {
            LinkVariant::NoUnderline => format!("{} no-underline", self.class),
            LinkVariant::Underline => format!("{} underline", self.class),
            LinkVariant::Button => format!("{} border px-2 align-middle", self.class),
        };

        html! {
            a href=(self.route.to_string()) class=(class) {
                (self.slot)
            }
        }
    }
}

pub enum LinkVariant {
    NoUnderline,
    Underline,
    Button, // In SvelteKit this was called box, but that's a reserved word in Rust.
}
