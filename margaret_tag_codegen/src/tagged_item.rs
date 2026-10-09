use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::tag::Tag;

use crate::tag_kind::TagKind;

pub struct TaggedItem<'index> {
    pub item: &'index IndexedItem,
    pub kind: TagKind,
    pub tag: Tag,
}
