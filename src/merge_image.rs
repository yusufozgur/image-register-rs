use image::DynamicImage;
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
