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

    // print_code_for_debugging(&code);
    // eprintln!("{}", &code.to_string());

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

// We wrap our built images in Arcs because we need owned copies
// to use Rayon, and we presume that cloning BuiltImages is expensive
// because they hold giant piles of image bytes.
fn get_images_from_disk(input: BuildImagesInput) -> Vec<BuiltImage> {
    eprintln!("Getting original images.");
    let original_images = get_image_files(&input.path_to_images_dir);
    eprintln!("Found {} original images.", original_images.len());

    eprintln!("Generating placeholders.");
    original_images
        .into_par_iter()
        .map(|image_file| {
            BuiltImage::new(
                &input.path_to_images_dir,
                image_file.absolute_path_to_image,
                image_file.image,
            )
        })
        .collect()
}

fn get_image_files(path_to_images_dir: &Path) -> Vec<ImageFile> {
    WalkDir::new(path_to_images_dir)
        .into_iter()
        .filter_map(try_get_image_file_from_dir_entry)
        .collect()
}

fn try_get_image_file_from_dir_entry(
    maybe_dir_entry: walkdir::Result<walkdir::DirEntry>,
) -> Option<ImageFile> {
    match maybe_dir_entry {
        Ok(entry) if entry.file_type().is_dir() => None,
        Ok(entry) => {
            let path = entry.path();
            try_get_image_file_from_path(path)
        }
        Err(error) => {
            eprintln!("Error reading image directory: {:?}", error);
            None
        }
    }
}

fn try_get_image_file_from_path(path: &Path) -> Option<ImageFile> {
    match image::open(path) {
        Ok(dynamic_image) => {
            let image_file = ImageFile {
                absolute_path_to_image: path.to_path_buf(),
                image: dynamic_image,
            };
            Some(image_file)
        }
        Err(error) => {
            eprintln!("Error opening image {:?}: {:?}", path, error);
            None
        }
    }
}

struct ImageFile {
    absolute_path_to_image: PathBuf,
    image: DynamicImage,
}

fn generate_code(built_images: &[BuiltImage]) -> proc_macro2::TokenStream {
    eprintln!("Generating code for built images.");

    eprintln!("Building property names.");
    let built_image_property_names: Vec<_> = built_images
        .iter()
        .map(|built_image| format_ident!("{}", built_image.name_in_source_code))
        .collect();

    eprintln!("Instantiating RunTimeBuiltImages.");
    let built_image_property_declarations = built_images.iter().map(|built_image| {
        eprintln!("Constructing ident.");
        let name_in_source_code = format_ident!("{}", built_image.name_in_source_code);
        eprintln!("Instantiating RunTimeBuiltImage.");
        let run_time_built_image = RunTimeBuiltImage::from_built_image(built_image);

        eprintln!("Quoting RunTimeBuiltImage.");

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

#[allow(dead_code)]
fn print_code_for_debugging(token_stream: &proc_macro2::TokenStream) {
    let wrapped_in_main_function = quote! {
        fn main() {
            #token_stream
        }
    };

    let item = syn::parse2::<syn::Item>(wrapped_in_main_function).unwrap();

    let syn_file = syn::File {
        attrs: vec![],
        items: vec![item],
        shebang: None,
    };

    let formatted_code = prettyplease::unparse(&syn_file);

    eprintln!("{}", formatted_code);
}
