use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn reports_missing_module_files() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/module_missing");
    let error = AttributeIndex::from_crate_root("module_missing", &directory)
        .err()
        .expect("a declared module with no backing source file must be rejected");

    assert!(matches!(error, AttributeError::ModuleFileNotFound { .. }));
}
