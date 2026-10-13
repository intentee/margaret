use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::tag::Tag;
use margaret_registered_claims::audience::Audience;

pub struct DeclaredTrust<'index> {
    pub anchor: &'index IndexedItem,
    pub audience: Audience,
    pub tag: Tag,
}
