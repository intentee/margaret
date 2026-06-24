use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source;

#[test]
fn reports_duplicate_field_name() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate_field");
    let error = generate_container_source("duplicate_field", &directory)
        .err()
        .expect("two providers resolving to the same container field must be rejected");

    assert!(matches!(error, ContainerError::DuplicateFieldName { .. }));
}
