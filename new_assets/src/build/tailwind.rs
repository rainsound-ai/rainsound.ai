use crate::workspace_root;
use std::process::Command;

pub fn build_tailwind(production: bool) {
    println!("Building Tailwind.");

    let npx_prefix = workspace_root::dir().join("target").join("node_modules");
    let npx_prefix = npx_prefix.to_str().unwrap();

    let config = workspace_root::dir().join("tailwind.config.js");
    let config = config.to_str().unwrap();

    let input_file = workspace_root::dir()
        .join("serverless_functions")
        .join("src")
        .join("main.css");
    let input_file = input_file.to_str().unwrap();

    let output_file = workspace_root::dir()
        .join("target")
        .join("tailwind")
        .join("built.css");
    let output_file = output_file.to_str().unwrap();

    let mut run_tailwind = Command::new("npx");

    run_tailwind
        .args(["--prefix", npx_prefix])
        .arg("tailwindcss")
        .args(["--config", config])
        .args(["--input", input_file])
        .args(["--output", output_file]);

    if production {
        run_tailwind.arg("--minify");
    }

    let exit_status = run_tailwind
        .spawn()
        .expect("Failed to execute process.")
        .wait()
        .expect("Failed to wait for process.");

    if exit_status.success() {
        println!("Successfully built Tailwind.");
    } else {
        panic!("Failed to build Tailwind.");
    }
}
