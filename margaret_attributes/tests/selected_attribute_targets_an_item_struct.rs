use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::framework_attribute::FrameworkAttribute;

#[test]
fn canonical_framework_attributes_target_item_structs() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("valid_crate", &directory))
        .expect("the valid fixture indexes cleanly")
        .build();
    let targets: Vec<String> = index
        .select_framework_attribute(FrameworkAttribute::Singleton)
        .iter()
        .map(|matched| matched.item().canonical_path().to_string())
        .collect();

    assert!(targets.contains(&"valid_crate::AliasedSingleton".to_string()));
    assert!(!targets.contains(&"valid_crate::UnrelatedSingleton".to_string()));

    let qualified = index
        .items()
        .iter()
        .find(|item| item.canonical_path().to_string() == "valid_crate::AliasedSingleton")
        .expect("the aliased singleton is indexed");

    assert!(qualified.kind().is_struct());
    assert_eq!(
        qualified.canonical_path().segments(),
        ["valid_crate".to_string(), "AliasedSingleton".to_string()]
    );
}
