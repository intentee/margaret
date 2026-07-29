use syn::GenericArgument;
use syn::PathArguments;
use syn::PathSegment;
use syn::Type;

#[must_use]
pub fn single_generic_argument(segment: &PathSegment) -> Option<&Type> {
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };

    let mut arguments = arguments.args.iter();

    match (arguments.next(), arguments.next()) {
        (Some(GenericArgument::Type(generic_type)), None) => Some(generic_type),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::PathSegment;

    use super::single_generic_argument;

    fn argument(segment_source: &str) -> Option<String> {
        let segment: PathSegment =
            syn::parse_str(segment_source).expect("the path segment fixture parses");

        single_generic_argument(&segment).map(|inner| quote!(#inner).to_string())
    }

    #[test]
    fn returns_the_only_generic_type_argument() {
        assert_eq!(argument("Wrapper<Node>").as_deref(), Some("Node"));
    }

    #[test]
    fn ignores_a_segment_without_generic_arguments() {
        assert!(argument("Wrapper").is_none());
    }

    #[test]
    fn ignores_a_segment_with_more_than_one_generic_argument() {
        assert!(argument("Wrapper<Node, Edge>").is_none());
    }

    #[test]
    fn ignores_a_segment_whose_only_argument_is_not_a_type() {
        assert!(argument("Wrapper<'static>").is_none());
    }
}
