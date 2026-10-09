use percent_encoding::AsciiSet;
use percent_encoding::utf8_percent_encode;

use crate::non_pchar::NON_PCHAR;

const NON_PCHAR_OR_SEPARATOR: &AsciiSet = &NON_PCHAR.remove(b'/');

#[must_use]
pub fn encode_url_path_literal(literal: &str) -> String {
    utf8_percent_encode(literal, NON_PCHAR_OR_SEPARATOR).to_string()
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::encode_url_path_literal;

    const RESERVED_LITERAL: &str = "/{callback} 100%/a\\b|c^d[e]f`g\"h<i>j?k#l\u{7f}m/żółw";
    const ENCODED_RESERVED_LITERAL: &str = "/%7Bcallback%7D%20100%25/a%5Cb%7Cc%5Ed%5Be%5Df%60g%22h%3Ci%3Ej%3Fk%23l%7Fm/%C5%BC%C3%B3%C5%82w";

    #[test]
    fn keeps_the_separators_and_dots_of_a_literal_path() {
        assert_eq!(
            encode_url_path_literal("/.well-known/openid-configuration"),
            "/.well-known/openid-configuration"
        );
    }

    #[test]
    fn encodes_every_character_that_is_not_a_path_character() {
        assert_eq!(
            encode_url_path_literal(RESERVED_LITERAL),
            ENCODED_RESERVED_LITERAL
        );
    }

    #[test]
    fn encodes_a_literal_into_the_path_that_url_parsing_preserves() {
        let url = Url::parse(&format!("https://example.test{ENCODED_RESERVED_LITERAL}"))
            .expect("the encoded literal forms a url");

        assert_eq!(url.path(), ENCODED_RESERVED_LITERAL);
    }
}
