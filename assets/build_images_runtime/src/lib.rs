use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::path::PathBuf;

#[derive(Clone)]
pub struct RunTimeBuiltImage {
    pub path_to_original_image: PathBuf,
    pub resized_copies: Vec<RunTimeResizedImage>,
    pub placeholder: RunTimePlaceholder,
    pub width: u32,
    pub height: u32,
}

impl ToTokens for RunTimeBuiltImage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let resized_copies = self.resized_copies.iter().map(|resized_copy| {
            quote! {
                #resized_copy
            }
        });

        tokens.extend(quote! {
            build_images_runtime::RunTimeBuiltImage {
                path_to_original_image: std::path::PathBuf::from(#self.path_to_original_image),
                resized_copies: vec![
                    #(#resized_copies),*
                ],
                placeholder: build_images_runtime::RunTimePlaceholder {
                    lqip_data_uri: #self.lqip_data_uri,
                    automatically_detected_color: #self.automatically_detected_color,
                },
                width: #self.width,
                height: #self.height,
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
    pub file_name: PathBuf,
}

impl ToTokens for RunTimeResizedImage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            build_images_runtime::RunTimeResizedImage {
                bytes: #self.bytes,
                mime_type: #self.mime_type,
                width: #self.width,
                height: #self.height,
                file_name: std::path::PathBuf::from(#self.file_name),
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
        tokens.extend(quote! {
            build_images_runtime::RunTimePlaceholder {
                lqip_data_uri: #self.lqip_data_uri,
                automatically_detected_color: #self.automatically_detected_color,
            }
        })
    }
}
