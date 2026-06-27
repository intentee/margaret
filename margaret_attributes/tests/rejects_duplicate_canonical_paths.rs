use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn rejects_duplicate_canonical_paths() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate");
    let error = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("duplicate", &directory))
        .err()
        .expect("two items sharing a canonical path must be rejected as ambiguous");

    assert!(matches!(
        error,
        AttributeError::DuplicateCanonicalPath { .. }
    ));
}
