use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn rejects_ambiguous_module_files() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/module_collision");
    let error = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("module_collision", &directory))
        .err()
        .expect("a module resolvable to both foo.rs and foo/mod.rs must be rejected");

    assert!(matches!(error, AttributeError::ModuleFileCollision { .. }));
}
