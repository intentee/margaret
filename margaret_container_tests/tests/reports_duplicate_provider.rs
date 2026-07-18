use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_duplicate_provider() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate_provider");
    let error = generate_container_source("duplicate_provider", &directory)
        .expect_err("two singletons providing the same type must be rejected");

    assert!(matches!(error, ContainerError::DuplicateProvider { .. }));
}
