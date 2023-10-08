use crate::workspace_root;
use std::process::Command;

// TO-DO: Run wasm-opt.
// ../target/cargo_install/bin/wasm-opt -Os -o ../target/browser/browser_bg.wasm ../target/browser/browser_bg.wasm

pub fn build_browser_crate(production: bool) {
    println!("Building browser crate.");

    let wasm_pack = workspace_root::dir()
        .join("target")
        .join("cargo_install")
        .join("bin")
        .join("wasm-pack");
    let wasm_pack = wasm_pack.to_str().unwrap();

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
