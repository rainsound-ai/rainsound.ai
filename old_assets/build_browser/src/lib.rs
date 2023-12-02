use parse_macro_arguments::*;
use proc_macro::TokenStream;
use quote::quote;
use std::path::{Path, PathBuf};
use std::process::Command;
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

#[proc_macro]
pub fn build_browser_crate(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as BuildBrowserCrateInput);

    let log_level = if input.debug {
        log::Level::max()
    } else {
        log::Level::Warn
    };
    simple_logger::init_with_level(log_level).unwrap();

    // For some reason the `?` operator is erroring here.
    let maybe_wasm_pack_output = run_wasm_pack(input);
    let wasm_pack_output = match maybe_wasm_pack_output {
        Ok(wasm_pack_output) => wasm_pack_output,
        Err(error) => return error,
    };

    let path_to_minified_js = minify_js(wasm_pack_output.path_to_built_js);

    let path_to_built_wasm_str = wasm_pack_output
        .path_to_built_wasm
        .to_str()
        .expect("Error converting the path to the built wasm file to a string.");
    let path_to_minified_js_str = path_to_minified_js
        .to_str()
        .expect("Error converting the path to the minified JS file to a string.");

    let output = quote! {
        {
            pub struct BrowserCrate {
                pub built_wasm: &'static [u8],
                pub built_js: &'static str,
            }

            BrowserCrate {
                built_wasm: include_bytes!(#path_to_built_wasm_str),
                built_js: include_str!(#path_to_minified_js_str),
            }
        }
    };

    output.into()
}

fn run_wasm_pack(input: BuildBrowserCrateInput) -> Result<WasmPackOutput, TokenStream> {
    log::info!("Building browser crate.");

    let wasm_pack = workspace_root_dir()
        .join("target")
        .join("cargo_install")
        .join("bin")
        .join("wasm-pack")
        .to_string_lossy()
        .to_string();

    let out_dir = out_dir();
    let out_dir_str = out_dir.to_str().unwrap();
    let mut run_wasm_pack = Command::new(wasm_pack);

    run_wasm_pack
        .args(["build"])
        .args(["--no-pack"]) // For some reason generating a package.json causes errors when running `spin build`: Error: invalid type: sequence, expected a string at line 3 column 19
        .args(["--target", "web"])
        .args(["--out-dir", out_dir_str])
        .args(["--out-name", "browser"]);

    if input.production {
        run_wasm_pack.arg("--release");
    } else {
        run_wasm_pack.arg("--dev");
    }

    let browser_crate = workspace_root_dir().join(input.path_to_browser_crate);
    let browser_crate_str = browser_crate.to_str().unwrap();
    log::info!("Looking for browser crate at {}.", browser_crate_str);
    if !browser_crate.exists() {
        let error_message = format!("Folder not found: {}", browser_crate_str);

        let error: TokenStream = syn::Error::new(input.span, error_message)
            .to_compile_error()
            .into();

        return Err(error);
    }
    if !browser_crate.is_dir() {
        let error_message = format!("{} is not a folder:", browser_crate_str);

        let error: TokenStream = syn::Error::new(input.span, error_message)
            .to_compile_error()
            .into();

        return Err(error);
    }
    run_wasm_pack.arg(browser_crate_str);

    // cargo arguments
    if !input.production {
        run_wasm_pack.args(["--features", "dev"]);
    }

    log::info!("Invoking wasm-pack CLI.");
    let wasm_pack_output = run_wasm_pack
        .output()
        .expect("Error invoking the wasm-pack CLI.");

    if wasm_pack_output.status.success() {
        log::info!("Successfully built browser crate.");

        Ok(WasmPackOutput {
            path_to_built_wasm: out_dir.join("browser_bg.wasm"),
            path_to_built_js: out_dir.join("browser.js"),
        })
    } else {
        let stdout = String::from_utf8(wasm_pack_output.stdout)
            .expect("Error converting wasm-pack's stdout to a string.");
        let stderr = String::from_utf8(wasm_pack_output.stderr)
            .expect("Error converting wasm-pack's stderr to a string.");
        let error_message = format!(
            "Error building browser crate.\nstdout:\n{}\n\nstderr:\n{}",
            stdout, stderr
        );
        log::error!("{}", error_message);
        let error: TokenStream = syn::Error::new(input.span, error_message)
            .to_compile_error()
            .into();

        Err(error)
    }
}

struct WasmPackOutput {
    path_to_built_wasm: PathBuf,
    path_to_built_js: PathBuf,
}

fn out_dir() -> PathBuf {
    workspace_root_dir()
        .join("assets")
        .join("build_browser")
        .join("target")
}

fn minify_js(path_to_js: PathBuf) -> PathBuf {
    let source = std::fs::read_to_string(&path_to_js).expect("Error reading JS file.");
    let js_bytes = source.as_bytes();

    let session = minify_js::Session::new();
    let mut minified_bytes = Vec::new();
    minify_js::minify(
        &session,
        minify_js::TopLevelMode::Module,
        js_bytes,
        &mut minified_bytes,
    )
    .unwrap();

    let minified_string =
        String::from_utf8(minified_bytes).expect("Error converting minified JS bytes to string.");

    let path_to_minified_js = path_to_js.with_extension("min.js");
    std::fs::write(&path_to_minified_js, minified_string)
        .expect("Error writing minified JS to file.");

    path_to_minified_js
}

fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = std::env!("CARGO_WORKSPACE_DIR");
    Path::new(&cargo_workspace_dir).to_path_buf()
}

struct BuildBrowserCrateInput {
    path_to_browser_crate: String,
    production: bool,
    span: proc_macro2::Span,
    debug: bool,
}

impl Parse for BuildBrowserCrateInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let input_span = input.span();

        let error_message = r#"Please make sure to pass arguments to build_browser! like this:

build_browser_crate!(
    path_to_browser_crate: \"browser\",
    production: true
);

The path should be relative to the workspace root. You can also pass an optional `debug` argument like this:

build_browser_crate!(
    path_to_browser_crate: \"browser\",
    production: true,
    debug: true
);
"#;
        let error = syn::Error::new(input.span(), error_message);

        // Validate and parse `path_to_browser_crate`.
        //
        // This argument is required, so if it's not present we
        // convert None to an error and return early.
        let path_to_browser_crate = parse_named_string_argument(
            "path_to_browser_crate",
            &input,
            // ArgumentPosition::First
        )
        .ok_or(error.clone())?;

        // Validate and parse `production`.
        //
        // This argument is also required, so if it's not present we
        // convert None to an error and return early.
        let production = parse_named_bool_argument(
            "production",
            &input,
            // ArgumentPosition::NotFirst
        )
        .ok_or(error)?;

        // This argument is optional, so we default to `false` if it's not present.
        let debug = parse_named_bool_argument(
            "debug", &input,
            // ArgumentPosition::NotFirst
        )
        .unwrap_or(false);

        Ok(BuildBrowserCrateInput {
            path_to_browser_crate,
            production,
            debug,
            span: input_span,
        })
    }
}
