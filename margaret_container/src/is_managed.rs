use margaret_attributes::indexed_item::IndexedItem;

use crate::build_plan::has_marker;
use crate::build_plan::managed_selectors;
use crate::build_plan::provides_endpoint_selector;

#[must_use]
pub fn is_managed(item: &IndexedItem) -> bool {
    managed_selectors()
        .iter()
        .any(|selector| has_marker(item, selector))
        || has_marker(item, &provides_endpoint_selector())
}
