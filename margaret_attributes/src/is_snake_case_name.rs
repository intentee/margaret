use syn::Ident;
use syn::ext::IdentExt;
use syn::parse::Parser;

#[must_use]
pub fn is_snake_case_name(name: &str) -> bool {
    Ident::parse_any
        .parse_str(name)
        .is_ok_and(|identifier| identifier == identifier.unraw())
        && !name.chars().any(|character| character.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::is_snake_case_name;

    #[test]
    fn accepts_a_snake_case_word() {
        assert!(is_snake_case_name("article_translations"));
    }

    #[test]
    fn accepts_a_word_that_is_a_rust_keyword() {
        assert!(is_snake_case_name("type"));
    }

    #[test]
    fn rejects_a_word_that_contains_uppercase() {
        assert!(!is_snake_case_name("Articles"));
    }

    #[test]
    fn rejects_the_raw_identifier_syntax() {
        assert!(!is_snake_case_name("r#type"));
    }

    #[test]
    fn rejects_text_that_is_not_a_single_word() {
        assert!(!is_snake_case_name("article-translations"));
    }
}
