use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_constructor_without_return_type() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/constructor_without_return");
    let error = generate_container_source("constructor_without_return", &directory)
        .expect_err("a #[constructor] without a return type must be rejected");

    assert!(matches!(
        error,
        ContainerError::ConstructorReturnTypeMismatch { .. }
    ));
}
