use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_bad_endpoint_provider_arguments() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/endpoint_provider_bad_args");
    let error = generate_container_source("endpoint_provider_bad_args", &directory)
        .expect_err("malformed #[endpoint_provider(...)] arguments must be rejected");

    assert!(matches!(error, ContainerError::Index { .. }));
}
