use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source;

#[test]
fn propagates_index_failure() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/index_failure");
    let error = generate_container_source("index_failure", &directory, &[])
        .err()
        .expect("an indexing failure must surface through container generation");

    assert!(matches!(
        error,
        ContainerError::Index {
            source: AttributeError::GlobImport { .. }
        }
    ));
}
