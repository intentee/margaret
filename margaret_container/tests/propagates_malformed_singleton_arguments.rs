use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source;

#[test]
fn propagates_malformed_singleton_arguments() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/malformed_singleton_args");
    let error = generate_container_source("malformed_singleton_args", &directory)
        .err()
        .expect("malformed #[singleton] arguments must surface through container generation");

    assert!(matches!(
        error,
        ContainerError::Index {
            source: AttributeError::AttributeArguments { .. }
        }
    ));
}
