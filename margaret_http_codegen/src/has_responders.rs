use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

pub fn has_responders(index: &AttributeIndex) -> bool {
    index.has(&AttributeSelector::parse("responds_to_http").expect("a valid selector"))
}
