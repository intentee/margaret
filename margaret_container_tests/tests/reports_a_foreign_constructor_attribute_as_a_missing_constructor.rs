use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_a_foreign_constructor_attribute_as_a_missing_constructor() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foreign_constructor_attribute");
    let error = generate_container_source("foreign_constructor_attribute", &directory)
        .expect_err("an attribute that is not #[constructor] must not mark a constructor");

    assert!(matches!(
        error,
        ContainerError::SingletonRequiresConstructor { field_count: 1, .. }
    ));
}
