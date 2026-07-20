use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_bad_provides_endpoint_arguments() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provides_endpoint_bad_args");
    let error = generate_container_source("provides_endpoint_bad_args", &directory)
        .expect_err("malformed #[provides_endpoint(...)] arguments must be rejected");

    assert!(matches!(error, ContainerError::Index { .. }));
}
