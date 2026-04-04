pub mod create_test_images;
pub mod test_config;
pub mod test_register;
#[test]
fn main_test() {
    create_test_images::create_test_images();
    test_register::test_register();
}
