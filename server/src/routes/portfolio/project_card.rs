use super::projects::*;
use crate::components::*;
use crate::side::*;
use maud::{html, Markup, Render};
use shared::Route;

pub struct ProjectCard {
    index: usize,
    project: Project,
    side: Side,
}

impl Render for ProjectCard {
    fn render(&self) -> Markup {
        let side = self.side;
        let background_color = self.background_color();
        let index = self.index;
        let text = self.project.text;

        let number = format!("0{}", index + 1);

        let (side_before, side_after) = match side {
            Left => (
                "before:left-0 before:bottom-full before:rounded-br-tooth before:shadow-[0_calc(10_*_100vw_/_180)_0_0_#283036]",
                "after:right-full after:bottom-0 after:rounded-br-tooth after:shadow-[0_calc(10_*_100vw_/_180)_0_0_#283036]",
            ),
            Right => (
                "before:left-full before:top-0 before:rounded-tl-tooth before:shadow-[0_calc(-10_*_100vw_/_180)_0_0_#283036]",
                "after:left-0 after:top-full after:rounded-tl-tooth after:shadow-[0_calc(-10_*_100vw_/_180)_0_0_#283036]",
            ),
        };
        let corner_before = "before:content-[''] before:absolute before:w-full before:h-full";
        let corner_after = "after:content-[''] after:absolute after:w-full after:h-full";
        let before_and_after = format!("{side_before} {side_after} {corner_before} {corner_after}");

        let cutout_side = match side {
            Left => "left bottom-0 right-0",
            Right => "right top-0 left-0",
        };

        let cutout_class =
            format!("{before_and_after} {cutout_side} bg-slate w-grid-32 h-grid-32 absolute z-10",);

        let maybe_text_right = match side {
            Left => "",
            Right => "text-right",
        };

        html! {
            div
                class={
                    "relative h-grid-69 w-grid-105 px-grid-5 py-grid-6 flex flex-col odd:self-start even:items-end even:self-end rounded-tooth z-0 "
                    (background_color)
                }
            {
                div
                    class={
                        "text-grid-8 underline font-light "
                        (maybe_text_right)
                    }
                {
                    (number)
                }

                h2 class={ "grow w-grid-69 " (maybe_text_right) } {
                    ( text )
                }

                div
                    class={
                        "flex justify-between z-20 "
                        (match side {
                            Left => "w-grid-64",
                            Right => "w-full",
                        })
                    }
                {
                    (Link::underline(self.project.link)
                        .slot("Learn More")
                    )
                    (Link::button(Route::Contact)
                        .slot(html! {
                            span class="inline-block translate-y-[0.2vw]" {
                                "Build This"
                            }
                        })
                    )
                }

                div class=(cutout_class) {}

            }
        }
    }
}

impl ProjectCard {
    pub fn new(index: usize, side: Side, project: Project) -> Self {
        Self {
            index,
            side,
            project,
        }
    }

    fn background_color(&self) -> &'static str {
        let mut color_classes = [
            "bg-petal-lavender",
            "bg-petal-salmon",
            "bg-petal-orange",
            "bg-petal-yellow",
            "bg-petal-blue",
            "bg-petal-purple",
        ]
        .into_iter()
        .cycle();

        color_classes.nth(self.index).unwrap()
    }
}
