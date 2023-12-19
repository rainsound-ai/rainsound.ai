use proc_macro::TokenStream;

mod browser_crate;
mod font;
mod images;
mod logger;
mod parse_macro_arguments;
mod tailwind;

#[proc_macro]
pub fn build_tailwind(input: TokenStream) -> TokenStream {
    tailwind::build(input)
}

#[proc_macro]
pub fn build_browser_crate(input: TokenStream) -> TokenStream {
    browser_crate::build(input)
}

#[proc_macro]
pub fn build_image(input: TokenStream) -> TokenStream {
    images::build_image(input)
}

#[proc_macro]
pub fn build_font(input: TokenStream) -> TokenStream {
    font::build(input)
}

// #[proc_macro]
// pub fn build_images(input: TokenStream) -> TokenStream {
//     lock_file::with_lock_file("build_images", || images::build_images_in_folder(input))
// }
