use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_endpoint_provider_not_concrete() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/endpoint_provider_not_concrete");
    let error = generate_container_source("endpoint_provider_not_concrete", &directory)
        .expect_err("an endpoint provider that also declares provides must be rejected");

    assert!(matches!(
        error,
        ContainerError::EndpointProviderNotConcrete { .. }
    ));
}
