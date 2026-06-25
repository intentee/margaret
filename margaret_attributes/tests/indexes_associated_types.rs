use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn indexes_associated_types() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/associated_types");
    let index = AttributeIndex::from_crate_root("associated_types", &directory)
        .expect("the associated types fixture indexes cleanly");

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

    let struct_paths: Vec<String> = index
        .struct_paths()
        .iter()
        .map(|path| path.to_string())
        .collect();

    assert!(struct_paths.contains(&"associated_types::Model".to_string()));
    assert!(struct_paths.contains(&"associated_types::Producer".to_string()));
}
