use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn rejects_glob_imports() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/glob");
    let error = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("glob", &directory))
        .err()
        .expect("a glob import hides identity and must be rejected");

    assert!(matches!(error, AttributeError::GlobImport { .. }));
}
