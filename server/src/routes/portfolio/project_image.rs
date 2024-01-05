use crate::assets::assets;
use crate::components::*;
use crate::side::*;
use assets::ImageAsset;
use maud::{html, Markup, Render};

pub struct ProjectImage {
    index: usize,
    side: Side,
}

impl Render for ProjectImage {
    fn render(&self) -> Markup {
        let right_or_left = match self.side {
            Left => "right-0",
            Right => "left-0",
        };
        let align_self = if self.index > 0 {
            "self-end"
        } else {
            "self-start"
        };

        let class = format!(
            "object-contain object-bottom !bg-slate z-0 w-grid-52 h-grid-52 {right_or_left} {align_self}",
        );

        html! {
            (Image::new(self.asset())
                .class(&*class)
            )
        }
    }
}

impl ProjectImage {
    pub fn new(index: usize, side: Side) -> Self {
        Self { index, side }
    }

    fn asset(&self) -> &ImageAsset {
        let mut image_assets = [
            &assets.project_flower_one,
            &assets.project_flower_two,
            &assets.project_flower_three,
            &assets.project_flower_four,
            &assets.project_flower_five,
            &assets.project_flower_six,
        ]
        .into_iter()
        .cycle();

        image_assets.nth(self.index).unwrap()
    }
}
