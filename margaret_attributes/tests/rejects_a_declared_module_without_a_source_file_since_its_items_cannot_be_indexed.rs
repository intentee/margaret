use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn rejects_a_declared_module_without_a_source_file_since_its_items_cannot_be_indexed() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/module_missing");
    let error = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("module_missing", &directory))
        .err()
        .expect("a declared module with no backing source file must be rejected");

    assert!(matches!(error, AttributeError::ModuleFileNotFound { .. }));
}
