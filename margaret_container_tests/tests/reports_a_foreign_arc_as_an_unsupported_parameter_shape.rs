use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_a_foreign_arc_as_an_unsupported_parameter_shape() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foreign_arc");
    let error = generate_container_source("foreign_arc", &directory)
        .expect_err("a pointer type that is not std::sync::Arc or std::rc::Rc must be rejected");

    assert!(matches!(
        error,
        ContainerError::UnsupportedParameterShape { .. }
    ));
}
