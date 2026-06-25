use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

#[test]
fn selects_items_bearing_an_attribute() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");
    let selector = AttributeSelector::parse("singleton").expect("the selector parses");
    let targets: Vec<String> = index
        .select(&selector)
        .iter()
        .map(|matched| matched.item().canonical_path().to_string())
        .collect();

    assert!(targets.contains(&"valid_crate::RootStruct".to_string()));
    assert!(targets.contains(&"valid_crate::WithProvides".to_string()));
    assert!(!targets.contains(&"valid_crate::RootTrait".to_string()));
}
