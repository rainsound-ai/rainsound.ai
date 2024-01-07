use super::tooth::*;
use crate::side::*;
use maud::{html, Markup, Render};

pub struct Row {
    pub side: Side,
    pub last: bool,
    pub slot: Markup,
}

impl Row {
    pub fn left() -> Self {
        Self {
            side: Left,
            slot: html! { "" },
            last: false,
        }
    }

    pub fn right() -> Self {
        Self {
            side: Right,
            slot: html! { "" },
            last: false,
        }
    }

    pub fn last(mut self, last: bool) -> Self {
        self.last = last;
        self
    }

    pub fn slot(mut self, slot: impl Render) -> Self {
        self.slot = slot.render();
        self
    }
}

impl Render for Row {
    fn render(&self) -> Markup {
        let last_section_classes = if self.last { "h-grid-52" } else { "h-grid-68" };

        let side_classes = match self.side {
            Left => "justify-start",
            Right => "justify-end",
        };

        let section_classes = format!(
            "relative flex flex-row px-grid-6 w-full {last_section_classes} {side_classes}"
        );

        html! {
            section
                class=(section_classes)
            {
                (Tooth::new(self.side).last(self.last))

                div
                    class={
                        "flex flex-col w-grid-93 z-20 "
                        (match self.side {
                            Left => "",
                            Right => "items-end",
                        })
                    }
                {
                    (self.slot)
                }
            }
        }
    }
}
