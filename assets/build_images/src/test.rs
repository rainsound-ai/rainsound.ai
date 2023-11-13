#![cfg(test)]

use crate::*;

#[test]
pub fn it_works() {
    let images = build_images!(path_to_images_dir: "build_images/src/test_images");

    assert_eq!(images.test_image.width, 3584);
    assert_eq!(images.test_image.height, 2298);
}
