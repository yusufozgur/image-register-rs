pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
pub struct TestConfig {
    pub crop_width: u32,
    pub crop_height: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub source: String,
    pub left_crop: String,
    pub right_crop: String,
    pub registered_ground_truth: String,
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
        }
    }
}
