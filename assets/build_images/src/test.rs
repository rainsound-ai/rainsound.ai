#![cfg(test)]

use crate::*;

#[test]
pub fn it_works() {
    let images = build_images!(path_to_images_dir: "build_images/src/test_images");

    let test_image = images.test_image;
    assert_eq!(test_image.width, 3584);
    assert_eq!(test_image.height, 2298);

    let first_resized_copy = test_image.resized_copies.first().unwrap();

    assert_eq!(first_resized_copy.file_name, "test_image_100w.jpg");
}
