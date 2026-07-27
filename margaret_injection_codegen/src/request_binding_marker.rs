use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_attribute::IndexedAttribute;

const REQUEST_BINDING_MARKERS: [&str; 3] =
    ["authenticated_user", "form_request", "route_parameter"];

#[must_use]
pub fn request_binding_marker(attributes: &[IndexedAttribute]) -> Option<&'static str> {
    REQUEST_BINDING_MARKERS.into_iter().find(|name| {
        let selector = AttributeSelector::from_marker(name);

        attributes
            .iter()
            .any(|attribute| selector.matches(attribute.path()))
    })
}

#[cfg(test)]
mod tests {
    use margaret_attributes::indexed_attribute::IndexedAttribute;
    use syn::parse_quote;

    use super::request_binding_marker;

    #[test]
    fn finds_the_authenticated_user_marker() {
        let attributes = vec![IndexedAttribute::new(parse_quote!(#[authenticated_user]))];

        assert_eq!(
            request_binding_marker(&attributes),
            Some("authenticated_user")
        );
    }

    #[test]
    fn finds_the_form_request_marker() {
        let attributes = vec![IndexedAttribute::new(
            parse_quote!(#[form_request(from = Query)]),
        )];

        assert_eq!(request_binding_marker(&attributes), Some("form_request"));
    }

    #[test]
    fn finds_the_route_parameter_marker() {
        let attributes = vec![IndexedAttribute::new(
            parse_quote!(#[route_parameter(from = "id")]),
        )];

        assert_eq!(request_binding_marker(&attributes), Some("route_parameter"));
    }

    #[test]
    fn reports_no_marker_on_an_unmarked_parameter() {
        let attributes = vec![IndexedAttribute::new(parse_quote!(#[console_argument]))];

        assert_eq!(request_binding_marker(&attributes), None);
    }
}
