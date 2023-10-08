// use manifest;
// use std::fs;

fn main() {
    println!("Running serverless_functions build.rs.");
    let workspace_root_dir = new_assets::workspace_root::dir();
    println!("{}", workspace_root_dir.to_string_lossy());
    new_assets::tailwind::build_tailwind(true);

    //     // println!("Removing built folder");
    //     // if let Err(error) = fs::remove_dir_all(&built_dir) {
    //     //     println!("Error removing ./built folder: {}", error);
    //     // }

    //     println!("Creating built folder.");
    //     if let Err(error) = fs::create_dir(&built_dir) {
    //         println!("Error creating built folder: {}", error);
    //     }

    //     println!("Saving assets to disk.");
    //     Assets::new().save_to_disk(&built_dir);
}
