use syn::Attribute;

use crate::attribute_selector::AttributeSelector;

pub fn marker<'attributes>(
    attributes: &'attributes [Attribute],
    selector: &AttributeSelector,
) -> Option<&'attributes Attribute> {
    attributes
        .iter()
        .find(|attribute| selector.matches(attribute.path()))
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use crate::attribute_selector::AttributeSelector;

    use super::marker;

    fn selector(input: &str) -> AttributeSelector {
        AttributeSelector::parse(input).expect("the selector parses")
    }

    #[test]
    fn finds_a_matching_marker() {
        let attributes: Vec<Attribute> = vec![parse_quote!(#[route_parameter(from = id)])];

        assert!(marker(&attributes, &selector("route_parameter")).is_some());
    }

    #[test]
    fn returns_nothing_when_no_marker_matches() {
        let attributes: Vec<Attribute> = vec![parse_quote!(#[form_request(from = Query)])];

        assert!(marker(&attributes, &selector("route_parameter")).is_none());
    }
}
