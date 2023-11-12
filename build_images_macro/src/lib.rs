use image::DynamicImage;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Result as SynResult, Token,
};
use walkdir::WalkDir;

mod built_image;
use built_image::*;
mod dynamic_image_extension;
mod paths;

#[proc_macro]
pub fn build_images(input: TokenStream) -> TokenStream {
    eprintln!("Building images.");
    let input = syn::parse_macro_input!(input as BuildImagesInput);

    eprintln!("Getting original images.");
    let original_images = get_original_images(&input.path_to_images_dir);
    eprintln!("Found {} original images.", original_images.len());

    eprintln!("Generating placeholders.");
    let built_images: Vec<_> = original_images
        .into_iter()
        .map(|(path_to_original_image, original_image)| {
            BuiltImage::new(path_to_original_image, original_image)
        })
        .collect();

    let resized_images: Vec<_> = built_images
        .iter()
        .flat_map(|built_image| built_image.resized_copies.iter())
        .collect();

    eprintln!("Saving resized images to disk if necessary.");
    resized_images
        .into_iter()
        .par_bridge()
        .for_each(|resized_image| {
            eprintln!("Generating resized image {}.", resized_image.path.display());
            resized_image.save_to_disk_if_necessary();
        });

    eprintln!("Generating code for built images.");
    let built_image_property_names: Vec<_> = built_images
        .iter()
        .map(|built_image| format_ident!("{}", built_image.name_in_source_code))
        .collect();

    let built_image_property_declarations = built_images.iter().map(|built_image| {
        let name_in_source_code = format_ident!("{}", built_image.name_in_source_code);
        let path_to_original_image = &built_image
            .path_to_original_image
            .to_string_lossy()
            .to_string();

        let resized_copies: Vec<_> = built_image
            .resized_copies
            .iter()
            .map(|resized_copy| {
                let path = &resized_copy.path.to_string_lossy().to_string();
                let mime_type = resized_copy.mime_type.to_string();
                let width = &resized_copy.width;
                let height = &resized_copy.height;

                let run_time_resized_image = quote! {
                    RunTimeResizedImage {
                        bytes: include_bytes!(#path),
                        mime_type: #mime_type,
                        width: #width,
                        height: #height,
                    }
                };

                run_time_resized_image
            })
            .collect();

        let lqip_data_uri = &built_image.placeholder.lqip;
        let automatically_detected_color = &built_image.placeholder.automatically_detected_color;
        let width = &built_image.width;
        let height = &built_image.height;

        quote! {
            #name_in_source_code: RunTimeBuiltImage {
                width: #width,
                height: #height,
                resized_copies: vec![
                    #(#resized_copies),*
                ],
                path_to_original_image: PathBuf::from_str(#path_to_original_image).unwrap(),
                placeholder: RunTimePlaceholder {
                    lqip_data_uri: #lqip_data_uri,
                    automatically_detected_color: #automatically_detected_color,
                }
            },
        }
    });

    let output = quote! {
        {
            use std::path::PathBuf;
            use std::str::FromStr;

            pub struct BuiltImages {
                #(
                    pub #built_image_property_names: RunTimeBuiltImage,
                )*
            }

            BuiltImages {
                #( #built_image_property_declarations )*
            }
        }
    };

    // eprintln!("{}", &output.to_string());

    output.into()
}

struct BuildImagesInput {
    path_to_images_dir: PathBuf,
}

impl Parse for BuildImagesInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let error_message = r#"Please make sure to pass arguments to build_images! like this:

build_images!(path_to_images_dir: \"src/original_images\");
"#;

        // Validate and parse "path_to_images_dir".
        let key: Result<Ident, _> = input.parse();
        let parsed_key = match key {
            Ok(parsed_key) => parsed_key,
            Err(error) => return Err(syn::Error::new(error.span(), error_message)),
        };
        if parsed_key != "path_to_images_dir" {
            return Err(syn::Error::new(input.span(), error_message));
        }
        let _: Token![:] = input.parse()?;
        let path_to_images_dir_starting_at_workspace_root_literal: LitStr = input.parse()?;
        let path_to_images_dir_starting_at_workspace_root_string =
            path_to_images_dir_starting_at_workspace_root_literal.value();
        let path_to_images_dir =
            paths::workspace_root_dir().join(path_to_images_dir_starting_at_workspace_root_string);

        Ok(BuildImagesInput { path_to_images_dir })
    }
}

fn get_original_images(path_to_images_dir: &Path) -> Vec<(PathBuf, DynamicImage)> {
    WalkDir::new(path_to_images_dir)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.expect("Error reading sub-directory. Apologies — I know this error message is unhelpful. If you actually see it, let's put the effort into making it better.");
            let path = entry.path();
            image::open(path).ok().map(|dynamic_image| (path.to_path_buf(), dynamic_image))
        })
        .collect()
}
