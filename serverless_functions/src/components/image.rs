use assets::{BuiltPlaceholder, ImageAsset};
use maud::{html, Markup};

pub fn image<'class>(class: impl Into<&'class str>, asset: &ImageAsset) -> Markup {
    let class = class.into();

    match &asset.placeholder {
        BuiltPlaceholder::Color { css_string } => {
            image_with_color_placeholder(class, asset, css_string)
        }
        BuiltPlaceholder::Lqip { data_uri } => image_with_lqip(class, asset, data_uri),
    }
}

fn image_with_color_placeholder(
    class: &str,
    asset: &ImageAsset,
    placeholder_color_css_string: &str,
) -> Markup {
    html!(img
        class={(class) " select-none"}
        style={"background-color: " (placeholder_color_css_string)}
        alt=(asset.alt)
        src=(asset.src)
        srcset=(asset.srcset);
    )
}

fn image_with_lqip(class: &str, asset: &ImageAsset, data_uri: &str) -> Markup {
    html!(
        div
            class={"select-none relative " (class)}
        {

            // LQIP.
            img
                alt=(asset.alt)
                class="shrink-0 min-w-full min-h-full object-cover"
                style="image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;"
                src=(data_uri);

            // Actual image.
            img
                alt=(asset.alt)
                class="absolute top-0 left-0 min-w-full min-h-full object-cover"
                src=(asset.src)
                srcset=(asset.srcset);
        }
    )
}
