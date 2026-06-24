use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn reports_missing_crate_root() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/does_not_exist");
    let error = AttributeIndex::from_crate_root("ghost_crate", &directory)
        .err()
        .expect("indexing a crate whose root file is absent must fail");

    assert!(matches!(error, AttributeError::FileRead { .. }));
}
