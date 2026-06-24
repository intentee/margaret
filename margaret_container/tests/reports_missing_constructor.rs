use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source;

#[test]
fn reports_missing_constructor() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/missing_constructor");
    let error = generate_container_source("missing_constructor", &directory, &[])
        .err()
        .expect("a singleton with no #[constructor] method must be rejected");

    assert!(matches!(error, ContainerError::MissingConstructor { .. }));
}
