use proc_macro::TokenStream;
use quote::quote;
use rayon::prelude::*;
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

mod parse_macro_arguments;
use parse_macro_arguments::*;

mod tailwind;

#[proc_macro]
pub fn save_to_disk(input: TokenStream) -> TokenStream {
    eprintln!("parsing macro input");
    let input = syn::parse_macro_input!(input as SaveToDiskInput);
    eprintln!("parsed macro input");
    let log_level = if input.debug {
        log::Level::max()
    } else {
        log::Level::Warn
    };
    eprintln!("setting log level");
    eprintln!("log level: {:?}", log_level);
    if let Err(error) = simple_logger::init_with_level(log_level) {
        log::warn!("Error initializing logger: {}", error);
    }

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

#[proc_macro]
pub fn build_tailwind(input: TokenStream) -> TokenStream {
    tailwind::build(input)
}
