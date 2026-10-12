use unicode_ident::is_xid_continue;
use unicode_ident::is_xid_start;

const IDENTIFIER_LEADING_UNDERSCORE: char = '_';

pub(crate) fn is_identifier_text(name: &str) -> bool {
    let mut characters = name.chars();

    characters
        .next()
        .is_some_and(|first| first == IDENTIFIER_LEADING_UNDERSCORE || is_xid_start(first))
        && characters.all(is_xid_continue)
}

#[cfg(test)]
mod tests {
    use super::is_identifier_text;

    #[test]
    fn accepts_unicode_identifier_characters_so_non_ascii_names_stay_usable() {
        assert!(is_identifier_text("żółw_2"));
    }

    #[test]
    fn accepts_a_leading_underscore_that_rust_allows_although_it_is_not_xid_start() {
        assert!(is_identifier_text("_1"));
    }

    #[test]
    fn rejects_an_empty_text_because_an_identifier_needs_a_first_character() {
        assert!(!is_identifier_text(""));
    }

    #[test]
    fn rejects_a_leading_digit_because_rust_identifiers_cannot_start_with_one() {
        assert!(!is_identifier_text("1a"));
    }

    #[test]
    fn rejects_a_space_inside_the_text_because_an_identifier_is_one_word() {
        assert!(!is_identifier_text("a b"));
    }
}
