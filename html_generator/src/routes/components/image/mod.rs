use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn Image<'a>(cx: Scope, asset: &'a ImageAsset, class: &'static str) -> Element<'a> {
    dbg!("Image");
    render!(
        div {
            //
            class: "select-none relative {class}",

            img {
                alt: asset.alt,
                class: "shrink-0 min-w-full min-h-full object-cover",
                style: "image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;",
                src: "{asset.lqip}"
            }

            img {
                alt: asset.alt,
                class: "absolute top-0 left-0 min-w-full min-h-full object-cover",
                src: asset.src(),
                srcset: asset.srcset()
            }
        }
    )
}
