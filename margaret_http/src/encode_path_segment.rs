use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use percent_encoding::utf8_percent_encode;

const PATH_SEGMENT: &AsciiSet = &CONTROLS
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

#[must_use]
pub fn encode_path_segment(value: &str) -> String {
    match value {
        "." => "%2E".to_string(),
        ".." => "%2E%2E".to_string(),
        other => utf8_percent_encode(other, PATH_SEGMENT).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::encode_path_segment;

    #[test]
    fn leaves_an_unreserved_identifier_unchanged() {
        assert_eq!(encode_path_segment("rust"), "rust");
        assert_eq!(encode_path_segment("v1.2"), "v1.2");
    }

    #[test]
    fn confines_traversal_and_query_delimiters_to_a_single_segment() {
        assert_eq!(encode_path_segment("../admin?mode=x"), "..%2Fadmin%3Fmode=x");
    }

    #[test]
    fn encodes_a_backslash_that_browsers_treat_as_a_slash() {
        assert_eq!(encode_path_segment("a\\b"), "a%5Cb");
    }

    #[test]
    fn neutralizes_the_single_dot_segment() {
        assert_eq!(encode_path_segment("."), "%2E");
    }

    #[test]
    fn neutralizes_the_parent_dot_segment() {
        assert_eq!(encode_path_segment(".."), "%2E%2E");
    }
}
