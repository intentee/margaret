use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_missing_endpoint_provider() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/endpoint_provider_missing");
    let error = generate_container_source("endpoint_provider_missing", &directory)
        .expect_err("an endpoint tag with no provider must be rejected");

    assert!(matches!(
        error,
        ContainerError::MissingEndpointProvider { .. }
    ));
}
