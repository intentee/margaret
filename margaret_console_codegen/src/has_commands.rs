use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

pub fn has_commands(index: &AttributeIndex) -> bool {
    let selector = AttributeSelector::parse("console_command").expect("a valid selector");

    !index.select(&selector).is_empty()
}
