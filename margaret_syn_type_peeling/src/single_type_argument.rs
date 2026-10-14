use syn::Type;

use crate::single_generic_argument::single_generic_argument;

#[must_use]
pub fn single_type_argument(declared: &Type) -> Option<&Type> {
    let Type::Path(type_path) = declared else {
        return None;
    };

    type_path
        .path
        .segments
        .last()
        .and_then(single_generic_argument)
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::Type;

    use super::single_type_argument;

    fn argument(type_source: &str) -> Option<String> {
        let declared: Type = syn::parse_str(type_source).expect("the type fixture parses");

        single_type_argument(&declared).map(|inner| quote!(#inner).to_string())
    }

    #[test]
    fn returns_the_only_type_argument_of_the_last_path_segment() {
        assert_eq!(argument("crate::Wrapper<Node>").as_deref(), Some("Node"));
    }

    #[test]
    fn ignores_a_borrowed_type_since_only_an_owned_path_carries_the_argument() {
        assert!(argument("&Wrapper<Node>").is_none());
    }
}
