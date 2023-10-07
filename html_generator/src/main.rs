use html_generator::assets::Assets;
use html_generator::manifest;
use std::fs;

fn main() {
    let built_dir = manifest::dir().join("built");

    // println!("Removing built folder");
    // if let Err(error) = fs::remove_dir_all(&built_dir) {
    //     println!("Error removing ./built folder: {}", error);
    // }

    println!("Creating built folder.");
    if let Err(error) = fs::create_dir(&built_dir) {
        println!("Error creating built folder: {}", error);
    }

    println!("Saving assets to disk.");
    Assets::new().save_to_disk(&built_dir);
}
