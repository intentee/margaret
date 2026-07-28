use crate::encode_url_catch_all::encode_url_catch_all;
use crate::encode_url_path_segment::encode_url_path_segment;
use crate::url_parameter::UrlParameter;
use crate::url_segment::UrlSegment;

#[must_use]
pub fn build_url(origin: &str, segments: &[UrlSegment]) -> String {
    let mut url = String::from(origin);

    for segment in segments {
        match segment {
            UrlSegment::CatchAllParameter(UrlParameter { value, .. }) => {
                url.push_str(&encode_url_catch_all(value));
            }
            UrlSegment::Literal(text) => url.push_str(text),
            UrlSegment::SegmentParameter(UrlParameter { value, .. }) => {
                url.push_str(&encode_url_path_segment(value));
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
    fn substitutes_parameter_values_in_segment_order() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::SegmentParameter(UrlParameter {
                    name: "article",
                    value: "rust".to_string(),
                }),
                UrlSegment::Literal("/comments/"),
                UrlSegment::SegmentParameter(UrlParameter {
                    name: "comment",
                    value: "42".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/articles/rust/comments/42");
    }

    #[test]
    fn keeps_a_segment_parameter_from_escaping_its_segment() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::SegmentParameter(UrlParameter {
                    name: "article",
                    value: "../admin".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/articles/%2E.%2Fadmin");
    }

    #[test]
    fn does_not_build_a_protocol_relative_url_from_a_leading_slash_parameter() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/"),
                UrlSegment::SegmentParameter(UrlParameter {
                    name: "page",
                    value: "/evil.example.com".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/%2Fevil.example.com");
    }

    #[test]
    fn keeps_the_separators_of_a_catch_all_parameter() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/files/"),
                UrlSegment::CatchAllParameter(UrlParameter {
                    name: "file_path",
                    value: "nested/../secret".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/files/nested/%2E./secret");
    }
}
