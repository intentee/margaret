use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn rejects_ambiguous_module_files() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/module_collision");
    let error = AttributeIndex::from_crate_root("module_collision", &directory)
        .err()
        .expect("a module resolvable to both foo.rs and foo/mod.rs must be rejected");

    assert!(matches!(error, AttributeError::ModuleFileCollision { .. }));
}
