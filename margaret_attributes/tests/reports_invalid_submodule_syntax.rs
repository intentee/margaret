use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn reports_invalid_submodule_syntax() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/nested_bad_syntax");
    let error = AttributeIndex::from_crate_root("nested_bad_syntax", &directory)
        .err()
        .expect("a submodule that does not parse must fail to index");

    assert!(matches!(error, AttributeError::FileParse { .. }));
}
