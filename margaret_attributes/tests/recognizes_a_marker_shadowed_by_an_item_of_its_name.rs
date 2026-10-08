use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_attribute::IndexedAttribute;
use margaret_attributes::indexed_field::IndexedField;

#[test]
fn recognizes_a_marker_shadowed_by_an_item_of_its_name() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/shadowed_marker");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("shadowed_marker", &directory))
        .expect("the fixture indexes cleanly")
        .build();
    let note = index
        .item(&CanonicalPath::new(vec![
            "shadowed_marker".to_string(),
            "Note".to_string(),
        ]))
        .expect("the model is indexed");

    assert_eq!(
        note.fields()
            .iter()
            .flat_map(IndexedField::attributes)
            .map(IndexedAttribute::framework_attribute)
            .collect::<Vec<_>>(),
        vec![Some(FrameworkAttribute::Index)]
    );
}
