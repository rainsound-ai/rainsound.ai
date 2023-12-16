use crate::parse_macro_arguments::*;
use assets_runtime::{paths::*, CssAsset};
use proc_macro::TokenStream;
use quote::quote;
use std::process::Command;
use std::str::FromStr;
use std::{path::PathBuf, time::Duration};
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

pub fn build(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as BuildTailwindInput);
    crate::logger::init_logger(input.debug);

    log::info!("Building Tailwind.");

    let config = tailwind_config_path();
    let config_str = config
        .to_str()
        .expect("Error converting the path to the Tailwind config to a string.");
    log::info!("Using Tailwind config at {}.", config_str);

    let input_file = workspace_root_dir().join(&input.path_to_input_file);
    let input_file_str = input_file
        .to_str()
        .expect("Error converting the path to the Tailwind input CSS file to a string.");
    log::info!("Using Tailwind input CSS file at {}.", input_file_str);

    if !input_file.exists() {
        let error_message = format!("File not found: {}", input_file_str);

        return syn::Error::new(input.span, error_message)
            .to_compile_error()
            .into();
    }

    let output_file = built_assets_dir().join(&input.url_path);
    let output_file_str = output_file
        .to_str()
        .expect("Error converting the path to the Tailwind output CSS file to a string.");
    log::info!("Saving Tailwind output to {}.", output_file_str);

    let mut run_tailwind = Command::new("npx");

    run_tailwind
        .arg("tailwindcss")
        .args(["--config", config_str])
        .args(["--input", input_file_str])
        .args(["--output", output_file_str]);

    if input.minify {
        run_tailwind.arg("--minify");
    }

    log::info!("Invoking Tailwind CLI.");
    let tailwind_cli_output = run_tailwind
        .output()
        .expect("Error invoking the Tailwind CLI.");

    if !tailwind_cli_output.status.success() {
        let stdout = String::from_utf8(tailwind_cli_output.stdout)
            .expect("Error converting the Tailwind CLI's output to a string.");
        let stderr = String::from_utf8(tailwind_cli_output.stderr)
            .expect("Error converting the Tailwind CLI's error output to a string.");
        let error_message = format!(
            "Error building Tailwind.\nstdout:\n{}\n\nstderr:\n{}",
            stdout, stderr
        );
        log::error!("{}", error_message);
        return syn::Error::new(input.span, error_message)
            .to_compile_error()
            .into();
    }

    log::info!("Successfully built Tailwind.");

    let built_css = std::fs::read_to_string(output_file).expect("Error reading built.css file.");

    let css_asset = CssAsset::new(input.url_path, built_css, input.performance_budget);

    let output = quote! {
        #css_asset
    };

    output.into()
}

pub fn tailwind_config_path() -> PathBuf {
    assets_macros_dir().join("tailwind.config.js")
}

struct BuildTailwindInput {
    path_to_input_file: String,
    url_path: PathBuf,
    performance_budget: Duration,
    minify: bool,
    debug: bool,
    span: proc_macro2::Span,
}

impl Parse for BuildTailwindInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let input_span = input.span();

        let error_message = r#"Please make sure to pass arguments to build_tailwind! like this:

build_tailwind!(
    path_to_input_file: \"src/main.css\",
    url_path: \"built.css\",
    performance_budget_millis: 300,
    minify: true,
    debug: true,
);
"#;

        let error = syn::Error::new(input_span, error_message);

        let path_to_input_file =
            parse_named_string_argument("path_to_input_file", &input).ok_or(error.clone())?;

        let url_path_string =
            parse_named_string_argument("url_path", &input).ok_or(error.clone())?;
        let url_path = PathBuf::from_str(&url_path_string).expect("Error parsing url_path.");

        let performance_budget_millis =
            parse_named_u64_argument("performance_budget_millis", &input).ok_or(error.clone())?;
        let performance_budget = Duration::from_millis(performance_budget_millis);

        // True if we're in release mode (i.e. `cargo build --release`).
        let release_mode = !cfg!(debug_assertions);
        // Default to minifying in release mode.
        let minify_by_default = release_mode;
        let minify = parse_named_bool_argument("minify", &input).unwrap_or(minify_by_default);

        let debug = parse_named_bool_argument("debug", &input).unwrap_or(false);

        Ok(BuildTailwindInput {
            path_to_input_file,
            url_path,
            performance_budget,
            minify,
            debug,
            span: input_span,
        })
    }
}
