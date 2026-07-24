use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::struct_shape::StructShape;

#[test]
fn indexes_enum_variant_identifiers_and_shapes() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/field_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("field_crate", &directory))
        .expect("the field fixture indexes cleanly")
        .build();
    let item = |identifier: &str| -> &IndexedItem {
        index
            .items()
            .iter()
            .find(|item| item.identifier() == identifier)
            .expect("the item is indexed")
    };

    let variants = item("Variants");
    assert_eq!(variants.variants().len(), 3);
    assert_eq!(variants.variants()[0].identifier(), "Unit");
    assert_eq!(variants.variants()[0].shape(), StructShape::Unit);
    assert_eq!(variants.variants()[1].identifier(), "Tuple");
    assert_eq!(
        variants.variants()[1].shape(),
        StructShape::Unnamed { field_count: 2 }
    );
    assert_eq!(variants.variants()[2].identifier(), "Named");
    assert_eq!(
        variants.variants()[2].shape(),
        StructShape::Named { field_count: 1 }
    );

    assert!(item("NamedFields").variants().is_empty());
}
