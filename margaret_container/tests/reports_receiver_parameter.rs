use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn reports_receiver_parameter() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/receiver_param");
    let error = generate_container_source("receiver_param", &directory)
        .err()
        .expect("a constructor with a self receiver must be rejected");

    assert!(matches!(
        error,
        ContainerError::UnsupportedParameterShape { .. }
    ));
}
