use margaret_attributes::attribute_selector::AttributeSelector;

pub(crate) fn console_argument_selector() -> AttributeSelector {
    AttributeSelector::parse("console_argument").expect("a valid selector")
}
