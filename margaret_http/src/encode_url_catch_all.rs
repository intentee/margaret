use crate::encode_url_path_segment::encode_url_path_segment;

pub(crate) fn encode_url_catch_all(value: &str) -> String {
    value
        .split('/')
        .map(encode_url_path_segment)
        .collect::<Vec<String>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::encode_url_catch_all;

    #[test]
    fn keeps_the_separators_between_components() {
        assert_eq!(
            encode_url_catch_all("chunks/chunk_E5F6G7H8.js"),
            "chunks/chunk_E5F6G7H8.js"
        );
    }

    #[test]
    fn encodes_a_leading_dot_in_every_component() {
        assert_eq!(encode_url_catch_all("a/../b"), "a/%2E./b");
    }

    #[test]
    fn encodes_characters_that_would_reshape_a_component() {
        assert_eq!(encode_url_catch_all("a/b?c"), "a/b%3Fc");
    }
}
