use std::path::Path;

use syn::parse_quote;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn resolves_an_item_imported_through_a_reexport_to_its_definition() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/reexported_items");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("reexported_items", &directory))
        .expect("the reexported items fixture indexes cleanly")
        .build();
    let routes = ["reexported_items".to_string(), "routes".to_string()];

    assert_eq!(
        index.resolve_module_path(&routes, &parse_quote!(User)),
        Some(CanonicalPath::new(vec![
            "reexported_items".to_string(),
            "models".to_string(),
            "user".to_string(),
            "User".to_string(),
        ]))
    );
}
