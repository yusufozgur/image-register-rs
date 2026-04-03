use image::{DynamicImage, GenericImageView};
use num_complex::Complex;
use rustfft::{FftPlanner, num_traits::Zero};

pub struct PhaseCorrelationResult {
    pub translation_x: f64,
    pub translation_y: f64,
    pub cross_power_spectrum: Vec<Complex<f64>>,
}

pub fn compute_phase_correlation(
    img1: &DynamicImage,
    img2: &DynamicImage,
) -> Result<PhaseCorrelationResult, String> {
    let (width, height) = img1.dimensions();
    let (width2, height2) = img2.dimensions();

    if width != width2 || height != height2 {
        return Err(String::from("Images must have the same dimensions"));
    }

    let n = (width * height) as usize;
    // rustfft uses these planner and fft objects to perform fft
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    let ifft = planner.plan_fft_inverse(n);

    // Convert images to Complex planes
    let mut buffer1: Vec<Complex<f64>> = img1
        .to_luma8()
        .pixels()
        .map(|p| Complex::new(p[0] as f64, 0.0))
        .collect();
    let mut buffer2: Vec<Complex<f64>> = img2
        .to_luma8()
        .pixels()
        .map(|p| Complex::new(p[0] as f64, 0.0))
        .collect();

    // Perform Forward FFT
    fft.process(&mut buffer1);
    fft.process(&mut buffer2);

    // 3. Compute Cross-Power Spectrum
    // R = (F * G*) / |F * G*|
    let mut cross_power_spectrum: Vec<Complex<f64>> = buffer1
        .iter()
        .zip(buffer2.iter())
        .map(|(&f, &g)| {
            let combined = f * g.conj();
            let norm = combined.norm();
            if norm > 0.0 {
                combined / norm
            } else {
                Complex::zero()
            }
        })
        .collect();

    // 4. Perform Inverse FFT
    ifft.process(&mut cross_power_spectrum);

    // 5. Find the peak position
    let mut max_val = -1.0;
    let mut peak_idx = 0;

    for (i, val) in cross_power_spectrum.iter().enumerate() {
        let mag = val.re; // The peak will be in the real component
        if mag > max_val {
            max_val = mag;
            peak_idx = i;
        }
    }

    // 6. Convert index to 2D coordinates
    let ty = (peak_idx / width as usize) as f64;
    let tx = (peak_idx % width as usize) as f64;

    // Adjust for circular shift (wrap-around)
    let final_tx = if tx > (width as f64 / 2.0) {
        tx - width as f64
    } else {
        tx
    };
    let final_ty = if ty > (height as f64 / 2.0) {
        ty - height as f64
    } else {
        ty
    };

    Ok(PhaseCorrelationResult {
        translation_x: final_tx,
        translation_y: final_ty,
        cross_power_spectrum: cross_power_spectrum,
    })
}

use image::RgbaImage;

pub fn merge_images(
    img1: &DynamicImage,
    img2: &DynamicImage,
    translation_x: f64,
    translation_y: f64,
) -> RgbaImage {
    let (tx, ty) = (translation_x.round() as i64, translation_y.round() as i64);

    // Determine the bounds of the new canvas
    let min_x = 0.min(tx);
    let max_x = (img1.width() as i64).max(img2.width() as i64 + tx);
    let min_y = 0.min(ty);
    let max_y = (img1.height() as i64).max(img2.height() as i64 + ty);

    let canvas_width = (max_x - min_x) as u32;
    let canvas_height = (max_y - min_y) as u32;

    let mut canvas = RgbaImage::new(canvas_width, canvas_height);

    // Calculate relative offsets
    let left_offset_x = (0 - min_x) as i64;
    let left_offset_y = (0 - min_y) as i64;
    let right_offset_x = (tx - min_x) as i64;
    let right_offset_y = (ty - min_y) as i64;

    // Place left image
    image::imageops::overlay(&mut canvas, img1, left_offset_x, left_offset_y);

    // Place right image
    // Note: If you want transparency blending, you'd iterate pixels.
    // Overlay simply replaces the pixels.
    image::imageops::overlay(&mut canvas, img2, right_offset_x, right_offset_y);

    canvas
}

pub struct TestConfig {
    pub crop_width: u32,
    pub crop_height: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub source: String,
    pub left_crop: String,
    pub right_crop: String,
    pub registered_ground_truth: String,
    pub cross_power_spectrum: String,
    pub registrated_metrics: String,
    pub registered_result: String,
    pub registered_error: String,
}

impl TestConfig {
    pub fn new() -> Self {
        Self {
            crop_width: 400,
            crop_height: 400,
            x_offset: 200,
            y_offset: 0,
            source: "test_images/original/at3_1m4_01.tif".to_string(),
            left_crop: "test_images/translated/at3_1m4_01_left.tif".to_string(),
            right_crop: "test_images/translated/at3_1m4_01_right.tif".to_string(),
            registered_ground_truth: "test_images/translated/at3_1m4_01_ground_truth.tif"
                .to_string(),
            cross_power_spectrum: "test_images/translated/at3_1m4_01_cross_power_spectrum.tif"
                .to_string(),
            registrated_metrics: "test_images/translated/at3_1m4_01_registered_metrics.json"
                .to_string(),
            registered_result: "test_images/translated/at3_1m4_01_registered_result.tif"
                .to_string(),
            registered_error: "test_images/translated/at3_1m4_01_registered_error.tif".to_string(),
        }
    }
}
