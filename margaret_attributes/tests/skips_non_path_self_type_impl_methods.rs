use std::path::Path;

use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn skips_non_path_self_type_impl_methods() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");
    let has_tuple_method = index
        .holders()
        .iter()
        .filter_map(|holder| match holder {
            AttributeHolder::Method(method) => Some(method),
            AttributeHolder::Item(_) => None,
        })
        .any(|method| method.identifier() == "skipped_tuple_method");

    assert!(!has_tuple_method);
}
