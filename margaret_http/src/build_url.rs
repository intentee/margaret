use crate::route_origin::RouteOrigin;
use crate::url_segment::UrlSegment;

const PATH_PARAMETER_ENCODE_SET: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'%')
    .add(b'/')
    .add(b':')
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
pub fn build_url(origin: &RouteOrigin, segments: &[UrlSegment]) -> String {
    let mut url = String::from(origin.as_str());

    for segment in segments {
        match segment {
            UrlSegment::Literal(text) => url.push_str(text),
            UrlSegment::Parameter(parameter) => url.extend(percent_encoding::utf8_percent_encode(
                &parameter.value,
                PATH_PARAMETER_ENCODE_SET,
            )),
        }
    }

    url
}

#[cfg(test)]
mod tests {
    use super::build_url;
    use crate::route_origin::RouteOrigin;
    use crate::url_parameter::UrlParameter;
    use crate::url_segment::UrlSegment;

    #[test]
    fn joins_literal_segments_onto_the_origin() {
        let origin = RouteOrigin::parse("https://example.test").expect("a valid origin");
        let url = build_url(&origin, &[UrlSegment::Literal("/greeting")]);

        assert_eq!(url, "https://example.test/greeting");
    }

    #[test]
    fn substitutes_parameter_values_in_segment_order() {
        let origin = RouteOrigin::parse("https://example.test").expect("a valid origin");
        let url = build_url(
            &origin,
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

        assert_eq!(url, "https://example.test/articles/rust/comments/42");
    }

    #[test]
    fn encodes_a_parameter_as_one_path_segment() {
        let origin = RouteOrigin::parse("https://example.test").expect("a valid origin");
        let url = build_url(
            &origin,
            &[
                UrlSegment::Literal("/files/"),
                UrlSegment::Parameter(UrlParameter {
                    name: "file",
                    value: "../secret?q=1#fragment".to_string(),
                }),
            ],
        );

        assert_eq!(
            url,
            "https://example.test/files/..%2Fsecret%3Fq=1%23fragment"
        );
    }
}
