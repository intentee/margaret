use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn rejects_path_attribute_modules() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/path_attr");
    let error = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("path_attr", &directory))
        .err()
        .expect("a #[path] module redirects file resolution and must be rejected");

    assert!(matches!(error, AttributeError::ModulePathAttribute { .. }));
}
