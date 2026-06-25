use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

pub fn has_responders(index: &AttributeIndex) -> bool {
    let selector = AttributeSelector::parse("responds_to_http").expect("a valid selector");

    !index.select(&selector).is_empty()
}
