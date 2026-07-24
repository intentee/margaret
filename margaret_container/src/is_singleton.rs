use margaret_attributes::indexed_item::IndexedItem;

use crate::build_plan::has_marker;
use crate::build_plan::singleton_selector;

#[must_use]
pub fn is_singleton(item: &IndexedItem) -> bool {
    has_marker(item, &singleton_selector())
}
