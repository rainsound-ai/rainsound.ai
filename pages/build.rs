use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("Running pages build.rs.");

    // Rerun.
    // If you change these, also change them in tailwind.config.js.
    println!("cargo:rerun-if-changed=../**/*.html");
    println!("cargo:rerun-if-changed=../**/*.rs");
    println!("cargo:rerun-if-changed=../**/*.css");
    build_serverless_functions_test();
}

fn build_serverless_functions_test() {
    run_wasm_pack(true);
    run_wasm_opt();
    // minify_js();
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

    let out_dir = workspace_root_dir().join("built").join(crate_name);
    let out_dir = out_dir.to_str().unwrap();

    dbg!(&wasm_pack);
    dbg!(&out_dir);
    dbg!(&crate_name);

    let mut run_wasm_pack = Command::new(wasm_pack);

    run_wasm_pack
        .args(["build"])
        .args(["--target", "web"])
        .args(["--out-dir", out_dir])
        .args(["--out-name", crate_name]);

    if production {
        run_wasm_pack.arg("--release");
    } else {
        run_wasm_pack.args(["--dev"]);
    }

    let path_to_this_crate = workspace_root_dir().join(crate_name);
    let path_to_this_crate = path_to_this_crate.to_str().unwrap();
    dbg!(&path_to_this_crate);
    run_wasm_pack.arg(path_to_this_crate);

    // cargo arguments
    // if !production {
    // run_wasm_pack.args(["--features", "dev"]);
    // }

    let exit_status = run_wasm_pack
        .spawn()
        .expect("Failed to execute process.")
        .wait()
        .expect("Failed to wait for process.");

    if exit_status.success() {
        println!("Successfully built {crate_name} crate.");
    } else {
        panic!("Failed to build {crate_name} crate.");
    }
}

fn run_wasm_opt() {
    println!("Running wasm-opt.");

    let wasm_opt = workspace_root_dir()
        .join("target")
        .join("cargo_install")
        .join("bin")
        .join("wasm-opt")
        .to_string_lossy()
        .to_string();

    let wasm_file_name = format!("{}_bg.wasm", crate_name);
    let input_file = workspace_root_dir()
        .join("built")
        .join(crate_name)
        .join(wasm_file_name)
        .to_string_lossy()
        .to_string();

    let output_file = input_file.clone();

    let mut run_wasm_opt = Command::new(wasm_opt);
    run_wasm_opt
        .arg("-Os") // Optimize for code size. Note, this is a capital letter "O", not a zero.
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

// fn minify_js() {
//     let browser_js = workspace_root_dir()
//         .join("target")
//         .join("browser")
//         .join("browser.js")
//         .to_string_lossy()
//         .to_string();

//     let browser_js_source =
//         std::fs::read_to_string(&browser_js).expect("Failed to read browser.js.");

//     let minified = minify_string(&browser_js_source);

//     std::fs::write(&browser_js, minified).expect("Failed to write browser.js.");
// }

// pub fn minify_string(source: &str) -> Vec<u8> {
//     let session = minify_js::Session::new();
//     let js_bytes = source.as_bytes();
//     let mut out = Vec::new();
//     minify_js::minify(
//         &session,
//         minify_js::TopLevelMode::Module,
//         js_bytes,
//         &mut out,
//     )
//     .unwrap();
//     out
// }

static crate_name: &str = "serverless_functions_test";

fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = std::env!("CARGO_WORKSPACE_DIR");
    Path::new(&cargo_workspace_dir).to_path_buf()
}
