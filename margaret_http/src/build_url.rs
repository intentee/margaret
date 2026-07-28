use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use percent_encoding::utf8_percent_encode;

use crate::url_segment::UrlSegment;

const PATH_SEPARATOR: char = '/';

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

fn push_segment(url: &mut String, value: &str) {
    url.extend(utf8_percent_encode(value, PATH_SEGMENT));
}

#[must_use]
pub fn build_url(origin: &str, segments: &[UrlSegment]) -> String {
    let mut url = String::from(origin);

    for segment in segments {
        match segment {
            UrlSegment::Literal(text) => url.push_str(text),
            UrlSegment::Parameter(parameter) => push_segment(&mut url, &parameter.value),
            UrlSegment::WildcardParameter(parameter) => {
                for (index, part) in parameter.value.split(PATH_SEPARATOR).enumerate() {
                    if index > 0 {
                        url.push(PATH_SEPARATOR);
                    }

                    push_segment(&mut url, part);
                }
            }
        }
    }

    url
}

#[cfg(test)]
mod tests {
    use super::build_url;
    use crate::url_parameter::UrlParameter;
    use crate::url_segment::UrlSegment;

    fn parameter(name: &'static str, value: &str) -> UrlParameter {
        UrlParameter {
            name,
            value: value.to_string(),
        }
    }

    #[test]
    fn joins_literal_segments_onto_the_origin() {
        let url = build_url("http://localhost", &[UrlSegment::Literal("/greeting")]);

        assert_eq!(url, "http://localhost/greeting");
    }

    #[test]
    fn substitutes_parameter_values_in_segment_order() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter(parameter("article", "rust")),
                UrlSegment::Literal("/comments/"),
                UrlSegment::Parameter(parameter("comment", "42")),
            ],
        );

        assert_eq!(url, "http://localhost/articles/rust/comments/42");
    }

    #[test]
    fn encodes_a_path_separator_inside_a_parameter() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter(parameter("article", "a/b")),
            ],
        );

        assert_eq!(url, "http://localhost/articles/a%2Fb");
    }

    #[test]
    fn encodes_a_dot_segment_that_a_parameter_would_introduce() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter(parameter("article", "../admin")),
            ],
        );

        assert_eq!(url, "http://localhost/articles/..%2Fadmin");
    }

    #[test]
    fn encodes_a_line_break_that_a_parameter_would_introduce() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter(parameter("article", "a\r\nX-Injected: 1")),
            ],
        );

        assert_eq!(url, "http://localhost/articles/a%0D%0AX-Injected:%201");
    }

    #[test]
    fn encodes_a_query_delimiter_that_a_parameter_would_introduce() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter(parameter("article", "a?b#c")),
            ],
        );

        assert_eq!(url, "http://localhost/articles/a%3Fb%23c");
    }

    #[test]
    fn encodes_a_percent_that_a_parameter_would_introduce() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/files/"),
                UrlSegment::Parameter(parameter("file", "100%")),
            ],
        );

        assert_eq!(url, "http://localhost/files/100%25");
    }

    #[test]
    fn keeps_the_separators_of_a_wildcard_parameter() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/assets/"),
                UrlSegment::WildcardParameter(parameter("asset_path", "css/app.css")),
            ],
        );

        assert_eq!(url, "http://localhost/assets/css/app.css");
    }

    #[test]
    fn encodes_each_part_of_a_wildcard_parameter() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/assets/"),
                UrlSegment::WildcardParameter(parameter("asset_path", "a b/c?d")),
            ],
        );

        assert_eq!(url, "http://localhost/assets/a%20b/c%3Fd");
    }
}
