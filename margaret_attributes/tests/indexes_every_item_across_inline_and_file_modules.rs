use std::path::Path;

use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;

#[test]
fn indexes_every_item_across_inline_and_file_modules() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");
    let paths: Vec<String> = index
        .holders()
        .iter()
        .filter_map(|holder| match holder {
            AttributeHolder::Item(item) => Some(item.canonical_path().to_string()),
            AttributeHolder::Method(_) => None,
        })
        .collect();

    assert!(paths.contains(&"valid_crate::RootStruct".to_string()));
    assert!(paths.contains(&"valid_crate::inline_module::InsideInline".to_string()));
    assert!(paths.contains(&"valid_crate::file_module::InFileModule".to_string()));
    assert!(paths.contains(&"valid_crate::file_module::nested::DeeplyNested".to_string()));
    assert!(paths.contains(&"valid_crate::dir_module::InDirModule".to_string()));
}
