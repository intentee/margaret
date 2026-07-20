use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_malformed_endpoint_provider() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/endpoint_provider_malformed");
    let error = generate_container_source("endpoint_provider_malformed", &directory)
        .expect_err("an #[endpoint_provider] without exactly one tag must be rejected");

    assert!(matches!(
        error,
        ContainerError::MalformedEndpointProvider { .. }
    ));
}
