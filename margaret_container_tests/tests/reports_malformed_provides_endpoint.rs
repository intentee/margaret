use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_malformed_provides_endpoint() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provides_endpoint_malformed");
    let error = generate_container_source("provides_endpoint_malformed", &directory)
        .expect_err("a #[provides_endpoint] without exactly one tag must be rejected");

    assert!(matches!(
        error,
        ContainerError::MalformedProvidesEndpoint { .. }
    ));
}
