use maud::{html, Markup, Render};

pub struct CtaButton<'a> {
    class: &'a str,
    slot: Markup,
}

impl<'a> CtaButton<'a> {
    pub fn new() -> Self {
        Self {
            class: "",
            slot: html! { "" },
        }
    }

    pub fn class(mut self, class: impl Into<&'a str>) -> Self {
        self.class = class.into();
        self
    }

    pub fn slot(mut self, slot: impl Render) -> Self {
        self.slot = slot.render();
        self
    }
}

impl Render for CtaButton<'_> {
    fn render(&self) -> Markup {
        html! {
            button
                class={
                    "cta bg-petal-salmon text-grid-4 px-grid-4 py-grid-2 rounded-sm border "
                    (self.class)
                }
            {
                (self.slot)
            }
        }
    }
}
