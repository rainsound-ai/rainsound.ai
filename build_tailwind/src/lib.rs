#![allow(non_upper_case_globals)]

use proc_macro::TokenStream;
use quote::quote;
use std::path::{Path, PathBuf};
use std::process::Command;
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitBool, LitStr, Result as SynResult, Token,
};

#[proc_macro]
pub fn build_tailwind(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as BuildTailwindInput);

    eprintln!("[build_tailwind] Building Tailwind.");
    let build_tailwind_dir = workspace_root_dir().join("build_tailwind");

    let config = build_tailwind_dir.join("tailwind.config.js");
    let config_str = config
        .to_str()
        .expect("Error converting the path to the Tailwind config to a string.");
    eprintln!("[build_tailwind] Using Tailwind config at {}.", config_str);

    let input_file = workspace_root_dir().join(&input.path_to_input_file);
    let input_file_str = input_file
        .to_str()
        .expect("Error converting the path to the Tailwind input CSS file to a string.");
    eprintln!(
        "[build_tailwind] Using Tailwind input CSS file at {}.",
        input_file_str
    );

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
    eprintln!(
        "[build_tailwind] Saving Tailwind output to {}.",
        output_file_str
    );

    let mut run_tailwind = Command::new("npx");

    run_tailwind
        .arg("tailwindcss")
        .args(["--config", config_str])
        .args(["--input", input_file_str])
        .args(["--output", output_file_str]);

    if input.minify {
        run_tailwind.arg("--minify");
    }

    eprintln!("[build_tailwind] Invoking Tailwind CLI.");
    let tailwind_cli_output = run_tailwind
        .output()
        .expect("Error invoking the Tailwind CLI.");

    if tailwind_cli_output.status.success() {
        eprintln!("[build_tailwind] Successfully built Tailwind.");
    } else {
        let stdout = String::from_utf8(tailwind_cli_output.stdout)
            .expect("Error converting the Tailwind CLI's output to a string.");
        let stderr = String::from_utf8(tailwind_cli_output.stderr)
            .expect("Error converting the Tailwind CLI's error output to a string.");
        let error_message = format!(
            "Error building Tailwind.\nstdout:\n{}\n\nstderr:\n{}",
            stdout, stderr
        );
        eprintln!("[build_tailwind] {}", error_message);
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

        // Validate and parse "path_to_input_file".
        let key: Result<Ident, _> = input.parse();
        let parsed_key = match key {
            Ok(parsed_key) => parsed_key,
            Err(error) => return Err(syn::Error::new(error.span(), error_message)),
        };
        if parsed_key != "path_to_input_file" {
            return Err(syn::Error::new(input.span(), error_message));
        }
        let _: Token![:] = input.parse()?;
        let path_to_input_file: LitStr = input.parse()?;

        // Parse the comma.
        let _: Token![,] = input.parse()?;

        // Validate and parse "minify".
        let key: Result<Ident, _> = input.parse();
        let parsed_key = match key {
            Ok(parsed_key) => parsed_key,
            Err(error) => return Err(syn::Error::new(error.span(), error_message)),
        };
        if parsed_key != "minify" {
            return Err(syn::Error::new(input.span(), error_message));
        }
        let _: Token![:] = input.parse()?;
        let minify: LitBool = input.parse()?;

        Ok(BuildTailwindInput {
            path_to_input_file: path_to_input_file.value(),
            minify: minify.value,
            span: input_span,
        })
    }
}
