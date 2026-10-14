use std::path::Path;

use syn::parse_quote;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn resolves_same_module_constants_and_statics_so_attribute_arguments_need_no_import() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/local_values");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("local_values", &directory))
        .expect("the local values fixture indexes cleanly")
        .build();
    let scheduled = index
        .items()
        .iter()
        .find(|item| item.canonical_path().to_string() == "local_values::Scheduled")
        .expect("the scheduled item is indexed");

    assert_eq!(
        index
            .resolve_item_path(scheduled, &parse_quote!(PERIOD))
            .map(|resolved| resolved.to_string()),
        Some("local_values::PERIOD".to_string())
    );
    assert_eq!(
        index
            .resolve_item_path(scheduled, &parse_quote!(LIMIT))
            .map(|resolved| resolved.to_string()),
        Some("local_values::LIMIT".to_string())
    );
}
