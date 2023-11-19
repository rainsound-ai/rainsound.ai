#![allow(non_upper_case_globals)]

use macro_helpers::*;
use proc_macro::TokenStream;
use quote::quote;
use std::path::{Path, PathBuf};
use std::process::Command;
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

#[proc_macro]
pub fn build_tailwind(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as BuildTailwindInput);

    let log_level = if input.debug {
        log::Level::max()
    } else {
        log::Level::Warn
    };
    simple_logger::init_with_level(log_level).unwrap();

    log::info!("Building Tailwind.");
    let build_tailwind_dir = workspace_root_dir().join("assets").join("build_tailwind");

    let config = build_tailwind_dir.join("tailwind.config.js");
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

    let output_file = build_tailwind_dir.join("target").join("built.css");
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

    if tailwind_cli_output.status.success() {
        log::info!("Successfully built Tailwind.");
    } else {
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

    let built_css = std::fs::read_to_string(output_file).expect("Error reading built.css file.");

    let output = quote! {
        #built_css
    };

    output.into()
}

fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = std::env!("CARGO_WORKSPACE_DIR");
    Path::new(&cargo_workspace_dir).to_path_buf()
}

struct BuildTailwindInput {
    path_to_input_file: String,
    minify: bool,
    span: proc_macro2::Span,
    debug: bool,
}

impl Parse for BuildTailwindInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let input_span = input.span();

        let error_message = r#"Please make sure to pass arguments to build_tailwind! like this:

build_tailwind!(
    path_to_input_file: \"src/main.css\",
    minify: true
);
"#;

        let error = syn::Error::new(input_span, error_message);

        let path_to_input_file =
            parse_named_string_argument("path_to_input_file", &input, ArgumentPosition::First)
                .ok_or(error.clone())?;

        let minify =
            parse_named_bool_argument("minify", &input, ArgumentPosition::NotFirst).ok_or(error)?;

        // Validate and parse "debug".
        let debug =
            parse_named_bool_argument("debug", &input, ArgumentPosition::NotFirst).unwrap_or(false);

        Ok(BuildTailwindInput {
            path_to_input_file,
            minify,
            debug,
            span: input_span,
        })
    }
}
