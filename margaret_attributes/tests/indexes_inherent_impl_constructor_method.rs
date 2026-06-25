use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn indexes_inherent_impl_constructor_method() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");
    let owner = index
        .items()
        .iter()
        .find(|item| item.canonical_path().to_string() == "valid_crate::WithConstructor")
        .expect("the constructor owner is indexed");
    let method = owner
        .methods()
        .iter()
        .find(|method| method.identifier() == "new")
        .expect("the constructor method is owned by its struct");

    assert!(
        method
            .attributes()
            .iter()
            .any(|attribute| attribute.path().is_ident("constructor"))
    );
    assert_eq!(method.signature().inputs.len(), 1);
}
