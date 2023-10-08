use crate::workspace_root;
use std::fs;
use std::io::Write;
use std::process::Command;

pub fn build_tailwind(production: bool) -> String {
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

    let npx_prefix = workspace_root::dir().join("target").join("node_modules");
    let npx_prefix = npx_prefix.to_str().unwrap();

    let mut run_tailwind = Command::new("npx");

    let serverless_functions_glob = workspace_root::dir()
        .join("serverless_functions")
        .join("src")
        .join("**")
        .join("*.{html,rs,css}");
    let serverless_functions_glob = serverless_functions_glob.to_str().unwrap();

    let browser_glob = workspace_root::dir()
        .join("browser")
        .join("src")
        .join("**")
        .join("*.{html,rs,css}");
    let browser_glob = browser_glob.to_str().unwrap();

    let shared_glob = workspace_root::dir()
        .join("browser")
        .join("src")
        .join("**")
        .join("*.{html,rs,css}");
    let shared_glob = shared_glob.to_str().unwrap();

    let config = workspace_root::dir().join("tailwind.config.js");
    let config = config.to_str().unwrap();

    run_tailwind
        .args(["--prefix", npx_prefix])
        .arg("tailwindcss")
        .args(["--input", input_file])
        .args(["--output", output_file])
        .args(["--config", config]);
    // .args(["--content", serverless_functions_glob])
    // .args(["--content", browser_glob])
    // .args(["--content", shared_glob]);

    if production {
        run_tailwind.arg("--minify");
    }

    let output = run_tailwind.output().expect("Failed to execute process.");

    println!("status: {}", output.status);
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();

    fs::read_to_string(output_file).unwrap()
}
