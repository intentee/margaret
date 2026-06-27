use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn reports_invalid_root_syntax() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/bad_syntax");
    let error = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("bad_syntax", &directory))
        .err()
        .expect("a crate root that does not parse must fail to index");

    assert!(matches!(error, AttributeError::FileParse { .. }));
}
