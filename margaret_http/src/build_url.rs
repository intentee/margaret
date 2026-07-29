use crate::encode_url_catch_all::encode_url_catch_all;
use crate::encode_url_path_segment::encode_url_path_segment;
use crate::url_segment::UrlSegment;

#[must_use]
pub fn build_url(origin: &str, segments: &[UrlSegment]) -> String {
    let mut url = String::from(origin);

    for segment in segments {
        match segment {
            UrlSegment::CatchAllParameter(parameter) => {
                url.push_str(&encode_url_catch_all(&parameter.value));
            }
            UrlSegment::Literal(text) => url.push_str(text),
            UrlSegment::Parameter(parameter) => {
                url.push_str(&encode_url_path_segment(&parameter.value));
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

    #[test]
    fn keeps_a_parameter_from_escaping_its_segment_with_a_dot_segment() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter(UrlParameter {
                    name: "article",
                    value: "..".to_string(),
                }),
                UrlSegment::Literal("/edit"),
            ],
        );

        assert_eq!(url, "http://localhost/articles/%2E./edit");
    }

    #[test]
    fn keeps_a_catch_all_component_from_becoming_a_dot_segment() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/files/"),
                UrlSegment::CatchAllParameter(UrlParameter {
                    name: "rest",
                    value: "nested/../secret".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/files/nested/%2E./secret");
    }

    #[test]
    fn does_not_build_a_protocol_relative_url_from_a_leading_slash_parameter() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/"),
                UrlSegment::Parameter(UrlParameter {
                    name: "page",
                    value: "/evil.example.com".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/%2Fevil.example.com");
    }
}
