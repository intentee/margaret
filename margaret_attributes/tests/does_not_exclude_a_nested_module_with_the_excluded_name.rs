use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn does_not_exclude_a_nested_module_with_the_excluded_name() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/nested_umbrella");
    let index = AttributeIndexBuilder::new()
        .exclude_root_module("margaret")
        .index_crate(&CrateRoot::new("nested_umbrella", &directory))
        .expect("a nested module sharing the excluded name still indexes")
        .build();
    let paths: Vec<String> = index
        .items()
        .iter()
        .map(|item| item.canonical_path().to_string())
        .collect();

    assert!(paths.contains(&"nested_umbrella::outer::margaret::Nested".to_string()));
}
