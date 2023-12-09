use crate::parse_macro_arguments::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

#[deprecated(note = "Use individual asset macros like `build_tailwind!` instead.")]
pub fn save_assets_to_disk(input: TokenStream) -> TokenStream {
    eprintln!("parsing macro input");
    let input = syn::parse_macro_input!(input as SaveToDiskInput);
    eprintln!("parsed macro input");
    crate::logger::init_logger(input.debug);

    log::info!("Saving assets to disk.");

    let all_assets = assets_runtime::non_html_assets.all_assets();
    log::info!("Found {} assets.", all_assets.len());
    for asset in all_assets {
        asset.save_to_disk();
    }
    // .into_iter()
    // // .par_bridge()
    // .for_each(|asset| {
    // });

    log::info!("Done saving assets to disk.");

    let output = quote! {};
    output.into()
}

struct SaveToDiskInput {
    debug: bool,
}

impl Parse for SaveToDiskInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let debug = parse_named_bool_argument("debug", &input).unwrap_or(false);
        Ok(SaveToDiskInput { debug })
    }
}
