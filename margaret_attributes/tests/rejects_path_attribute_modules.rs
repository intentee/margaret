use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn rejects_path_attribute_modules() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/path_attr");
    let error = AttributeIndex::from_crate_root("path_attr", &directory)
        .err()
        .expect("a #[path] module redirects file resolution and must be rejected");

    assert!(matches!(error, AttributeError::ModulePathAttribute { .. }));
}
