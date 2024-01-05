use crate::side::*;
use maud::{html, Markup, Render};

pub struct Tooth {
    pub side: Side,
    pub last: bool,
}

impl Tooth {
    pub fn new(side: Side) -> Self {
        Self { side, last: false }
    }

    pub fn last(mut self, last: bool) -> Self {
        self.last = last;
        self
    }
}

impl Render for Tooth {
    fn render(&self) -> Markup {
        // CSS for the figure element.
        let side = match self.side {
            Left => "right-0", // These are intentionally flipped.
            Right => "left-0",
        };
        let last = if self.last {
            "!w-grid-78 !h-grid-52"
        } else {
            ""
        };

        // CSS for the figure element's before and after pseudo-classes.
        let corner_before = "before:content-[''] before:absolute before:h-grid-20 before:w-grid-20";
        let corner_after = "after:content-[''] after:absolute after:h-grid-20 after:w-grid-20";

        let before = "before:-top-grid-20 before:shadow-[0_calc(10_*_100vw_/_180)_0_0_#283036]";
        let after = "after:-bottom-grid-20 after:shadow-[0_calc(-10_*_100vw_/_180)_0_0_#283036]";

        let (side_before, side_after) = match self.side {
            Left => (
                "before:right-0 before:rounded-br-tooth",
                "after:right-0 after:rounded-tr-tooth",
            ),
            Right => (
                "before:left-0 before:rounded-bl-tooth",
                "after:left-0 after:rounded-tl-tooth",
            ),
        };

        let figure_class = format!(
            "{corner_before} {corner_after} {before} {after} {side_before} {side_after} {side} {last}"
        );

        html! {
            figure
                class={"
                    absolute top-0
                    w-grid-96
                    h-grid-64
                    grid grid-cols-3 grid-rows-2 
                    z-10 "
                    (figure_class)
                }
            {
                (match self.side {
                    Left => left_children(self.last),
                    Right => right_children(self.last),
                })
            }
        }
    }
}

fn left_children(last: bool) -> Markup {
    let tooth_block_classes = tooth_block_classes(last);

    html! {
        div class={ (tooth_block_classes) "!ring-0 left-children" } {}
        div class={ (tooth_block_classes) "bg-slate rounded-tl-tooth" } {}
        div class={ (tooth_block_classes) "bg-slate" } {}

        div class={ (tooth_block_classes) "bg-slate rounded-l-tooth" } {}
        div class={ (tooth_block_classes) "bg-slate" } {}
        div class={ (tooth_block_classes) "bg-slate" } {}
    }
}

fn right_children(last: bool) -> Markup {
    let tooth_block_classes = tooth_block_classes(last);

    html! {
        div class={ (tooth_block_classes) " bg-slate right-children"  } {}
        div class={ (tooth_block_classes) " bg-slate" } {}
        div class={ (tooth_block_classes) " bg-slate rounded-r-tooth" } {}

        div class={ (tooth_block_classes) " bg-slate" } {}
        div class={ (tooth_block_classes) " bg-slate rounded-br-tooth" } {}
        div class={ (tooth_block_classes) " !ring-0" } {}
    }
}

fn tooth_block_classes(last: bool) -> String {
    let last_classes = if last { "!w-grid-26 !h-grid-26" } else { "" };
    format!("w-grid-32 h-grid-32 ring-1 ring-slate {last_classes}")
}
