use syn::Ident;
use syn::Type;

pub fn type_leaf_ident(declared: &Type) -> Option<&Ident> {
    match declared {
        Type::Path(type_path) => type_path.path.segments.last().map(|segment| &segment.ident),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use syn::Type;
    use syn::parse_quote;

    use super::type_leaf_ident;

    #[test]
    fn returns_the_last_segment_of_a_path_type() {
        let declared: Type = parse_quote!(models::article::Article);

        assert_eq!(
            type_leaf_ident(&declared).map(|ident| ident.to_string()),
            Some("Article".to_string())
        );
    }

    #[test]
    fn returns_nothing_for_a_non_path_type() {
        let declared: Type = parse_quote!(&Article);

        assert_eq!(type_leaf_ident(&declared), None);
    }
}
