use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_attribute::IndexedAttribute;

const REQUEST_BINDING_MARKERS: [FrameworkAttribute; 3] = [
    FrameworkAttribute::AuthenticatedUser,
    FrameworkAttribute::FormRequest,
    FrameworkAttribute::RouteParameter,
];

#[must_use]
pub fn request_binding_marker(attributes: &[IndexedAttribute]) -> Option<FrameworkAttribute> {
    REQUEST_BINDING_MARKERS.into_iter().find(|marker| {
        attributes
            .iter()
            .any(|attribute| attribute.framework_attribute() == Some(*marker))
    })
}

#[cfg(test)]
mod tests {
    use margaret_attributes::framework_attribute::FrameworkAttribute;
    use margaret_attributes::indexed_attribute::IndexedAttribute;
    use syn::parse_quote;

    use super::request_binding_marker;

    #[test]
    fn finds_the_authenticated_user_marker() {
        let attributes = vec![IndexedAttribute::new(&parse_quote!(#[authenticated_user]))];

        assert_eq!(
            request_binding_marker(&attributes),
            Some(FrameworkAttribute::AuthenticatedUser)
        );
    }

    #[test]
    fn finds_the_form_request_marker() {
        let attributes = vec![IndexedAttribute::new(
            &parse_quote!(#[form_request(from = Query)]),
        )];

        assert_eq!(
            request_binding_marker(&attributes),
            Some(FrameworkAttribute::FormRequest)
        );
    }

    #[test]
    fn finds_the_route_parameter_marker() {
        let attributes = vec![IndexedAttribute::new(
            &parse_quote!(#[route_parameter(from = "id")]),
        )];

        assert_eq!(
            request_binding_marker(&attributes),
            Some(FrameworkAttribute::RouteParameter)
        );
    }

    #[test]
    fn reports_no_marker_on_an_unmarked_parameter() {
        let attributes = vec![IndexedAttribute::new(&parse_quote!(#[console_argument]))];

        assert_eq!(request_binding_marker(&attributes), None);
    }
}
