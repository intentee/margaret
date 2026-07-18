use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_missing_provider() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/missing_provider");
    let error = generate_container_source("missing_provider", &directory)
        .expect_err("a dependency with no providing singleton must be rejected");

    assert!(matches!(error, ContainerError::MissingProvider { .. }));
}
