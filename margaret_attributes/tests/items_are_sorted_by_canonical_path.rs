use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn items_are_sorted_by_canonical_path() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");
    let paths: Vec<String> = index
        .items()
        .iter()
        .map(|item| item.canonical_path().to_string())
        .collect();

    let mut sorted = paths.clone();
    sorted.sort();

    assert_eq!(paths, sorted);
}
