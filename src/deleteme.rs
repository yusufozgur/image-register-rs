use crate::phase_correlation::{PhaseCorrelationResult, compute_phase_correlation};
use crate::test_config::TestConfig;
#[test]
fn main() -> () {
    let test = TestConfig::new();

    let leftimg = match image::open(&test.left_crop) {
        Ok(file) => file,
        Err(error) => {
            panic!(
                "Error: Failed to open left image at '{}'. Reason: {}",
                &test.left_crop, error
            );
        }
    };

    let rightimg = match image::open(&test.right_crop) {
        Ok(file) => file,
        Err(error) => {
            panic!(
                "Error: Failed to open left image at '{}'. Reason: {}",
                &test.right_crop, error
            );
        }
    };
    let PhaseCorrelationResult {
        translation_x,
        translation_y,
        cross_power_spectrum,
    } = compute_phase_correlation(&leftimg, &rightimg).unwrap();

    print!(
        "translation_x: {}, translation_y: {}",
        translation_x, translation_y
    );

    //create parent dirs if not exist
    fs::create_dir_all(Path::new(&test.cross_power_spectrum).parent().unwrap()).unwrap();
    fs::create_dir_all(Path::new(&test.registered_result).parent().unwrap()).unwrap();
    fs::create_dir_all(Path::new(&test.registrated_metrics).parent().unwrap()).unwrap();

    // Save cross power spectrum as an image
    save_spectrum_as_image(
        &cross_power_spectrum,
        test.crop_width,
        test.crop_height,
        &test.cross_power_spectrum,
    )
    .unwrap();

    // merge them
    let registered_img = merge_images(&leftimg, &rightimg, translation_x, translation_y);
    registered_img.save(test.registered_result).unwrap();

    // save translation x and y to json
    let error_x = (test.x_offset as f64) - translation_x;
    let error_y = (test.y_offset as f64) - translation_y;
    if let Err(e) = save_translation_to_json(
        TranslationData {
            translation_x,
            translation_y,
            error_x: error_x,
            error_y: error_y,
        },
        test.registrated_metrics,
    ) {
        eprintln!("Failed to save JSON: {}", e);
    }

    // calculate error

    // testing

    assert_eq!(translation_x, test.x_offset as f64);
    assert_eq!(translation_y, test.y_offset as f64);
    assert_eq!(error_x, 0.0);
    assert_eq!(error_y, 0.0);

    return ();
}

use crate::merge_image::merge_images;
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

use serde::Serialize;
use std::fs::{self, File};
use std::io::BufWriter;

#[derive(Serialize)]
struct TranslationData {
    translation_x: f64,
    translation_y: f64,
    error_x: f64,
    error_y: f64,
}

/// Serializes translation coordinates to a JSON file at the specified path.
fn save_translation_to_json<P: AsRef<Path>>(
    data: TranslationData,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file and create a buffered writer
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    // Serialize and write to the file
    // use to_writer_pretty for human-readable output
    serde_json::to_writer_pretty(writer, &data)?;

    Ok(())
}
