use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

#[must_use]
pub fn has_authenticated_users(index: &AttributeIndex) -> bool {
    index.has(&AttributeSelector::from_marker("infers_authenticated_user"))
}
