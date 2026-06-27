use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn skips_trait_impl_methods() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("valid_crate", &directory))
        .expect("the valid fixture indexes cleanly")
        .build();
    let has_trait_method = index
        .items()
        .iter()
        .flat_map(|item| item.methods())
        .any(|method| method.identifier() == "skipped_trait_method");

    assert!(!has_trait_method);
}
