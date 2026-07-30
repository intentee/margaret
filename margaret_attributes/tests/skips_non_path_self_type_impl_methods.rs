use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn skips_non_path_self_type_impl_methods() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("valid_crate", &directory))
        .expect("the valid fixture indexes cleanly")
        .build();
    let has_tuple_method = index
        .items()
        .iter()
        .flat_map(margaret_attributes::indexed_item::IndexedItem::methods)
        .any(|method| method.identifier() == "skipped_tuple_method");

    assert!(!has_tuple_method);
}
