use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_a_borrowed_constructor_parameter_as_a_shape_the_container_cannot_inject() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/unsupported_param");
    let error = generate_container_source("unsupported_param", &directory)
        .expect_err("a constructor parameter outside the wrapper allowlist must be rejected");

    assert!(matches!(
        error,
        ContainerError::UnsupportedParameterShape { .. }
    ));
}
