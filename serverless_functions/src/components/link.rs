use crate::routes::Route;
use maud::{html, Markup, Render};

pub struct Link<'a> {
    pub route: Route,
    pub slot: Markup,
    pub class: &'a str,
    pub variant: LinkVariant,
}

impl<'a> Link<'a> {
    pub fn new(route: Route) -> Self {
        Self {
            route,
            slot: html! { "" },
            class: "",
            variant: LinkVariant::Underline,
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

    pub fn variant(mut self, variant: LinkVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Render for Link<'_> {
    fn render(&self) -> Markup {
        let class = match self.variant {
            LinkVariant::NoUnderline => format!("{} no-underline", self.class),
            LinkVariant::Underline => format!("{} underline", self.class),
            LinkVariant::Box => format!("{} border px-2 align-middle", self.class),
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
    Box,
}
