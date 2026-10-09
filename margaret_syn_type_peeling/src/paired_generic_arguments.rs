use syn::GenericArgument;
use syn::PathArguments;
use syn::PathSegment;

use crate::generic_argument_pair::GenericArgumentPair;

#[must_use]
pub fn paired_generic_arguments(segment: &PathSegment) -> Option<GenericArgumentPair<'_>> {
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };

    match arguments.args.iter().collect::<Vec<_>>().as_slice() {
        [GenericArgument::Type(first), GenericArgument::Type(second)] => {
            Some(GenericArgumentPair { first, second })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::PathSegment;

    use super::paired_generic_arguments;
    use crate::generic_argument_pair::GenericArgumentPair;

    fn arguments(segment_source: &str) -> Option<String> {
        let segment: PathSegment =
            syn::parse_str(segment_source).expect("the path segment fixture parses");

        paired_generic_arguments(&segment)
            .map(|GenericArgumentPair { first, second }| quote!(#first | #second).to_string())
    }

    #[test]
    fn returns_both_generic_type_arguments() {
        assert_eq!(
            arguments("Wrapper<Claims, Profile>").as_deref(),
            Some("Claims | Profile")
        );
    }

    #[test]
    fn ignores_a_segment_with_a_single_generic_argument() {
        assert!(arguments("Wrapper<Claims>").is_none());
    }

    #[test]
    fn ignores_a_segment_without_generic_arguments() {
        assert!(arguments("Wrapper").is_none());
    }
}
