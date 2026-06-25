use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn propagates_non_path_collection_argument() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/collection_not_a_path");
    let error = generate_container_source("collection_not_a_path", &directory)
        .err()
        .expect("a non-path collection argument must surface through container generation");

    assert!(matches!(
        error,
        ContainerError::Index {
            source: AttributeError::UnexpectedArgument { .. }
        }
    ));
}
