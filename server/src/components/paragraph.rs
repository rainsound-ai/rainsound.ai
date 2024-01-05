use crate::routes::Route;
use maud::{html, Markup, Render};

pub struct Paragraph<'a> {
    pub class: &'a str,
    pub slot: Markup,
}

impl<'a> Paragraph<'a> {
    pub fn new() -> Self {
        Self {
            class: "",
            slot: html! { "" },
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

impl Render for Paragraph<'_> {
    fn render(&self) -> Markup {
        let class = format!("{} mb-grid-2", self.class);

        html! {
            p class=(class) {
                (self.slot)
            }
        }
    }
}
