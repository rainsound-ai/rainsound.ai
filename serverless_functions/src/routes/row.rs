use maud::{html, Markup, Render};

use super::tooth::*;

pub struct Row {
    pub side: RowSide,
    pub last: bool,
    pub slot: Markup,
}

impl Row {
    pub fn left() -> Self {
        Self {
            side: RowSide::Left,
            slot: html! { "" },
            last: false,
        }
    }

    pub fn right() -> Self {
        Self {
            side: RowSide::Right,
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
            RowSide::Left => "justify-start",
            RowSide::Right => "justify-end",
        };

        let section_classes = format!(
            "relative flex flex-row px-grid-6 w-full {last_section_classes} {side_classes}"
        );

        html! {
            section
                class=(section_classes)
            {
                (Tooth::new(self.side.clone()).last(self.last))

                div
                    class={
                        "flex flex-col w-grid-93 "
                        (match self.side {
                            RowSide::Left => "",
                            RowSide::Right => "items-end",
                        })
                    }
                {
                    (self.slot)
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum RowSide {
    Right,
    Left,
}
