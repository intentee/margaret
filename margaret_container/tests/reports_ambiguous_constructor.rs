use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source;

#[test]
fn reports_ambiguous_constructor() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ambiguous_constructor");
    let error = generate_container_source("ambiguous_constructor", &directory)
        .err()
        .expect("a singleton with two #[constructor] methods must be rejected");

    assert!(matches!(error, ContainerError::AmbiguousConstructor { .. }));
}
