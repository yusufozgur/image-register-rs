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
