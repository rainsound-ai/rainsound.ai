// use manifest;
// use std::fs;

fn main() {
    println!("Running serverless_functions build.rs.");

    // If you change these, also change them in tailwind.config.js.
    println!("cargo:rerun-if-changed=../**/*.html");
    println!("cargo:rerun-if-changed=../**/*.rs");
    println!("cargo:rerun-if-changed=../**/*.css");

    new_assets::build::tailwind::build_tailwind(true);
    new_assets::build::browser_crate::build_browser_crate(true);
    new_assets::non_html_assets.save_to_disk();
}
