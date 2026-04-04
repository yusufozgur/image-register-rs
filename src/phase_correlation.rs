use image::{DynamicImage, GenericImageView};
use num_complex::Complex;
use rustfft::{FftPlanner, num_traits::Zero};

pub struct PhaseCorrelationResult {
    pub translation_x: f64,
    pub translation_y: f64,
    pub cross_power_spectrum: Vec<Complex<f64>>,
}

pub fn register_phase_correlation(
    img1: &DynamicImage,
    img2: &DynamicImage,
) -> Result<PhaseCorrelationResult, String> {
    let (width, height) = img1.dimensions();
    let (width2, height2) = img2.dimensions();

    if width != width2 || height != height2 {
        return Err(String::from("Images must have the same dimensions"));
    }

    let n = (width * height) as usize;
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    let ifft = planner.plan_fft_inverse(n);

    let mut buffer1 = img_to_complex_array(img1);
    let mut buffer2 = img_to_complex_array(img2);

    // Perform Forward FFT
    fft.process(&mut buffer1);
    fft.process(&mut buffer2);

    // Compute Cross-Power Spectrum
    let mut cross_power_spectrum = compute_normalized_cross_power(&buffer1, &buffer2);

    // Perform Inverse FFT to return to spatial domain
    ifft.process(&mut cross_power_spectrum);

    // Find the peak position
    let peak_idx = find_peak_index(&cross_power_spectrum);

    let ty = (peak_idx / width as usize) as f64;
    let tx = (peak_idx % width as usize) as f64;

    // Adjust for circular shift (wrap-around) to handle negative translations
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
        cross_power_spectrum,
    })
}

/// Converts a DynamicImage to a Vec of Complex numbers based on luminance.
fn img_to_complex_array(img: &DynamicImage) -> Vec<Complex<f64>> {
    img.to_luma8()
        .pixels()
        .map(|p| Complex::new(p[0] as f64, 0.0))
        .collect()
}

/// Computes the normalized cross-power spectrum: R = (F * G*) / |F * G*|
fn compute_normalized_cross_power(f: &[Complex<f64>], g: &[Complex<f64>]) -> Vec<Complex<f64>> {
    f.iter()
        .zip(g.iter())
        .map(|(&f_val, &g_val)| {
            let combined = f_val * g_val.conj();
            let norm = combined.norm();
            if norm > 1e-9 {
                combined / norm
            } else {
                Complex::zero()
            }
        })
        .collect()
}

/// Identifies the index of the peak value in the real component of the spatial domain.
fn find_peak_index(spatial_data: &[Complex<f64>]) -> usize {
    spatial_data
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.re.partial_cmp(&b.re).unwrap())
        .map(|(index, _)| index)
        .expect("Cannot find peak index: spatial_data slice is empty")
}
