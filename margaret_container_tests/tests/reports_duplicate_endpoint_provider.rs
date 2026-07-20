use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_duplicate_endpoint_provider() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/endpoint_provider_duplicate");
    let error = generate_container_source("endpoint_provider_duplicate", &directory)
        .expect_err("two singletons providing the same endpoint tag must be rejected");

    assert!(matches!(
        error,
        ContainerError::DuplicateEndpointProvider { .. }
    ));
}
