use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn reports_a_derive_list_that_cannot_be_read() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/malformed_derive");
    let error = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .err()
        .expect("indexing a crate with an unreadable derive list must fail");

    assert!(matches!(error, AttributeError::Arguments(_)));
}
