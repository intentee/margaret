use std::path::Path;

use syn::parse_quote;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::resolution::Resolution;

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

    let resolution = index.struct_resolution();
    let resolved = |written: syn::Path| match resolution.resolve(&written, "associated_types") {
        Resolution::Resolved(path) => path.to_string(),
        Resolution::NotFound => "<not found>".to_string(),
        Resolution::Ambiguous(_) => "<ambiguous>".to_string(),
    };

    assert_eq!(resolved(parse_quote!(Model)), "associated_types::Model");
    assert_eq!(
        resolved(parse_quote!(Producer)),
        "associated_types::Producer"
    );
}
