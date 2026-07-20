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

    let trait_impls = producer.trait_impls();

    assert_eq!(trait_impls.len(), 1);

    let produces = &trait_impls[0];

    assert_eq!(produces.module_path().join("::"), "associated_types");
    assert_eq!(
        index
            .resolve_module_path(produces.module_path(), produces.trait_path())
            .expect("the trait path resolves at the impl site")
            .to_string(),
        "associated_types::Produces"
    );

    let names: Vec<&str> = produces
        .associated_types()
        .iter()
        .map(|associated_type| associated_type.name())
        .collect();

    assert_eq!(names, ["Extra", "Output"]);
    assert!(produces.associated_type("Missing").is_none());

    let output = produces
        .associated_type("Output")
        .expect("the Output associated type is indexed");
    let model = index
        .resolve_module_type(produces.module_path(), output.ty())
        .expect("the associated type resolves at the impl site");

    assert_eq!(model.to_string(), "associated_types::Model");
    assert!(index.struct_identifier(&model).is_some());

    let same_module = index
        .resolve_item_path(producer, &parse_quote!(Model))
        .expect("a same-module reference resolves");

    assert_eq!(same_module.to_string(), "associated_types::Model");
    assert!(
        index
            .struct_identifier(&CanonicalPath::new(vec![
                "associated_types".to_string(),
                "Producer".to_string(),
            ]))
            .is_some()
    );
}
