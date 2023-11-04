use std::path::{Path, PathBuf};
use std::process::Command;

pub fn main() {
    println!("cargo:rerun-if-changed=../**/*.html");
    println!("cargo:rerun-if-changed=../**/*.rs");
    println!("cargo:rerun-if-changed=../**/*.css");

    run_wasm_pack(true);
    minify_js();
}

fn run_wasm_pack(production: bool) {
    println!("Running wasm-pack.");

    let wasm_pack = workspace_root_dir()
        .join("target")
        .join("cargo_install")
        .join("bin")
        .join("wasm-pack")
        .to_string_lossy()
        .to_string();

    let out_dir = workspace_root_dir().join("target").join("browser");
    let out_dir = out_dir.to_str().unwrap();

    let mut run_wasm_pack = Command::new(wasm_pack);

    run_wasm_pack
        .args(["build"])
        .args(["--no-pack"]) // For some reason generating a package.json causes errors when running `spin build`: Error: invalid type: sequence, expected a string at line 3 column 19
        .args(["--target", "web"])
        .args(["--out-dir", out_dir])
        .args(["--out-name", "browser"]);

    if production {
        run_wasm_pack.arg("--release");
    } else {
        run_wasm_pack.arg("--dev");
    }

    let browser_crate = workspace_root_dir().join("browser");
    let browser_crate = browser_crate.to_str().unwrap();
    run_wasm_pack.arg(browser_crate);

    // cargo arguments
    if !production {
        run_wasm_pack.args(["--features", "dev"]);
    }

    println!("Invoking wasm-pack command.");
    let exit_status = run_wasm_pack
        .spawn()
        .expect("Failed to execute process.")
        .wait()
        .expect("Failed to wait for process.");

    if exit_status.success() {
        println!("Successfully built browser crate.");
    } else {
        panic!("Failed to build browser crate.");
    }
}

fn minify_js() {
    let browser_js = workspace_root_dir()
        .join("target")
        .join("browser")
        .join("browser.js")
        .to_string_lossy()
        .to_string();

    let browser_js_source =
        std::fs::read_to_string(&browser_js).expect("Failed to read browser.js.");

    let minified = minify_string(&browser_js_source);

    std::fs::write(&browser_js, minified).expect("Failed to write browser.js.");
}

pub fn minify_string(source: &str) -> Vec<u8> {
    let session = minify_js::Session::new();
    let js_bytes = source.as_bytes();
    let mut out = Vec::new();
    minify_js::minify(
        &session,
        minify_js::TopLevelMode::Module,
        js_bytes,
        &mut out,
    )
    .unwrap();
    out
}

pub fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = std::env!("CARGO_WORKSPACE_DIR");
    Path::new(&cargo_workspace_dir).to_path_buf()
}
