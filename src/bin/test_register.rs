use std::process;

use image_register_rs::PhaseCorrelationResult;
use image_register_rs::TestConfig;
use image_register_rs::compute_phase_correlation;
fn main() {
    let test = TestConfig::new();

    let leftimg = match image::open(&test.left_crop) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Failed to open image at '{}'.", &test.left_crop);
            eprintln!("Reason: {}", error);
            process::exit(1);
        }
    };

    let rightimg = match image::open(&test.right_crop) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Failed to open image at '{}'.", &test.right_crop);
            eprintln!("Reason: {}", error);
            process::exit(1);
        }
    };
    let PhaseCorrelationResult {
        translation_x: final_tx,
        translation_y: final_ty,
        cross_power_spectrum,
    } = compute_phase_correlation(&leftimg, &rightimg).unwrap();

    print!("final_tx: {}, final_ty: {}", final_tx, final_ty);

    // Save cross power spectrum as an image
    save_spectrum_as_image(
        &cross_power_spectrum,
        test.crop_width,
        test.crop_height,
        &test.cross_power_spectrum,
    )
    .unwrap();

    // merge them

    // calculate error
}

use image::{ImageBuffer, Luma};
use num_complex::Complex;
use std::path::Path;

/// Normalizes a complex-valued cross-power spectrum and saves it as a grayscale image.
/// Uses logarithmic scaling to ensure visibility of lower-magnitude components.
fn save_spectrum_as_image(
    spectrum: &[Complex<f64>],
    width: u32,
    height: u32,
    path: &str,
) -> Result<(), image::ImageError> {
    // 1. Calculate magnitudes
    let magnitudes: Vec<f64> = spectrum.iter().map(|c| c.norm()).collect();

    // 2. Find the maximum magnitude for normalization
    let max_mag = magnitudes.iter().fold(0.0f64, |a, &b| a.max(b));

    // 3. Create the ImageBuffer
    let mut img_buf = ImageBuffer::new(width, height);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let index = (y * width + x) as usize;
        let mag = magnitudes[index];

        // Apply logarithmic scaling: s = log(1 + mag) / log(1 + max_mag) * 255
        // This compresses the dynamic range so the noise and secondary peaks are visible.
        let intensity = if max_mag > 0.0 {
            ((mag + 1.0).ln() / (max_mag + 1.0).ln() * 255.0) as u8
        } else {
            0
        };

        *pixel = Luma([intensity]);
    }

    // 4. Save to the specified path
    img_buf.save(Path::new(path))
}
