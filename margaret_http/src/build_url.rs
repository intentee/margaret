use crate::url_segment::UrlSegment;

#[must_use]
pub fn build_url(origin: &str, segments: &[UrlSegment], values: &[String]) -> String {
    let mut values = values.iter();

    segments
        .iter()
        .fold(String::from(origin), |mut url, segment| {
            match segment {
                UrlSegment::Literal(text) => url.push_str(text),
                UrlSegment::Parameter(_) => url.extend(values.next().map(String::as_str)),
            }

            url
        })
}

#[cfg(test)]
mod tests {
    use super::build_url;
    use crate::url_segment::UrlSegment;

    #[test]
    fn joins_literal_segments_onto_the_origin() {
        let url = build_url("http://localhost", &[UrlSegment::Literal("/greeting")], &[]);

        assert_eq!(url, "http://localhost/greeting");
    }

    #[test]
    fn substitutes_parameter_values_in_segment_order() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter("article"),
                UrlSegment::Literal("/comments/"),
                UrlSegment::Parameter("comment"),
            ],
            &["rust".to_string(), "42".to_string()],
        );

        assert_eq!(url, "http://localhost/articles/rust/comments/42");
    }
}
