use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source;

#[test]
fn reports_constructor_return_type_mismatch() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/bad_return");
    let error = generate_container_source("bad_return", &directory, &[])
        .err()
        .expect("a #[constructor] that does not return Self must be rejected");

    assert!(matches!(
        error,
        ContainerError::ConstructorReturnTypeMismatch { .. }
    ));
}
