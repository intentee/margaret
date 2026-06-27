use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn rejects_glob_imports_inside_inline_modules() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inline_glob");
    let error = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("inline_glob", &directory))
        .err()
        .expect("a glob import nested in an inline module must also be rejected");

    assert!(matches!(error, AttributeError::GlobImport { .. }));
}
