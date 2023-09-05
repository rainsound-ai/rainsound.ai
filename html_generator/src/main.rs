#![allow(non_snake_case, non_upper_case_globals)]

use std::fs;

pub mod assets;
pub mod extensions;
pub mod manifest;
pub mod prelude;
pub mod routes;

use prelude::*;

#[tokio::main]
async fn main() {
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
    Assets::new().await.save_to_disk(&built_dir);
}
