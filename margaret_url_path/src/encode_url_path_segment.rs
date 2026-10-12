use percent_encoding::utf8_percent_encode;

use crate::non_pchar::NON_PCHAR;

#[must_use]
pub fn encode_url_path_segment(segment: &str) -> String {
    utf8_percent_encode(segment, NON_PCHAR).to_string()
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::encode_url_path_segment;

    const RESERVED_SEGMENT: &str =
        "{callback} 100%a\\b|c^d[e]f`g\"h<i>j?k#l\u{7f}m\u{17c}\u{f3}\u{142}w";
    const ENCODED_RESERVED_SEGMENT: &str = "%7Bcallback%7D%20100%25a%5Cb%7Cc%5Ed%5Be%5Df%60g%22h%3Ci%3Ej%3Fk%23l%7Fm%C5%BC%C3%B3%C5%82w";

    #[test]
    fn encodes_every_character_that_is_not_a_path_character_so_no_url_part_is_shifted() {
        assert_eq!(
            encode_url_path_segment(RESERVED_SEGMENT),
            ENCODED_RESERVED_SEGMENT
        );
    }

    #[test]
    fn encodes_a_segment_into_text_that_url_parsing_keeps_verbatim() {
        let url = Url::parse(&format!("https://example.test/{ENCODED_RESERVED_SEGMENT}"))
            .expect("the encoded segment forms a url");

        assert_eq!(url.path(), format!("/{ENCODED_RESERVED_SEGMENT}"));
    }

    #[test]
    fn encodes_characters_that_would_reshape_the_path() {
        assert_eq!(
            encode_url_path_segment("a/b?c#d e%f"),
            "a%2Fb%3Fc%23d%20e%25f"
        );
    }

    #[test]
    fn preserves_characters_allowed_inside_a_segment_so_valid_values_pass_unchanged() {
        assert_eq!(
            encode_url_path_segment("aZ9-_~!$&'()*+,;=:@"),
            "aZ9-_~!$&'()*+,;=:@"
        );
    }

    #[test]
    fn keeps_a_leading_dot_of_a_segment_that_is_not_a_dot_segment() {
        assert_eq!(encode_url_path_segment(".env"), ".env");
    }

    #[test]
    fn percent_encodes_non_ascii_characters_as_utf8_bytes_since_urls_only_carry_ascii() {
        assert_eq!(encode_url_path_segment("żółw"), "%C5%BC%C3%B3%C5%82w");
    }
}
