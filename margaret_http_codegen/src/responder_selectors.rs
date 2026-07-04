use margaret_attributes::attribute_selector::AttributeSelector;

pub(crate) struct ResponderSelectors {
    pub(crate) form_request: AttributeSelector,
    pub(crate) route_parameter: AttributeSelector,
}

impl ResponderSelectors {
    pub(crate) fn new() -> Self {
        Self {
            form_request: AttributeSelector::parse("form_request").expect("a valid selector"),
            route_parameter: AttributeSelector::parse("route_parameter").expect("a valid selector"),
        }
    }
}
