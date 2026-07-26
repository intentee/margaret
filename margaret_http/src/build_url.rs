use crate::encode_path_segment::encode_path_segment;
use crate::url_segment::UrlSegment;

fn encode_path_tail(value: &str) -> String {
    value
        .split('/')
        .map(encode_path_segment)
        .collect::<Vec<String>>()
        .join("/")
}

#[must_use]
pub fn build_url(origin: &str, segments: &[UrlSegment]) -> String {
    let mut url = String::from(origin);

    for segment in segments {
        match segment {
            UrlSegment::Literal(text) => url.push_str(text),
            UrlSegment::Parameter(parameter) => {
                url.push_str(&encode_path_segment(&parameter.value));
            }
            UrlSegment::CatchAll(parameter) => {
                url.push_str(&encode_path_tail(&parameter.value));
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
    fn confines_a_hostile_parameter_to_a_single_segment() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter(UrlParameter {
                    name: "article",
                    value: "../admin?mode=x".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/articles/..%2Fadmin%3Fmode=x");
    }

    #[test]
    fn keeps_slashes_in_a_catch_all_while_neutralizing_a_traversal_sub_segment() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/assets/"),
                UrlSegment::CatchAll(UrlParameter {
                    name: "asset_path",
                    value: "css/../secret".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/assets/css/%2E%2E/secret");
    }

    #[test]
    fn keeps_a_legitimate_catch_all_path_intact() {
        let url = build_url(
            "http://localhost",
            &[
                UrlSegment::Literal("/assets/"),
                UrlSegment::CatchAll(UrlParameter {
                    name: "asset_path",
                    value: "css/app.css".to_string(),
                }),
            ],
        );

        assert_eq!(url, "http://localhost/assets/css/app.css");
    }
}
