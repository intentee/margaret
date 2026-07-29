use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use percent_encoding::utf8_percent_encode;

use crate::url_segment::UrlSegment;

const CATCH_ALL_PARAMETER: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'%')
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

const PARAMETER: &AsciiSet = &CATCH_ALL_PARAMETER.add(b'/');

#[must_use]
pub fn build_url(origin: &str, segments: &[UrlSegment]) -> String {
    let mut url = String::from(origin);

    for segment in segments {
        match segment {
            UrlSegment::CatchAllParameter(parameter) => {
                url.extend(utf8_percent_encode(&parameter.value, CATCH_ALL_PARAMETER));
            }
            UrlSegment::Literal(text) => url.push_str(text),
            UrlSegment::Parameter(parameter) => {
                url.extend(utf8_percent_encode(&parameter.value, PARAMETER));
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

    #[test]
    fn joins_literal_segments_onto_the_origin() {
        let url = build_url("http://localhost", &[UrlSegment::Literal("/greeting")]);

        assert_eq!(url, "http://localhost/greeting");
    }

    #[test]
    fn percent_encodes_a_parameter_value_that_contains_url_delimiters() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter(UrlParameter {
                    name: "article",
                    value: "a b#c?d%e".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/articles/a%20b%23c%3Fd%25e");
    }

    #[test]
    fn keeps_path_separators_inside_a_catch_all_parameter_value() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/files/"),
                UrlSegment::CatchAllParameter(UrlParameter {
                    name: "rest",
                    value: "images/logo .png".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/files/images/logo%20.png");
    }

    #[test]
    fn percent_encodes_a_parameter_value_that_contains_a_path_separator() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter(UrlParameter {
                    name: "article",
                    value: "rust/lang".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/articles/rust%2Flang");
    }

    #[test]
    fn substitutes_parameter_values_in_segment_order() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter(UrlParameter {
                    name: "article",
                    value: "rust".to_string(),
                }),
                UrlSegment::Literal("/comments/"),
                UrlSegment::Parameter(UrlParameter {
                    name: "comment",
                    value: "42".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/articles/rust/comments/42");
    }
}
