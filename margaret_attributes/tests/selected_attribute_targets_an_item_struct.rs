use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

#[test]
fn selected_attribute_targets_an_item_struct() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");
    let selector = AttributeSelector::parse("ns::tagged").expect("the selector parses");
    let targets: Vec<String> = index
        .select(&selector)
        .iter()
        .map(|matched| matched.item().canonical_path().to_string())
        .collect();

    assert!(targets.contains(&"valid_crate::Qualified".to_string()));

    let qualified = index
        .items()
        .iter()
        .find(|item| item.canonical_path().to_string() == "valid_crate::Qualified")
        .expect("the qualified item is indexed");

    assert!(qualified.kind().is_struct());
    assert_eq!(
        qualified.canonical_path().segments(),
        ["valid_crate".to_string(), "Qualified".to_string()]
    );
}
