use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn reports_singleton_requires_constructor() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/missing_constructor");
    let error = generate_container_source("missing_constructor", &directory)
        .expect_err("a singleton with fields but no #[constructor] method must be rejected");

    assert!(matches!(
        error,
        ContainerError::SingletonRequiresConstructor { field_count: 1, .. }
    ));
}
