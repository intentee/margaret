use std::path::Path;

use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn indexes_inherent_impl_constructor_method() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");
    let method = index
        .holders()
        .iter()
        .filter_map(|holder| match holder {
            AttributeHolder::Method(method) => Some(method),
            AttributeHolder::Item(_) => None,
        })
        .find(|method| {
            method.self_type_path().to_string() == "valid_crate::WithConstructor"
                && method.identifier() == "new"
        })
        .expect("the constructor method is indexed");

    assert!(
        method
            .attributes()
            .iter()
            .any(|attribute| attribute.path().is_ident("constructor"))
    );
    assert_eq!(method.signature().inputs.len(), 1);
}
