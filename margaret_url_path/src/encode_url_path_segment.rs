use percent_encoding::utf8_percent_encode;

use crate::non_pchar::NON_PCHAR;

#[must_use]
pub fn encode_url_path_segment(segment: &str) -> String {
    utf8_percent_encode(segment, NON_PCHAR).to_string()
}

#[cfg(test)]
mod tests {
    use super::encode_url_path_segment;

    #[test]
    fn encodes_characters_that_would_reshape_the_path() {
        assert_eq!(
            encode_url_path_segment("a/b?c#d e%f"),
            "a%2Fb%3Fc%23d%20e%25f"
        );
    }

    #[test]
    fn preserves_characters_allowed_inside_a_segment() {
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
    fn encodes_non_ascii_characters() {
        assert_eq!(encode_url_path_segment("żółw"), "%C5%BC%C3%B3%C5%82w");
    }
}
