use margaret_attributes::indexed_item::IndexedItem;

use crate::build_plan::singleton_selector;

#[must_use]
pub fn is_singleton(item: &IndexedItem) -> bool {
    item.has_attribute(&singleton_selector())
}
