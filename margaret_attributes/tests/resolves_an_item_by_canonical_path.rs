use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn resolves_an_indexed_item_by_its_canonical_path() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/field_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("field_crate", &directory))
        .expect("the field fixture indexes cleanly")
        .build();

    let present = CanonicalPath::new(vec!["field_crate".to_string(), "Variants".to_string()]);
    assert_eq!(
        index
            .item(&present)
            .expect("the enum is resolvable by its canonical path")
            .identifier(),
        "Variants"
    );

    let absent = CanonicalPath::new(vec!["field_crate".to_string(), "Absent".to_string()]);
    assert!(index.item(&absent).is_none());
}
