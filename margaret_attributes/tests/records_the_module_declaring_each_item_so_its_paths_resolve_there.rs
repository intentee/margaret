use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn records_the_module_declaring_each_item_so_its_paths_resolve_there() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("valid_crate", &directory))
        .expect("the valid fixture indexes cleanly")
        .build();
    let inside_inline = index
        .item(&CanonicalPath::new(vec![
            "valid_crate".to_string(),
            "inline_module".to_string(),
            "InsideInline".to_string(),
        ]))
        .expect("InsideInline is indexed");

    assert_eq!(
        inside_inline.module_path(),
        ["valid_crate".to_string(), "inline_module".to_string()]
    );
}
