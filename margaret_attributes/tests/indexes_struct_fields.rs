use std::path::Path;

use syn::parse_quote;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::field_identifier::FieldIdentifier;
use margaret_attributes::indexed_item::IndexedItem;

#[test]
fn indexes_named_tuple_and_unit_struct_fields() {
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
            .expect("the struct is indexed")
    };

    let named = item("NamedFields");
    assert_eq!(named.fields().len(), 2);
    assert_eq!(
        named.fields()[0].identifier(),
        &FieldIdentifier::Named(parse_quote!(first))
    );
    assert_eq!(named.fields()[0].attributes().len(), 1);
    assert_eq!(
        named.fields()[1].identifier(),
        &FieldIdentifier::Named(parse_quote!(second))
    );
    assert!(named.fields()[1].attributes().is_empty());

    let tuple = item("TupleFields");
    assert_eq!(
        tuple.fields()[0].identifier(),
        &FieldIdentifier::Positional(0)
    );
    assert_eq!(
        tuple.fields()[1].identifier(),
        &FieldIdentifier::Positional(1)
    );

    assert!(item("UnitStruct").fields().is_empty());
}
