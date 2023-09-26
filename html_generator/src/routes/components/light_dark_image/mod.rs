use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props)]
pub struct LightDarkImageProps<'a> {
    asset: &'a LightDarkImageAsset,
    #[props(default = String::from(""))]
    class: String,
    #[props(default = false)]
    above_the_fold: bool,
    #[props(default = false)]
    is_largest_contentful_paint: bool,
}

pub fn LightDarkImage<'a>(cx: Scope<'a, LightDarkImageProps<'a>>) -> Element<'a> {
    let props = cx.props;
    match &props.asset.placeholder {
        LightDarkPlaceholder::Color {
            light_mode_css_string,
            dark_mode_css_string,
        } => render!(LightDarkImageWithColorPlaceholder {
            asset: props.asset,
            class: "{props.class}",
            above_the_fold: props.above_the_fold,
            is_largest_contentful_paint: props.is_largest_contentful_paint,
            light_mode_css_string: light_mode_css_string,
            dark_mode_css_string: dark_mode_css_string
        }),

        LightDarkPlaceholder::Lqip {
            light_mode_data_uri,
            light_mode_mime_type,
            dark_mode_data_uri,
            dark_mode_mime_type,
        } => render!(LightDarkImageWithLqip {
            asset: props.asset,
            class: "{props.class}",
            above_the_fold: props.above_the_fold,
            is_largest_contentful_paint: props.is_largest_contentful_paint,
            light_mode_data_uri: light_mode_data_uri,
            light_mode_mime_type: light_mode_mime_type,
            dark_mode_data_uri: dark_mode_data_uri,
            dark_mode_mime_type: dark_mode_mime_type
        }),
    }
}

#[inline_props]
pub fn LightDarkImageWithLqip<'a>(
    cx: Scope<'a>,
    asset: &'a LightDarkImageAsset,
    class: &'a str,
    above_the_fold: bool,
    is_largest_contentful_paint: bool,
    light_mode_data_uri: &'a str,
    light_mode_mime_type: &'static str,
    dark_mode_data_uri: &'a str,
    dark_mode_mime_type: &'static str,
) -> Element<'a> {
    // style: "image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;",
    render!(
        div { class: "light-dark-image-with-lqip-placeholder select-none relative overflow-hidden {class}",

            picture { class: "shrink-0 min-w-full min-h-full object-cover blur-lg",

                source {
                    //
                    "media": "(prefers-color-scheme: light)",
                    "srcset": "{light_mode_data_uri}",
                    "type": "{light_mode_mime_type}"
                }

                source {
                    //
                    "media": "(prefers-color-scheme: dark)",
                    "srcset": "{dark_mode_data_uri}",
                    "type": "{dark_mode_mime_type}"
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
                    "loading": if *above_the_fold { "eager" } else { "lazy" },
                    "fetchpriority": if *is_largest_contentful_paint { "high" } else { "auto" },
                    alt: asset.alt,
                    class: "absolute top-0 left-0 min-w-full min-h-full object-cover",
                    src: asset.light_mode.src()
                }
            }
        }
    )
}

#[inline_props]
pub fn LightDarkImageWithColorPlaceholder<'a>(
    cx: Scope<'a>,
    asset: &'a LightDarkImageAsset,
    class: &'a str,
    above_the_fold: bool,
    is_largest_contentful_paint: bool,
    light_mode_css_string: &'a str,
    dark_mode_css_string: &'a str,
) -> Element<'a> {
    // style: "image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;",
    render!(
        div { class: "light-dark-image-with-color-placeholder select-none relative h-[500px] {class}",

            div {
                class: "absolute top-0 left-0 shrink-0 min-w-full min-h-full object-cover",
                background_color: "{dark_mode_css_string}"
            }

            div {
                class: "absolute top-0 left-0 shrink-0 min-w-full min-h-full object-cover dark:hidden",
                background_color: "{light_mode_css_string}"
            }

            picture {
                //
                class: "absolute top-0 left-0 min-w-full min-h-full object-cover"
            }
        }
    )
}

// source {
//     //
//     "media": "(prefers-color-scheme: light)",
//     "srcset": asset.light_mode.srcset(),
//     "type": asset.light_mode.mime_type()
// }

// source {
//     //
//     "media": "(prefers-color-scheme: dark)",
//     "srcset": asset.dark_mode.srcset(),
//     "type": asset.dark_mode.mime_type()
// }

// img {
//     //
//     "loading": if *above_the_fold { "eager" } else { "lazy" },
//     "fetchpriority": if *is_largest_contentful_paint { "high" } else { "auto" },
//     alt: asset.alt,
//     class: "absolute top-0 left-0 min-w-full min-h-full object-cover",
//     src: asset.light_mode.src()
// }
