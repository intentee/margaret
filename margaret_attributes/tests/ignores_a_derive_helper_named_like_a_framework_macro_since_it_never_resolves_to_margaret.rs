use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn ignores_a_derive_helper_named_like_a_framework_macro_since_it_never_resolves_to_margaret() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/derive_helper_named_like_a_macro");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new(
            "derive_helper_named_like_a_macro",
            &directory,
        ))
        .expect("the derive helper fixture indexes cleanly")
        .build();
    let note = index
        .item(&CanonicalPath::new(vec![
            "derive_helper_named_like_a_macro".to_string(),
            "Note".to_string(),
        ]))
        .expect("Note is indexed");
    let model_helper = note
        .attributes()
        .iter()
        .find(|attribute| attribute.path().is_ident("model"))
        .expect("the derive helper is indexed");

    assert_eq!(model_helper.framework_attribute(), None);
}
