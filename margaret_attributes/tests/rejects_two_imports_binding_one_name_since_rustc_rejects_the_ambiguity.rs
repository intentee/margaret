use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn rejects_two_imports_binding_one_name_since_rustc_rejects_the_ambiguity() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/conflicting_imports");
    let error = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("conflicting_imports", &directory))
        .err()
        .expect("two imports of one name must be rejected");

    assert!(matches!(
        error,
        AttributeError::DuplicateImport { ref module, ref name }
            if module == "conflicting_imports" && name == "Widget"
    ));
}
