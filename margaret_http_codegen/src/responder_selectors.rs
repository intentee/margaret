use margaret_attributes::attribute_selector::AttributeSelector;

pub(crate) struct ResponderSelectors {
    pub(crate) form_request: AttributeSelector,
    pub(crate) route_parameter: AttributeSelector,
}

impl ResponderSelectors {
    pub(crate) fn new() -> Self {
        Self {
            form_request: AttributeSelector::from_marker("form_request"),
            route_parameter: AttributeSelector::from_marker("route_parameter"),
        }
    }
}
