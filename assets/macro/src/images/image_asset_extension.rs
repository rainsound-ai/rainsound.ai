use crate::images::*;
use assets_runtime::ImageAsset;

pub trait ImageAssetExtension {
    fn from_image_to_build(image_to_build: &ImageToBuild) -> Self;
}

impl ImageAssetExtension for ImageAsset {
    fn from_image_to_build(image_to_build: &ImageToBuild) -> Self {
        ImageAsset {
            alt: image_to_build.alt.clone(),
            placeholder: image_to_build.placeholder.clone(),
            width: image_to_build.width,
            height: image_to_build.height,
            srcset: generate_srcset(&image_to_build.resized_copies),
            src: generate_src(image_to_build),
        }
    }
}

fn generate_src(built_image: &ImageToBuild) -> String {
    // If their browser doesn't have support for the srcset attribute,
    // it's probably an old mobile browser. If that's the case, they
    // also probably don't have a lot of bandwidth so go with the smallest
    // image possible.
    let narrowest = built_image
        .resized_copies
        .iter()
        .min_by_key(|resized_copy| resized_copy.width)
        .expect("Received a built image with no resized copies.");

    assets_runtime::asset_url_path(&narrowest.path_starting_from_images_dir)
        .to_string_lossy()
        .to_string()
}

fn generate_srcset(resized_copies: &[BuildTimeResizedImage]) -> String {
    resized_copies
        .iter()
        .map(|resized_copy| {
            let width = resized_copy.width;
            let path = assets_runtime::asset_url_path(&resized_copy.path_starting_from_images_dir);
            let path_str = path.to_str().unwrap();
            format!("{path_str} {width}w")
        })
        .collect::<Vec<String>>()
        .join(", ")
}
