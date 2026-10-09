use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn rejects_a_crate_indexed_twice() {
    let root = CrateRoot::new(
        "second_crate",
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/second_crate"),
    );

    assert!(matches!(
        AttributeIndexBuilder::new()
            .index_crate(&root)
            .expect("the crate is indexed")
            .index_crate(&root)
            .err(),
        Some(AttributeError::DuplicateCanonicalPath { path }) if path == "second_crate::SecondStruct"
    ));
}
