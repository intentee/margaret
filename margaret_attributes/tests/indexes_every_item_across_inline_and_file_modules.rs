use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn indexes_every_item_across_inline_and_file_modules() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("valid_crate", &directory))
        .expect("the valid fixture indexes cleanly")
        .build();
    let paths: Vec<String> = index
        .items()
        .iter()
        .map(|item| item.canonical_path().to_string())
        .collect();

    assert!(paths.contains(&"valid_crate::RootStruct".to_string()));
    assert!(paths.contains(&"valid_crate::inline_module::InsideInline".to_string()));
    assert!(paths.contains(&"valid_crate::file_module::InFileModule".to_string()));
    assert!(paths.contains(&"valid_crate::file_module::nested::DeeplyNested".to_string()));
    assert!(paths.contains(&"valid_crate::dir_module::InDirModule".to_string()));
}
