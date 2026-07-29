use syn::Type;

use crate::single_generic_argument::single_generic_argument;

#[must_use]
pub fn option_inner(declared_type: &Type) -> Option<&Type> {
    let Type::Path(type_path) = declared_type else {
        return None;
    };

    match type_path.path.segments.last() {
        Some(segment) if segment.ident == "Option" => single_generic_argument(segment),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::Type;

    use super::option_inner;

    fn inner(type_source: &str) -> Option<String> {
        let declared_type: Type = syn::parse_str(type_source).expect("the type fixture parses");

        option_inner(&declared_type).map(|inner| quote!(#inner).to_string())
    }

    #[test]
    fn peels_the_optional_type() {
        assert_eq!(inner("Option<Node>").as_deref(), Some("Node"));
    }

    #[test]
    fn peels_a_fully_qualified_optional_type() {
        assert_eq!(inner("std::option::Option<Node>").as_deref(), Some("Node"));
    }

    #[test]
    fn ignores_a_non_optional_path() {
        assert!(inner("Node").is_none());
    }

    #[test]
    fn ignores_a_non_path_type() {
        assert!(inner("[u8; 4]").is_none());
    }

    #[test]
    fn ignores_an_optional_with_more_than_one_argument() {
        assert!(inner("Option<Node, Edge>").is_none());
    }
}
