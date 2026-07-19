use quote::format_ident;
use syn::FnArg;
use syn::Pat;
use syn::Signature;

use crate::parameter_view::ParameterView;

#[must_use]
pub fn parameters(signature: &Signature) -> Vec<ParameterView<'_>> {
    signature
        .inputs
        .iter()
        .enumerate()
        .filter_map(|(position, input)| {
            let FnArg::Typed(pattern_type) = input else {
                return None;
            };

            let holder = match pattern_type.pat.as_ref() {
                Pat::Ident(pattern_ident) => pattern_ident.ident.clone(),
                _ => format_ident!("argument_{position}"),
            };

            Some(ParameterView {
                attributes: &pattern_type.attrs,
                declared: pattern_type.ty.as_ref(),
                holder,
                position,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use syn::Signature;
    use syn::parse_quote;

    use super::parameters;

    #[test]
    fn skips_the_receiver_and_names_holders_from_identifier_patterns() {
        let signature: Signature = parse_quote!(fn run(&self, request: &Request, count: usize));

        let views = parameters(&signature);

        assert_eq!(views.len(), 2);
        assert_eq!(views[0].holder.to_string(), "request");
        assert_eq!(views[0].position, 1);
        assert_eq!(views[1].holder.to_string(), "count");
        assert_eq!(views[1].position, 2);
    }

    #[test]
    fn synthesizes_a_holder_for_a_destructured_pattern() {
        let signature: Signature = parse_quote!(fn run(&self, Point { x, y }: Point));

        let views = parameters(&signature);

        assert_eq!(views.len(), 1);
        assert_eq!(views[0].holder.to_string(), "argument_1");
    }
}
