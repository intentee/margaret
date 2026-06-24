use std::path::Path;

use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn exposes_item_identifier() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");
    let identifier = index
        .holders()
        .iter()
        .filter_map(|holder| match holder {
            AttributeHolder::Item(item) => Some(item),
            AttributeHolder::Method(_) => None,
        })
        .find(|item| item.canonical_path().to_string() == "valid_crate::RootStruct")
        .expect("RootStruct is indexed")
        .identifier()
        .to_string();

    assert_eq!(identifier, "RootStruct");
}
