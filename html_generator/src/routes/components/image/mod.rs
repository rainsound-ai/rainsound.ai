use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props)]
pub struct ImageProps<'a> {
    asset: &'a ImageAsset,
    #[props(default = "")]
    class: &'static str,
}

pub fn Image<'a>(cx: Scope<'a, ImageProps<'a>>) -> Element<'a> {
    let asset = cx.props.asset;
    let class = cx.props.class;
    dbg!("Image");

    match &asset.placeholder {
        GeneratedPlaceholder::Color { css_string } => render!(
            //
            ImageWithColorPlaceholder {
                asset: asset,
                class: class,
                placeholder_color_css_string: css_string
            }
        ),

        GeneratedPlaceholder::Lqip {
            data_uri,
            mime_type: _mime_type,
        } => {
            render!(ImageWithLqip {
                asset: asset,
                class: class,
                data_uri: data_uri
            })
        }
    }
}

#[inline_props]
pub fn ImageWithColorPlaceholder<'a>(
    cx: Scope,
    asset: &'a ImageAsset,
    class: &'static str,
    placeholder_color_css_string: &'a str,
) -> Element<'a> {
    render!(img {
        class: "select-none {class}",
        style: "background-color: {placeholder_color_css_string}",
        alt: asset.alt,
        src: asset.src(),
        srcset: asset.srcset()
    })
}

#[inline_props]
pub fn ImageWithLqip<'a>(
    cx: Scope,
    asset: &'a ImageAsset,
    class: &'static str,
    data_uri: &'a str,
) -> Element<'a> {
    render!(
        div {
            //
            class: "select-none relative {class}",

            img {
                alt: asset.alt,
                class: "shrink-0 min-w-full min-h-full object-cover",
                style: "image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;",
                src: "{data_uri}"
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
