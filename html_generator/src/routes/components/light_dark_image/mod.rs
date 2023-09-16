use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props)]
pub struct LightDarkImageProps<'a> {
    asset: &'a LightDarkImageAsset,
    class: String,
    #[props(default = false)]
    above_the_fold: bool,
    #[props(default = false)]
    is_largest_contentful_paint: bool,
}

pub fn LightDarkImage<'a>(cx: Scope<'a, LightDarkImageProps<'a>>) -> Element<'a> {
    let class = &cx.props.class;
    let asset = cx.props.asset;
    let above_the_fold = cx.props.above_the_fold;
    let is_largest_contentful_paint = cx.props.is_largest_contentful_paint;

    // style: "image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;",
    cx.render(rsx!(
        div {
            //
            class: "select-none relative {class}",

            picture { class: "shrink-0 min-w-full min-h-full object-cover blur-lg",

                source {
                    //
                    "media": "(prefers-color-scheme: light)",
                    "srcset": "{asset.light_mode.lqip}",
                    "type": "image/jpeg"
                }

                source {
                    //
                    "media": "(prefers-color-scheme: dark)",
                    "srcset": "{asset.dark_mode.lqip}",
                    "type": "image/jpeg"
                }

                img {
                    //
                    alt: asset.alt,
                    class: "shrink-0 min-w-full min-h-full object-cover",
                    src: asset.light_mode.src()
                }
            }

            picture {
                //
                class: "absolute top-0 left-0 min-w-full min-h-full object-cover",

                source {
                    //
                    "media": "(prefers-color-scheme: light)",
                    "srcset": asset.light_mode.srcset(),
                    "type": asset.light_mode.mime_type()
                }

                source {
                    //
                    "media": "(prefers-color-scheme: dark)",
                    "srcset": asset.dark_mode.srcset(),
                    "type": asset.dark_mode.mime_type()
                }

                img {
                    //
                    "loading": if above_the_fold { "eager" } else { "lazy" },
                    "fetchpriority": if is_largest_contentful_paint { "high" } else { "auto" },
                    alt: asset.alt,
                    class: "absolute top-0 left-0 min-w-full min-h-full object-cover",
                    src: asset.light_mode.src()
                }
            }
        }
    ))
}
