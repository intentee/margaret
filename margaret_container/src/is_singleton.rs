use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;

#[must_use]
pub fn is_singleton(item: &IndexedItem) -> bool {
    item.has_framework_attribute(FrameworkAttribute::Singleton)
}
