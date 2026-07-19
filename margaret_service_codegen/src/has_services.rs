use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

#[must_use]
pub fn has_services(index: &AttributeIndex) -> bool {
    let service = AttributeSelector::from_marker("service");
    let ticker = AttributeSelector::from_marker("scheduled_with_tick_timer");

    index.has(&service) || index.has(&ticker)
}
