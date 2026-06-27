use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn selects_one_entry_per_repeated_attribute_occurrence() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("valid_crate", &directory))
        .expect("the valid fixture indexes cleanly")
        .build();
    let selector = AttributeSelector::parse("singleton").expect("the selector parses");
    let occurrences = index
        .select(&selector)
        .iter()
        .filter(|matched| {
            matched.item().canonical_path().to_string() == "valid_crate::RepeatedAttrs"
        })
        .count();

    assert_eq!(occurrences, 2);
}
