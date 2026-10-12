use syn::Ident;

use crate::is_identifier_text::is_identifier_text;

pub(crate) fn is_identifier(name: &str) -> bool {
    is_identifier_text(name) && syn::parse_str::<Ident>(name).is_ok()
}

#[cfg(test)]
mod tests {
    use super::is_identifier;

    #[test]
    fn accepts_a_plain_identifier_that_generated_code_can_name_an_item_with() {
        assert!(is_identifier("get_article"));
    }

    #[test]
    fn rejects_an_identifier_padded_with_whitespace_since_format_ident_would_panic_on_it() {
        assert!(!is_identifier("get_article "));
    }

    #[test]
    fn rejects_a_keyword_because_generated_code_cannot_name_an_item_with_it() {
        assert!(!is_identifier("match"));
    }
}
