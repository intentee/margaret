use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn reports_a_parse_failure_for_a_submodule_so_none_of_its_items_are_silently_missed() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/nested_bad_syntax");
    let error = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("nested_bad_syntax", &directory))
        .err()
        .expect("a submodule that does not parse must fail to index");

    assert!(matches!(error, AttributeError::FileParse { .. }));
}
