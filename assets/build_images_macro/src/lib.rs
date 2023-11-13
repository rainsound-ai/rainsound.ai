use build_images_runtime::*;
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
mod run_time_built_image_extension;
mod run_time_resized_image_extension;
use run_time_built_image_extension::*;

#[proc_macro]
pub fn build_images(input: TokenStream) -> TokenStream {
    eprintln!("Building images.");
    let input = syn::parse_macro_input!(input as BuildImagesInput);

    let built_images = get_images_from_disk(input);
    let code = generate_code(&built_images);
    // We do this last because we need the owned copy of the data
    // to use Rayon.
    save_images_to_disk_if_necessary(built_images);

    eprintln!("{}", &code.to_string());

    code.into()
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

fn get_images_from_disk(input: BuildImagesInput) -> Vec<BuiltImage> {
    eprintln!("Getting original images.");
    let original_images = get_image_files(&input.path_to_images_dir);
    eprintln!("Found {} original images.", original_images.len());

    eprintln!("Generating placeholders.");
    original_images
        .into_iter()
        .map(|(path_to_original_image, original_image)| {
            BuiltImage::new(path_to_original_image, original_image)
        })
        .collect()
}

fn get_image_files(path_to_images_dir: &Path) -> Vec<(PathBuf, DynamicImage)> {
    WalkDir::new(path_to_images_dir)
        .into_iter()
        .filter_map(|maybe_entry| match maybe_entry {
            Ok(entry) if entry.file_type().is_dir() => None,
            Ok(entry) => {
                let path = entry.path();
                match image::open(path) {
                    Ok(dynamic_image) => Some((path.to_path_buf(), dynamic_image)),
                    Err(error) => {
                        eprintln!("Error opening image {:?}: {:?}", path, error);
                        None
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading image directory: {:?}", error);
                None
            }
        })
        .collect()
}

fn generate_code(built_images: &[BuiltImage]) -> proc_macro2::TokenStream {
    eprintln!("Generating code for built images.");
    let built_image_property_names: Vec<_> = built_images
        .iter()
        .map(|built_image| format_ident!("{}", built_image.name_in_source_code))
        .collect();

    let built_image_property_declarations = built_images.iter().map(|built_image| {
        let name_in_source_code = format_ident!("{}", built_image.name_in_source_code);
        let run_time_built_image = RunTimeBuiltImage::from_built_image(built_image);

        quote! {
            #name_in_source_code: #run_time_built_image,
        }
    });

    quote! {
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
    }
}

fn save_images_to_disk_if_necessary(built_images: Vec<BuiltImage>) {
    // We have to iterate over owned values because we want to use Rayon.
    let resized_images: Vec<_> = built_images
        .into_iter()
        .flat_map(|built_image| built_image.resized_copies.into_iter())
        .collect();

    eprintln!("Saving resized images to disk if necessary.");
    resized_images
        .into_iter()
        .par_bridge()
        .for_each(|resized_image| {
            eprintln!("Generating resized image {}.", resized_image.path.display());
            resized_image.save_to_disk_if_necessary();
        });
}
