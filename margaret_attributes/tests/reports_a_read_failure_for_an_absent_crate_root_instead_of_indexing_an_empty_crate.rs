use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn reports_a_read_failure_for_an_absent_crate_root_instead_of_indexing_an_empty_crate() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/does_not_exist");
    let error = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("ghost_crate", &directory))
        .err()
        .expect("indexing a crate whose root file is absent must fail");

    assert!(matches!(error, AttributeError::FileRead { .. }));
}
