use crate::url_segment::UrlSegment;

#[must_use]
pub fn build_url(origin: &str, segments: &[UrlSegment]) -> String {
    let mut url = String::from(origin);

    for segment in segments {
        match segment {
            UrlSegment::Literal(text) => url.push_str(text),
            UrlSegment::Parameter(parameter) => url.push_str(&parameter.value),
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
