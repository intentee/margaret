use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn reports_provider_missing_dependency() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provider_missing_dependency");
    let error = generate_container_source("provider_missing_dependency", &directory)
        .err()
        .expect("a provider whose constructor dependency is unprovided must be rejected");

    assert!(matches!(error, ContainerError::MissingProvider { .. }));
}
