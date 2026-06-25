use std::path::Path;

use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::item_kind::ItemKind;

#[test]
fn records_each_item_kind() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_crate");
    let index = AttributeIndex::from_crate_root("valid_crate", &directory)
        .expect("the valid fixture indexes cleanly");
    let kinds: Vec<ItemKind> = index
        .holders()
        .iter()
        .filter_map(|holder| match holder {
            AttributeHolder::Item(item) => Some(item.kind()),
            AttributeHolder::Method(_) => None,
        })
        .collect();

    assert!(kinds.iter().any(ItemKind::is_struct));
    assert!(kinds.contains(&ItemKind::Enum));
    assert!(kinds.contains(&ItemKind::Function));
    assert!(kinds.contains(&ItemKind::Trait));
    assert!(kinds.contains(&ItemKind::Module));
}
