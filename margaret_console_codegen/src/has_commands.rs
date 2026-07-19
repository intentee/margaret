use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

#[must_use]
pub fn has_commands(index: &AttributeIndex) -> bool {
    index.has(&AttributeSelector::parse("console_command").expect("a valid selector"))
}
