use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::framework_attribute::FrameworkAttribute;

#[test]
fn canonicalizes_method_attributes_at_the_impl_site() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("valid_crate", &directory))
        .expect("the valid fixture indexes cleanly")
        .build();
    let owner = index
        .items()
        .iter()
        .find(|item| item.canonical_path().to_string() == "valid_crate::WithAliasedConstructor")
        .expect("the constructor owner is indexed");
    let constructor = owner
        .methods()
        .iter()
        .find(|method| method.identifier() == "create")
        .expect("the constructor declared in another module is indexed");

    assert!(
        constructor.has_framework_attribute(FrameworkAttribute::Constructor),
        "the alias imported where the impl is declared resolves to #[constructor]",
    );
}
