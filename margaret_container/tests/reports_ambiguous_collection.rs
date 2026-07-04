use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn reports_ambiguous_collection() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/collection_ambiguous");
    let error = generate_container_source("collection_ambiguous", &directory)
        .expect_err("a collection trait matching more than one trait must be rejected");

    assert!(matches!(error, ContainerError::CollectionAmbiguous { .. }));
}
