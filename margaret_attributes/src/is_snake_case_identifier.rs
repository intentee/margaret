pub fn is_snake_case_identifier(name: &str) -> bool {
    syn::parse_str::<syn::Ident>(name).is_ok()
        && !name.chars().any(|character| character.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::is_snake_case_identifier;

    #[test]
    fn accepts_a_snake_case_identifier() {
        assert!(is_snake_case_identifier("get_article"));
    }

    #[test]
    fn rejects_an_identifier_that_contains_uppercase() {
        assert!(!is_snake_case_identifier("getArticle"));
    }

    #[test]
    fn rejects_a_reserved_keyword() {
        assert!(!is_snake_case_identifier("match"));
    }
}
