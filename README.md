# image-register-rs

This rust library provides capabilities for image registration via phase correlation.

<img src="./overview.svg" width="30%" alt="overview image">

## Example

```rust
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

```

You can also take look at the test example at src/testing/test_register.rs, and test inputs and outputs at test_images/.

## API

```rust
pub fn register_phase_correlation(
    img1: &DynamicImage,
    img2: &DynamicImage,
) -> Result<PhaseCorrelationResult, String> {...}
    
pub struct PhaseCorrelationResult {
    pub translation_x: f64,
    pub translation_y: f64,
    pub cross_power_spectrum: Vec<Complex<f64>>,
}
```

## testing

1. From the example image, creates cropped and offset images at test_images/translated.
2. Runs registration and merging and outputs the registered metrics and merged image to test_images/registered.
```
cargo test
```

# TODO
- Make api struct based, with defaults
- add quality control metrics: Peak to secondary peak ratio (PSPR) and Peak Z-score
- add hanning window option for edge artifacts
- add sub pixel registration option
- [OpenCV uses 5x5 weighted centroids for sub-pixel accuracy](https://docs.opencv.org/4.x/d7/df3/group__imgproc__motion.html#ga552420a2ace9ef3fb053cd630fdb4952).
- Add option to calculate a registration error metric like RMSE.
