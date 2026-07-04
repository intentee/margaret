use syn::Type;

use margaret_attributes::type_leaf_ident::type_leaf_ident;

pub fn leaf_matches(declared: &Type, name: &str) -> bool {
    type_leaf_ident(declared).is_some_and(|ident| ident == name)
}

#[cfg(test)]
mod tests {
    use syn::Type;
    use syn::parse_quote;

    use super::leaf_matches;

    #[test]
    fn matches_the_last_segment_of_a_path_type() {
        let declared: Type = parse_quote!(crate::margaret::routes::Routes);

        assert!(leaf_matches(&declared, "Routes"));
    }

    #[test]
    fn rejects_a_reference_type() {
        let declared: Type = parse_quote!(&Routes);

        assert!(!leaf_matches(&declared, "Routes"));
    }
}
