use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_a_constructor_taking_self_because_no_instance_exists_before_construction() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/receiver_param");
    let error = generate_container_source("receiver_param", &directory)
        .expect_err("a constructor with a self receiver must be rejected");

    assert!(matches!(
        error,
        ContainerError::UnsupportedParameterShape { .. }
    ));
}
