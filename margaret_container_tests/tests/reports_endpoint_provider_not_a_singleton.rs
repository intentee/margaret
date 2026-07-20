use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_endpoint_provider_not_a_singleton() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/provides_endpoint_not_a_singleton");
    let error = generate_container_source("provides_endpoint_not_a_singleton", &directory)
        .expect_err("#[provides_endpoint] on a non-singleton must be rejected");

    assert!(matches!(
        error,
        ContainerError::EndpointProviderNotASingleton { .. }
    ));
}
