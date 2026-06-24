use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source;

#[test]
fn reports_ambiguous_reference() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ambiguous_reference");
    let error = generate_container_source("ambiguous_reference", &directory, &[])
        .err()
        .expect("a dependency matching more than one provider must be rejected");

    assert!(matches!(error, ContainerError::AmbiguousReference { .. }));
}
