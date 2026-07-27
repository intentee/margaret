use margaret_attributes::indexed_method::IndexedMethod;

use crate::parameter_view::ParameterView;

#[must_use]
pub fn parameters(method: &IndexedMethod) -> Vec<ParameterView<'_>> {
    method
        .parameters()
        .iter()
        .map(|parameter| ParameterView {
            attributes: parameter.attributes(),
            declared: parameter.declared(),
            holder: parameter.holder().clone(),
            position: parameter.position(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use margaret_attributes::indexed_method::IndexedMethod;
    use syn::parse_quote;

    use super::parameters;

    #[test]
    fn skips_the_receiver_and_names_holders_from_identifier_patterns() {
        let signature = parse_quote!(fn run(&self, request: &Request, count: usize));
        let method = IndexedMethod::new("run".to_string(), Vec::new(), signature);

        let views = parameters(&method);

        assert_eq!(views.len(), 2);
        assert_eq!(views[0].holder.to_string(), "request");
        assert_eq!(views[0].position, 1);
        assert_eq!(views[1].holder.to_string(), "count");
        assert_eq!(views[1].position, 2);
    }

    #[test]
    fn synthesizes_a_holder_for_a_destructured_pattern() {
        let signature = parse_quote!(fn run(&self, Point { x, y }: Point));
        let method = IndexedMethod::new("run".to_string(), Vec::new(), signature);

        let views = parameters(&method);

        assert_eq!(views.len(), 1);
        assert_eq!(views[0].holder.to_string(), "argument_1");
    }
}
