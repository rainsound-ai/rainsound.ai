use crate::components::*;
use crate::side::*;
use maud::{html, Markup};

mod projects;
use projects::*;
mod project_card;
use self::project_card::*;
mod project_image;
use self::project_image::*;

pub fn portfolio_page() -> Markup {
    layout(html! {
        @for (index, project) in all_projects().into_iter().enumerate() {
            @let side = if index % 2 == 0 { Left } else { Right };

            @let project_card = ProjectCard::new(index, side, project);
            @let project_image = ProjectImage::new(index, side);

            section class="relative flex justify-between w-full" {
                @match side {
                    Left => {
                        (project_card)
                        (project_image)
                    }
                    Right => {
                        (project_image)
                        (project_card)
                    }
                }
            }
        }
    })
}
