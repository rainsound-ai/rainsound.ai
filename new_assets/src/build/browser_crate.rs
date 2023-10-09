use crate::workspace_root;
use std::process::Command;

pub fn build_browser_crate(production: bool) {
    run_wasm_pack(production);
    run_wasm_opt();
    minify_js();
}

fn run_wasm_pack(production: bool) {
    println!("Building browser crate.");

    let wasm_pack = workspace_root::dir()
        .join("target")
        .join("cargo_install")
        .join("bin")
        .join("wasm-pack")
        .to_string_lossy()
        .to_string();

    let out_dir = workspace_root::dir().join("target").join("browser");
    let out_dir = out_dir.to_str().unwrap();

    let mut run_wasm_pack = Command::new(wasm_pack);

    run_wasm_pack
        .args(["build"])
        .args(["--target", "web"])
        .args(["--out-dir", out_dir])
        .args(["--out-name", "browser"]);

    if production {
        run_wasm_pack.arg("--release");
    } else {
        run_wasm_pack.args(["--dev"]);
    }

    let browser_crate = workspace_root::dir().join("browser");
    let browser_crate = browser_crate.to_str().unwrap();
    run_wasm_pack.arg(browser_crate);

    // cargo arguments
    if !production {
        run_wasm_pack.args(["--features", "dev"]);
    }

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

fn run_wasm_opt() {
    println!("Building browser crate.");

    let wasm_opt = workspace_root::dir()
        .join("target")
        .join("cargo_install")
        .join("bin")
        .join("wasm-opt")
        .to_string_lossy()
        .to_string();

    let input_file = workspace_root::dir()
        .join("target")
        .join("browser")
        .join("browser_bg.wasm")
        .to_string_lossy()
        .to_string();

    let output_file = input_file.clone();

    let mut run_wasm_opt = Command::new(wasm_opt);
    run_wasm_opt
        .arg("-Os") // Optimize for code size. Note, this is a capital "O", not a zero.
        .args(["--output", &output_file])
        .arg(input_file);

    let exit_status = run_wasm_opt
        .spawn()
        .expect("Failed to execute process.")
        .wait()
        .expect("Failed to wait for process.");

    if exit_status.success() {
        println!("Successfully optimized browser crate.");
    } else {
        panic!("Failed to optimize browser crate.");
    }
}

fn minify_js() {
    let browser_js = workspace_root::dir()
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
