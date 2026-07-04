use syn::Type;

use margaret_attributes::type_leaf_ident::type_leaf_ident;

pub fn reference_leaf_matches(declared: &Type, name: &str) -> bool {
    let Type::Reference(reference) = declared else {
        return false;
    };

    type_leaf_ident(&reference.elem).is_some_and(|ident| ident == name)
}

#[cfg(test)]
mod tests {
    use syn::Type;
    use syn::parse_quote;

    use super::reference_leaf_matches;

    #[test]
    fn matches_a_reference_to_the_named_type() {
        let declared: Type = parse_quote!(&Request);

        assert!(reference_leaf_matches(&declared, "Request"));
    }

    #[test]
    fn rejects_a_non_reference_type() {
        let declared: Type = parse_quote!(Request);

        assert!(!reference_leaf_matches(&declared, "Request"));
    }

    #[test]
    fn rejects_a_reference_to_a_different_type() {
        let declared: Type = parse_quote!(&Routes);

        assert!(!reference_leaf_matches(&declared, "Request"));
    }
}
