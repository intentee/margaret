use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::item_kind::ItemKind;

#[test]
fn records_each_item_kind() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("valid_crate", &directory))
        .expect("the valid fixture indexes cleanly")
        .build();
    let kinds: Vec<ItemKind> = index
        .items()
        .iter()
        .map(margaret_attributes::indexed_item::IndexedItem::kind)
        .collect();

    assert!(kinds.iter().any(ItemKind::is_struct));
    assert!(kinds.contains(&ItemKind::Enum));
    assert!(kinds.contains(&ItemKind::Function));
    assert!(kinds.contains(&ItemKind::Trait));
    assert!(kinds.contains(&ItemKind::Module));
}
