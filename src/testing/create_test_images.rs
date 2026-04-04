use crate::testing::test_config::TestConfig;
use image::GenericImageView;
use std::{fs, path::Path};

pub fn create_test_images() {
    let test = TestConfig::new();

    let img = match image::open(&test.source) {
        Ok(file) => file,
        Err(error) => {
            panic!(
                "Error: Failed to open image at '{}'. Reason: {}.",
                &test.source, error
            );
        }
    };

    println!("Dimensions: {:?}", img.dimensions());

    let img_edited_left = img.view(0, 0, test.crop_width, test.crop_height).to_image();
    let img_edited_right = img
        .view(
            0 + test.x_offset,
            0 + test.y_offset,
            test.crop_width,
            test.crop_height,
        )
        .to_image();

    let img_registered_ground_truth = img
        .view(
            0,
            0,
            test.crop_width + test.x_offset,
            test.crop_height + test.y_offset,
        )
        .to_image();

    // create parents dir if not exist
    fs::create_dir_all(Path::new(&test.left_crop).parent().unwrap()).unwrap();
    fs::create_dir_all(Path::new(&test.right_crop).parent().unwrap()).unwrap();

    img_edited_left
        .save(test.left_crop)
        .expect("Failed to save left image");

    img_edited_right
        .save(test.right_crop)
        .expect("Failed to save right image");

    img_registered_ground_truth
        .save(test.registered_ground_truth)
        .expect("Failed to save right image");
}
