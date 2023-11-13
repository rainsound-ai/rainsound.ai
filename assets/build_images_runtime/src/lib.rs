use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::path::PathBuf;

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
            RunTimeBuiltImage {
                path_to_original_image: std::path::PathBuf::from(#self.path_to_original_image),
                resized_copies: vec![
                    #(#resized_copies),*
                ],
                placeholder: RunTimePlaceholder {
                    lqip_data_uri: #self.lqip_data_uri,
                    automatically_detected_color: #self.automatically_detected_color,
                },
                width: #self.width,
                height: #self.height,
            }
        })
    }
}

pub struct RunTimeResizedImage {
    pub bytes: &'static [u8],
    pub mime_type: &'static str,
    pub width: u32,
    pub height: u32,
    pub file_name: PathBuf,
}

impl ToTokens for RunTimeResizedImage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            RunTimeResizedImage {
                bytes: #self.bytes,
                mime_type: #self.mime_type,
                width: #self.width,
                height: #self.height,
                file_name: std::path::PathBuf::from(#self.file_name),
            }
        })
    }
}

pub struct RunTimePlaceholder {
    pub lqip_data_uri: &'static str,
    pub automatically_detected_color: &'static str,
}

impl ToTokens for RunTimePlaceholder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            RunTimePlaceholder {
                lqip_data_uri: #self.lqip_data_uri,
                automatically_detected_color: #self.automatically_detected_color,
            }
        })
    }
}
