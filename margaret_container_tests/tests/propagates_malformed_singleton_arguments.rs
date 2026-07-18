use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn propagates_malformed_singleton_arguments() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/malformed_singleton_args");
    let error = generate_container_source("malformed_singleton_args", &directory)
        .expect_err("malformed #[singleton] arguments must surface through container generation");

    assert!(matches!(
        error,
        ContainerError::Index {
            source: AttributeError::AttributeArguments { .. }
        }
    ));
}
