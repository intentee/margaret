use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn drops_members_without_an_indexed_item() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");

    assert!(
        !index
            .items()
            .iter()
            .any(|item| item.canonical_path().to_string() == "valid_crate::Undeclared")
    );
    assert!(
        !index
            .items()
            .iter()
            .flat_map(|item| item.methods())
            .any(|method| method.identifier() == "orphaned_method")
    );
}
