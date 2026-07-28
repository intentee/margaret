use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use percent_encoding::utf8_percent_encode;

const NON_PCHAR: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'%')
    .add(b'/')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');

const ENCODED_LEADING_DOT: &str = "%2E";

pub(crate) fn encode_url_path_segment(value: &str) -> String {
    match value.strip_prefix('.') {
        Some(rest) => format!(
            "{ENCODED_LEADING_DOT}{}",
            utf8_percent_encode(rest, NON_PCHAR)
        ),
        None => utf8_percent_encode(value, NON_PCHAR).to_string(),
    }
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
    fn keeps_a_dot_that_does_not_open_the_segment() {
        assert_eq!(encode_url_path_segment("photo.jpg"), "photo.jpg");
    }

    #[test]
    fn encodes_a_leading_dot_so_the_segment_cannot_be_a_dot_segment() {
        assert_eq!(encode_url_path_segment(".."), "%2E.");
        assert_eq!(encode_url_path_segment("."), "%2E");
        assert_eq!(encode_url_path_segment(".env"), "%2Eenv");
    }

    #[test]
    fn encodes_non_ascii_characters() {
        assert_eq!(encode_url_path_segment("żółw"), "%C5%BC%C3%B3%C5%82w");
    }
}
