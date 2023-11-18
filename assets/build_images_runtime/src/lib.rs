use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::path::PathBuf;

#[derive(Clone)]
pub struct RunTimeBuiltImage {
    pub path_starting_from_images_dir: PathBuf,
    pub absolute_path_to_original_image: PathBuf,
    pub resized_copies: Vec<RunTimeResizedImage>,
    pub placeholder: RunTimePlaceholder,
    pub width: u32,
    pub height: u32,
}

impl ToTokens for RunTimeBuiltImage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        eprintln!("Converting RunTimeBuiltImage to tokens.");

        let resized_copies = self.resized_copies.iter().map(|resized_copy| {
            eprintln!("Quoting resized copy.");
            quote! {
                #resized_copy
            }
        });
        eprintln!("Done quoting resized copies.");

        let path_starting_from_images_dir = self.path_starting_from_images_dir.to_str().unwrap();
        let absolute_path_to_original_image =
            self.absolute_path_to_original_image.to_str().unwrap();
        let width = &self.width;
        let height = &self.height;

        let placeholder = &self.placeholder;
        let placeholder = quote! {
            #placeholder
        };

        tokens.extend(quote! {
            RunTimeBuiltImage {
                path_starting_from_images_dir: std::path::PathBuf::from(#path_starting_from_images_dir),
                absolute_path_to_original_image: std::path::PathBuf::from(#absolute_path_to_original_image),
                resized_copies: vec![
                    #(#resized_copies),*
                ],
                placeholder: #placeholder,
                width: #width,
                height: #height,
            }
        })
    }
}

#[derive(Clone)]
pub struct RunTimeResizedImage {
    pub bytes: Vec<u8>,
    pub mime_type: String,
    pub width: u32,
    pub height: u32,
    pub path_starting_from_images_dir: PathBuf,
    pub absolute_path: PathBuf,
}

impl ToTokens for RunTimeResizedImage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        eprintln!("Converting RunTimeResizedImage to tokens.");
        let mime_type = &self.mime_type;
        let width = &self.width;
        let height = &self.height;
        let path_starting_from_images_dir = self.path_starting_from_images_dir.to_str().unwrap();
        let absolute_path = self.absolute_path.to_str().unwrap();

        tokens.extend(quote! {
            RunTimeResizedImage {
                bytes: include_bytes!(#absolute_path).to_vec(),
                mime_type: #mime_type.to_string(),
                width: #width,
                height: #height,
                path_starting_from_images_dir: std::path::PathBuf::from(#path_starting_from_images_dir),
                absolute_path: std::path::PathBuf::from(#absolute_path),
            }
        })
    }
}

#[derive(Clone)]
pub struct RunTimePlaceholder {
    pub lqip_data_uri: String,
    pub automatically_detected_color: String,
}

impl ToTokens for RunTimePlaceholder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        eprintln!("Converting RunTimePlaceholder to tokens.");
        let lqip_data_uri = &self.lqip_data_uri;
        let automatically_detected_color = &self.automatically_detected_color;

        tokens.extend(quote! {
            RunTimePlaceholder {
                lqip_data_uri: #lqip_data_uri.to_string(),
                automatically_detected_color: #automatically_detected_color.to_string(),
            }
        })
    }
}
