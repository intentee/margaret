use margaret_attributes::identifier::Identifier;
use margaret_attributes::indexed_item::IndexedItem;

pub struct DeclaredAnchor<'index> {
    pub identifier: &'index Identifier,
    pub item: &'index IndexedItem,
}
