use image_register_rs::phase_correlation::{PhaseCorrelationResult, register_phase_correlation};

pub fn main() -> () {
    let leftimg = image::open("test_images/translated/at3_1m4_01_left.tif").unwrap();
    let rightimg = image::open("test_images/translated/at3_1m4_01_right.tif").unwrap();

    let PhaseCorrelationResult {
        translation_x,
        translation_y,
        cross_power_spectrum: _,
    } = register_phase_correlation(&leftimg, &rightimg).unwrap();

    println!("translation_x: {translation_x}, translation_y: {translation_y}.")
}
