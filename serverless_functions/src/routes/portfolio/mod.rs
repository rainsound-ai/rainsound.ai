use crate::components::*;
use crate::side::*;
use maud::{html, Markup, Render};

mod projects;
use projects::*;
use shared::Route;

pub fn portfolio_page() -> Markup {
    layout(html! {
        @for (index, project) in all_projects().into_iter().enumerate() {
            (ProjectCard::new(index, project))
        }
    })
}

struct ProjectCard {
    index: usize,
    project: Project,
}

impl ProjectCard {
    fn new(index: usize, project: Project) -> Self {
        Self { index, project }
    }

    fn side(&self) -> Side {
        if self.index % 2 == 0 { Left } else { Right }
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

impl Render for ProjectCard {
    fn render(&self) -> Markup {
        let side = self.side();
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
            section
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
                        .slot("Build This")
                    )
                }

                div class=(cutout_class) {}
            }
        }
    }
}

// <script lang="ts">
// 	import projects from './projects'
// </script>

// {#each projects as { color, text, link }, index}
// 	{@const side = index % 2 === 0 ? 'left' : 'right'}

// 	<section
// 		class="relative h-grid-69 w-grid-105 px-grid-5 py-grid-6 flex flex-col odd:self-start even:items-end even:self-end rounded-tooth {color}"
// 	>
// 		<div class:text-right={side === 'right'} class="text-grid-8 underline font-light">
// 			0{index + 1}
// 		</div>

// 		<h2 class:text-right={side === 'right'} class="grow w-grid-69">{text}</h2>

// 		<div
// 			class="flex justify-between
// 			{side === 'left' ? 'w-grid-64' : 'w-full'}"
// 		>
// 			<a href={link}>Learn More</a>
// 			<a href="/contact" class="box">Build This</a>
// 		</div>

// 		<div class="cutout {side === 'left' ? 'left bottom-0 right-0' : 'right top-0 left-0'}" />
// 	</section>
// {/each}

// <style lang="postcss">
// 	✅ .cutout {
// 		@apply bg-slate w-grid-32 h-grid-32 absolute;
// 	}

// 	✅.corner {
// 		content: '';
// 		@apply absolute w-full h-full;
// 	}

// ✅.cutout:before {
//   @apply corner;
// }

// ✅.cutout:after {
//   @apply corner;
// }

// if left
// before: before:left-0 before:bottom-full before:rounded-br-tooth before:shadow-[0_calc(10_*_100vw_/_180)_0_0_#283036]
// after: after:right-full after:bottom-0 after:rounded-br-tooth after:shadow-[0_calc(10_*_100vw_/_180)_0_0_#283036]

// if right
// before: before:left-full before:top-0 before:rounded-tl-tooth before:shadow-[0_calc(-10_*_100vw_/_180)_0_0_#283036]
// after: after:left-0 after:top-full after:rounded-tl-tooth after:shadow-[0_calc(-10_*_100vw_/_180)_0_0_#283036]

// 	✅ .cutout.left:before {
// 		@apply left-0 bottom-full rounded-br-tooth;
// 		box-shadow: 0 calc(10 * 100vw / 180) 0 0 #283036;
// 	}

// 	✅ .cutout.left:after {
// 		@apply right-full bottom-0 rounded-br-tooth;
// 		box-shadow: 0 calc(10 * 100vw / 180) 0 0 #283036;
// 	}

// 	✅ .cutout.right:before {
// 		@apply left-full top-0 rounded-tl-tooth;
// 		box-shadow: 0 calc(-10 * 100vw / 180) 0 0 #283036;
// 	}

// 	✅ .cutout.right:after {
// 		@apply left-0 top-full rounded-tl-tooth;
// 		box-shadow: 0 calc(-10 * 100vw / 180) 0 0 #283036;
// 	}
// </style>
