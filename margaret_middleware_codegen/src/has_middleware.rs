use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

#[must_use]
pub fn has_middleware(index: &AttributeIndex) -> bool {
    index.has(&AttributeSelector::from_marker(
        "handles_middleware_attribute",
    ))
}
