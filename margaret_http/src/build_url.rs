use crate::url_segment::UrlSegment;

#[must_use]
pub fn build_url(origin: &str, segments: &[UrlSegment], values: &[String]) -> String {
    let mut url = String::from(origin);
    let mut value_index = 0;

    for segment in segments {
        match segment {
            UrlSegment::Literal(text) => url.push_str(text),
            UrlSegment::Parameter(_) => {
                url.push_str(&values[value_index]);
                value_index += 1;
            }
        }
    }

    url
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
