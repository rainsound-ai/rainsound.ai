use maud::{html, Markup, Render};

pub struct Flower<'a> {
    pub class: &'a str,
}

impl<'a> Flower<'a> {
    pub fn new() -> Self {
        Self { class: "" }
    }

    #[allow(dead_code)]
    pub fn class(mut self, class: impl Into<&'a str>) -> Self {
        self.class = class.into();
        self
    }
}

impl Render for Flower<'_> {
    fn render(&self) -> Markup {
        let petal_classes = "petal absolute rounded-[50%] origin-[50%_0]";

        html! {
            div class={ "flower flower-container relative " (self.class) } {
                div class={"bg-petal-purple " (petal_classes)} style="width: calc(11.2361vw); height: calc(3.86111vw); left: calc(-6.42206vw); top: calc(6.16159vw); transform: rotate(458.005deg);" {}
                div class={"bg-petal-lavender " (petal_classes)} style="width: calc(11.2361vw); height: calc(3.86111vw); left: calc(-11.3249vw); top: calc(2.33038vw); transform: rotate(518.005deg);" {}
                div class={"bg-petal-salmon " (petal_classes)} style="width: calc(11.2361vw); height: calc(3.86111vw); left: calc(-10.4584vw); top: calc(-3.83121vw); transform: rotate(578.005deg);" {}
                div class={"bg-petal-orange " (petal_classes)} style="width: calc(11.2361vw); height: calc(3.86111vw); left: calc(-4.68905vw); top: calc(-6.16159vw); transform: rotate(638.005deg);" {}
                div class={"bg-petal-yellow " (petal_classes)} style="width: calc(11.2361vw); height: calc(3.86111vw); left: calc(0.213792vw); top: calc(-2.33038vw); transform: rotate(698.005deg);" {}
                div class={"bg-petal-blue " (petal_classes)} style="width: calc(11.2361vw); height: calc(3.86111vw); left: calc(-0.652712vw); top: calc(3.83121vw); transform: rotate(398.005deg);" {}
                // button
                //     class="click-target absolute w-grid-64 h-grid-64 -left-grid-32 -top-grid-32"
                //     style="-webkit-tap-highlight-color: transparent;"
                // {}
            }
        }
    }
}

// <style>
// 	button {
// 		-webkit-tap-highlight-color: transparent; /* for removing the highlight */
// 	}

// 	.flower-container {
// 		@apply relative;
// 	}

// 	.petal {
// 		@apply absolute rounded-[50%];
// 		transform-origin: 50% 0;
// 	}
// </style>
