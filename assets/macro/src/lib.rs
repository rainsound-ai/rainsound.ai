use proc_macro::TokenStream;

mod browser_crate;
mod images;
mod lock_file;
mod logger;
mod parse_macro_arguments;
mod tailwind;

#[proc_macro]
pub fn build_tailwind(input: TokenStream) -> TokenStream {
    // block_until_other_invocations_are_finished("build_tailwind");
    // tailwind::build(input)
    lock_file::with_lock_file("build_tailwind", || tailwind::build(input))
}

#[proc_macro]
pub fn build_browser_crate(input: TokenStream) -> TokenStream {
    lock_file::with_lock_file("build_browser_crate", || browser_crate::build(input))
}

#[proc_macro]
pub fn build_images(input: TokenStream) -> TokenStream {
    lock_file::with_lock_file("build_images", || images::build_all_images_in_folder(input))
}
