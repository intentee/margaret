use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn excludes_the_named_crate_root_module() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/excluded_umbrella");

    let rejected = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("excluded_umbrella", &directory))
        .err()
        .expect("without exclusion the #[path] umbrella is rejected");

    assert!(matches!(
        rejected,
        AttributeError::ModulePathAttribute { .. }
    ));

    let index = AttributeIndexBuilder::new()
        .exclude_root_module("margaret")
        .index_crate(&CrateRoot::new("excluded_umbrella", &directory))
        .expect("excluding the umbrella lets the crate index cleanly")
        .build();
    let paths: Vec<String> = index
        .items()
        .iter()
        .map(|item| item.canonical_path().to_string())
        .collect();

    assert!(paths.contains(&"excluded_umbrella::Kept".to_string()));
    assert!(!paths.contains(&"excluded_umbrella::margaret".to_string()));
}
