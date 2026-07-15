use std::path::Path;

use syn::parse_quote;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn indexes_associated_types() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/associated_types");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("associated_types", &directory))
        .expect("the associated types fixture indexes cleanly")
        .build();

    let producer = index
        .items()
        .iter()
        .find(|item| item.canonical_path().to_string() == "associated_types::Producer")
        .expect("the producer struct is indexed");
    let names: Vec<&str> = producer
        .associated_types()
        .iter()
        .map(|associated_type| associated_type.name())
        .collect();

    assert_eq!(names, ["Extra", "Output"]);

    let model = index
        .resolve_item_path(producer, &parse_quote!(Model))
        .expect("a same-module reference resolves");

    assert_eq!(model.to_string(), "associated_types::Model");
    assert!(index.is_indexed_struct(&model));
    assert!(index.is_indexed_struct(&CanonicalPath::new(vec![
        "associated_types".to_string(),
        "Producer".to_string(),
    ])));
}
