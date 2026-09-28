use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;

#[test]
fn recognizes_framework_attributes_named_through_reexports_and_module_imports() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/reexported_items");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("reexported_items", &directory))
        .expect("the reexported items fixture indexes cleanly")
        .build();
    let singletons: Vec<&str> = index
        .select_framework_attribute(FrameworkAttribute::Singleton)
        .map(|matched| matched.item())
        .map(IndexedItem::identifier)
        .collect();

    assert_eq!(
        singletons,
        vec!["ModuleImportedMacroSingleton", "ReexportedMacroSingleton"]
    );
}
