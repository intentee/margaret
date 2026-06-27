use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn items_are_sorted_by_canonical_path() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("valid_crate", &directory))
        .expect("the valid fixture indexes cleanly")
        .build();
    let paths: Vec<String> = index
        .items()
        .iter()
        .map(|item| item.canonical_path().to_string())
        .collect();

    let mut sorted = paths.clone();
    sorted.sort();

    assert_eq!(paths, sorted);
}
