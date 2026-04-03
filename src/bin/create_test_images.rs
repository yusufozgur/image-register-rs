use image::GenericImageView;
use std::process;

const TEST_CROP_WIDTH: u32 = 400;
const TEST_CROP_HEIGHT: u32 = 400;
const TEST_X_OFFSET: u32 = 200;
const TEST_Y_OFFSET: u32 = 0;

const TEST_SOURCE: &str = "test_images/original/at3_1m4_01.tif";
const TEST_LEFT_CROP: &str = "test_images/translated/at3_1m4_01_left.tif";
const TEST_RIGHT_CROP: &str = "test_images/translated/at3_1m4_01_right.tif";
const TEST_REGISTERED_GROUND_TRUTH: &str = "test_images/translated/at3_1m4_01_ground_truth.tif";

fn main() {
    let path = TEST_SOURCE;

    let img = match image::open(path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Failed to open image at '{}'.", path);
            eprintln!("Reason: {}", error);
            process::exit(1);
        }
    };

    println!("Dimensions: {:?}", img.dimensions());

    let img_edited_left = img.view(0, 0, TEST_CROP_WIDTH, TEST_CROP_HEIGHT).to_image();
    let img_edited_right = img
        .view(
            0 + TEST_X_OFFSET,
            0 + TEST_Y_OFFSET,
            TEST_CROP_WIDTH,
            TEST_CROP_HEIGHT,
        )
        .to_image();

    let img_registered_ground_truth = img
        .view(
            0,
            0,
            TEST_CROP_WIDTH + TEST_X_OFFSET,
            TEST_CROP_HEIGHT + TEST_Y_OFFSET,
        )
        .to_image();

    img_edited_left
        .save(TEST_LEFT_CROP)
        .expect("Failed to save left image");

    img_edited_right
        .save(TEST_RIGHT_CROP)
        .expect("Failed to save right image");

    img_registered_ground_truth
        .save(TEST_REGISTERED_GROUND_TRUTH)
        .expect("Failed to save right image");
}
