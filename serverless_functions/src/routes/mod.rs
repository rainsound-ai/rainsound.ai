use crate::components::*;
use maud::{html, Markup};

mod build_time;
pub use self::build_time::*;
mod contact;
pub use self::contact::*;
mod not_found;
pub use self::not_found::*;
mod router;
pub use router::*;
mod portfolio;
pub use self::portfolio::*;
mod flower;
use self::flower::*;
mod row;
use self::row::*;
mod tooth;
use self::tooth::*;

pub fn home_page() -> Markup {
    layout(html! {
        (hero_section())
        (body())
    })
}

fn hero_section() -> Markup {
    html! {
        div class="hero h-grid-108 z-10 relative flex items-center justify-center w-full" {
            div class="flex flex-col w-grid-128" {
                h1 class="text-left font-fugi" {
                    "We're an AI"
                }
                h1 class="text-center font-fugi" {
                    "Technology Studio"
                }
                h4 class="text-right font-fugi" {
                    "We can help transform your vision into a beautifully crafted product"
                }
            }

            (Flower::new().class("!absolute top-grid-36 left-0 scale-[80%]"))
            (Flower::new().class("!absolute top-grid-24 right-grid-16 scale-[200%] rotate-180"))
            (Flower::new().class("!absolute bottom-grid-24 left-grid-24 scale-[130%] rotate-90"))
            (Flower::new().class("!absolute bottom-grid-20 right-grid-40 scale-[40%] rotate-45"))
        }
    }
}

fn body() -> Markup {
    let h2_class = "mb-grid-4";

    html! {
        div class="flex flex-col rounded-tooth w-full" style="background: linear-gradient(to bottom, #e189dc 0%, #e189ab 25%, #e1a889 50%, #e2d989 100%);" {
            (Row::left()
                .slot(html! {
                    h2 class={ (h2_class) " pt-grid-4" } {
                        "Who We Are"
                    }
                    (Paragraph::new()
                        .class("w-grid-62")
                        .slot("We're a technology studio with expertise in machine learning, AI engineering, and unreasonably effective tools like Rust.")
                    )
                    (Paragraph::new()
                        .class("w-grid-62")
                        .slot("We love to build beautiful products for industry leaders and early-to-mid stage startups.")
                    )
                })
            )

            (Row::right()
                .slot(html! {
                    h2 class=(h2_class) {
                        "How We Can Help"
                    }
                    (Paragraph::new()
                        .class("w-grid-52")
                        .slot("We can supercharge your business with cutting-edge AI solutions and application development.")
                    )
                    (Paragraph::new()
                        .slot("Our unique blend of technical excellence and emotional intelligence ensures smooth collaboration and sustainable results that prioritize human well-being.")
                    )
                })
            )

            (Row::left()
                .slot(html! {
                    h2 class=(h2_class) {
                        "Build Something With Us"
                    }
                    (Paragraph::new()
                        .class("w-grid-50")
                        .slot("Got an idea for an AI-related project? We offer complimentary solution architecture and design! Work with us to bring your vision to life.")
                    )
                    (Paragraph::new()
                        .class("w-grid-50")
                        .slot(html! {
                            "See some examples of our work "
                            (Link::underline(Route::Portfolio)
                                .slot("here")
                            )
                            "."
                        })
                    )
                })
            )

            (Row::right()
                .last(true)
                .slot(html! {
                    h2 class=(h2_class) {
                        "What's Your Vision?"
                    }

                    div class="inline-block w-grid-64 mt-grid-10 text-center float-right" {
                        (Link::no_underline(Route::Contact)
                            .slot(html! {
                                button class="cta" {
                                    "Tell us about it!"
                                }
                            })
                        )
                    }
                })
            )
        }
    }
}

// <div class="gradient flex flex-col rounded-tooth w-full">
// 	<Row side="left">
// 		<h2 class="pt-grid-4">Who We Are</h2>
// 		<p class="w-grid-62">
// 			We're a technology studio with expertise in machine learning, AI engineering, and unreasonably
// 			effective tools like Rust.
// 		</p>
// 		<p class="w-grid-62">
// 			We love to build beautiful products for industry leaders and early-to-mid stage startups.
// 		</p>
// 	</Row>

// 	<Row side="right">
// 		<h2>How We Can Help</h2>
// 		<p class="w-grid-52">
// 			We can supercharge your business with cutting-edge AI solutions and application development.
// 		</p>
// 		<p>
// 			Our unique blend of technical excellence and emotional intelligence ensures smooth
// 			collaboration and sustainable results that prioritize human well-being.
// 		</p>
// 	</Row>

// 	<Row side="left">
// 		<h2>Build Something With Us</h2>
// 		<p class="w-grid-50">
// 			Got an idea for an AI-related project? We offer complimentary solution architecture and
// 			design! Work with us to bring your vision to life.
// 		</p>

// 		<p class="w-grid-50">
// 			See some examples of our work
// 			<a href="/portfolio">here</a>.
// 		</p>
// 	</Row>

// 	<Row side="right" last>
// 		<h2>What's Your Vision?</h2>
// 		<div class="inline-block w-grid-64 mt-grid-10 text-center float-right">
// 			<a href="/contact">
// 				<button class="cta">Tell us about it!</button>
// 			</a>
// 		</div>
// 	</Row>
// </div>

// <style lang="postcss">
// 	h2 {
// 		@apply mb-grid-4;
// 	}

// 	.gradient {
// 		/* @apply bg-gradient-to-b from-petal-purple from-0% via-25% via-petal-salmon via-50% via-petal-orange to-100% to-petal-yellow; */
// 		background: linear-gradient(to bottom, #e189dc 0%, #e189ab 25%, #e1a889 50%, #e2d989 100%);
// 	}
// </style>
