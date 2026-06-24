use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn rejects_duplicate_canonical_paths() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate");
    let error = AttributeIndex::from_crate_root("duplicate", &directory)
        .err()
        .expect("two items sharing a canonical path must be rejected as ambiguous");

    assert!(matches!(
        error,
        AttributeError::DuplicateCanonicalPath { .. }
    ));
}
