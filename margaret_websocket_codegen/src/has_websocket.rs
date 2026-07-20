use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

#[must_use]
pub fn has_websocket(index: &AttributeIndex) -> bool {
    index.has(&AttributeSelector::from_marker("websocket_transition"))
}
