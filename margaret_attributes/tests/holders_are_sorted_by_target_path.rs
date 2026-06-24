use std::path::Path;

use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn holders_are_sorted_by_target_path() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");
    let targets: Vec<String> = index
        .holders()
        .iter()
        .map(AttributeHolder::target_path)
        .collect();

    let mut sorted = targets.clone();
    sorted.sort();

    assert_eq!(targets, sorted);
}
