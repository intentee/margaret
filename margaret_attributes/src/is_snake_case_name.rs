use crate::is_identifier_text::is_identifier_text;

#[must_use]
pub fn is_snake_case_name(name: &str) -> bool {
    is_identifier_text(name) && !name.chars().any(|character| character.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::is_snake_case_name;

    #[test]
    fn accepts_a_snake_case_word_so_it_can_serve_as_a_declared_name() {
        assert!(is_snake_case_name("article_translations"));
    }

    #[test]
    fn accepts_a_word_that_is_a_rust_keyword() {
        assert!(is_snake_case_name("type"));
    }

    #[test]
    fn rejects_a_word_that_contains_uppercase_so_declared_names_stay_snake_case() {
        assert!(!is_snake_case_name("Articles"));
    }

    #[test]
    fn rejects_the_raw_identifier_syntax_to_keep_it_out_of_declared_names() {
        assert!(!is_snake_case_name("r#type"));
    }

    #[test]
    fn rejects_a_word_padded_with_whitespace_so_no_declared_name_carries_spaces() {
        assert!(!is_snake_case_name(" articles "));
    }

    #[test]
    fn rejects_text_that_is_not_a_single_word() {
        assert!(!is_snake_case_name("article-translations"));
    }
}
