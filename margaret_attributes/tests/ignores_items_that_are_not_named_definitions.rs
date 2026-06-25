use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn ignores_items_that_are_not_named_definitions() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");
    let paths: Vec<String> = index
        .items()
        .iter()
        .map(|item| item.canonical_path().to_string())
        .collect();

    assert!(!paths.contains(&"valid_crate::ROOT_CONST".to_string()));
    assert!(!paths.contains(&"valid_crate::RootAlias".to_string()));
}
