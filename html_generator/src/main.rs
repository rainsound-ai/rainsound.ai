#![allow(non_snake_case, non_upper_case_globals)]

use std::fs;

pub mod assets;
pub mod extensions;
pub mod manifest;
pub mod prelude;
pub mod routes;

use prelude::*;

fn main() {
    // println!("Removing built folder");
    // if let Err(error) = fs::remove_dir_all(&built_dir) {
    //     println!("Error removing ./built folder: {}", error);
    // }
    Assets::new().save_to_disk();
}
