use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::framework_attribute::FrameworkAttribute;

#[test]
fn selects_items_bearing_an_attribute() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("valid_crate", &directory))
        .expect("the valid fixture indexes cleanly")
        .build();
    let targets: Vec<String> = index
        .select_framework_attribute(FrameworkAttribute::Singleton)
        .map(|matched| matched.item().canonical_path().to_string())
        .collect();

    assert!(targets.contains(&"valid_crate::RootStruct".to_string()));
    assert!(targets.contains(&"valid_crate::WithProvides".to_string()));
    assert!(!targets.contains(&"valid_crate::RootTrait".to_string()));
}
