use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_constructor_return_type_mismatch() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/bad_return");
    let error = generate_container_source("bad_return", &directory)
        .expect_err("a #[constructor] that does not return Self must be rejected");

    assert!(matches!(
        error,
        ContainerError::ConstructorReturnTypeMismatch { .. }
    ));
}

#[test]
fn rejects_a_constructor_returning_a_non_anyhow_result() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/non_anyhow_result_return");
    let error = generate_container_source("non_anyhow_result_return", &directory)
        .expect_err("a #[constructor] returning a non-anyhow Result must be rejected");

    assert!(matches!(
        error,
        ContainerError::ConstructorReturnTypeMismatch { .. }
    ));
}
