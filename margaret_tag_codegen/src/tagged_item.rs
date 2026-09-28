use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::tag::Tag;

pub struct TaggedItem<'index> {
    pub item: &'index IndexedItem,
    pub tag: Tag,
}
