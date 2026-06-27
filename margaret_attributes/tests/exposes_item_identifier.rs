use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn exposes_item_identifier() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("valid_crate", &directory))
        .expect("the valid fixture indexes cleanly")
        .build();
    let identifier = index
        .items()
        .iter()
        .find(|item| item.canonical_path().to_string() == "valid_crate::RootStruct")
        .expect("RootStruct is indexed")
        .identifier()
        .to_string();

    assert_eq!(identifier, "RootStruct");
}
