use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn reports_missing_collection_provider() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/missing_collection_provider");
    let error = generate_container_source("missing_collection_provider", &directory)
        .expect_err("a collection dependency whose trait is not in scope must be rejected");

    assert!(matches!(error, ContainerError::MissingProvider { .. }));
}
