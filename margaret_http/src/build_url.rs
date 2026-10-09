use margaret_url_path::admit_path_segment::admit_path_segment;
use margaret_url_path::encode_url_path_segment::encode_url_path_segment;
use margaret_url_path::path_segment_admission::PathSegmentAdmission;
use margaret_url_path::path_segment_rejection::PathSegmentRejection;

use crate::route_addressing::RouteAddressing;
use crate::unaddressable_parameter::UnaddressableParameter;
use crate::url_parameter::UrlParameter;
use crate::url_segment::UrlSegment;

const PATH_SEPARATOR: char = '/';

fn unaddressable(name: &'static str, rejection: PathSegmentRejection) -> RouteAddressing<String> {
    RouteAddressing::Unaddressable(UnaddressableParameter { name, rejection })
}

fn addressed_catch_all(
    prefix: &str,
    UrlParameter { name, value }: &UrlParameter,
) -> RouteAddressing<String> {
    if value.is_empty() {
        return unaddressable(name, PathSegmentRejection::Empty);
    }

    let text = [prefix, value].concat();
    let mut encoded = Vec::new();
    let mut components = text.split(PATH_SEPARATOR).peekable();

    while let Some(component) = components.next() {
        let ends_with_separator = component.is_empty() && components.peek().is_none();

        if !ends_with_separator
            && let PathSegmentAdmission::Rejected(rejection) = admit_path_segment(component)
        {
            return unaddressable(name, rejection);
        }

        encoded.push(encode_url_path_segment(component));
    }

    RouteAddressing::Addressed(encoded.join("/"))
}

fn addressed_parameter(
    prefix: &str,
    UrlParameter { name, value }: &UrlParameter,
    suffix: &str,
) -> RouteAddressing<String> {
    if value.is_empty() {
        return unaddressable(name, PathSegmentRejection::Empty);
    }

    let segment = [prefix, value, suffix].concat();

    match admit_path_segment(&segment) {
        PathSegmentAdmission::Admitted => {
            RouteAddressing::Addressed(encode_url_path_segment(&segment))
        }
        PathSegmentAdmission::Rejected(rejection) => unaddressable(name, rejection),
    }
}

#[must_use]
pub fn build_url(origin: &str, segments: &[UrlSegment]) -> RouteAddressing<String> {
    let mut url = String::from(origin);

    for segment in segments {
        let addressing = match segment {
            UrlSegment::CatchAllParameter { parameter, prefix } => {
                addressed_catch_all(prefix, parameter)
            }
            UrlSegment::Literal(text) => {
                url.push_str(text);

                continue;
            }
            UrlSegment::Parameter {
                parameter,
                prefix,
                suffix,
            } => addressed_parameter(prefix, parameter, suffix),
        };

        match addressing {
            RouteAddressing::Addressed(encoded) => url.push_str(&encoded),
            RouteAddressing::Unaddressable(parameter) => {
                return RouteAddressing::Unaddressable(parameter);
            }
        }
    }

    RouteAddressing::Addressed(url)
}

#[cfg(test)]
mod tests {
    use margaret_url_path::path_segment_rejection::PathSegmentRejection;

    use super::build_url;
    use crate::route_addressing::RouteAddressing;
    use crate::unaddressable_parameter::UnaddressableParameter;
    use crate::url_parameter::UrlParameter;
    use crate::url_segment::UrlSegment;

    const ORIGIN: &str = "http://localhost";

    fn parameter(name: &'static str, value: &str) -> UrlSegment {
        UrlSegment::Parameter {
            parameter: UrlParameter {
                name,
                value: value.to_string(),
            },
            prefix: "",
            suffix: "",
        }
    }

    fn catch_all(prefix: &'static str, value: &str) -> UrlSegment {
        UrlSegment::CatchAllParameter {
            parameter: UrlParameter {
                name: "rest",
                value: value.to_string(),
            },
            prefix,
        }
    }

    fn article_url(value: &str) -> RouteAddressing<String> {
        build_url(
            ORIGIN,
            &[
                UrlSegment::Literal("/articles/"),
                parameter("article", value),
                UrlSegment::Literal("/edit"),
            ],
        )
    }

    fn file_url(value: &str) -> RouteAddressing<String> {
        build_url(
            ORIGIN,
            &[UrlSegment::Literal("/files/"), catch_all("", value)],
        )
    }

    fn addressed(url: &str) -> RouteAddressing<String> {
        RouteAddressing::Addressed(url.to_string())
    }

    fn unaddressable(
        name: &'static str,
        rejection: PathSegmentRejection,
    ) -> RouteAddressing<String> {
        RouteAddressing::Unaddressable(UnaddressableParameter { name, rejection })
    }

    #[test]
    fn joins_literal_segments_onto_the_origin() {
        assert_eq!(
            build_url(ORIGIN, &[UrlSegment::Literal("/greeting")]),
            addressed("http://localhost/greeting")
        );
    }

    #[test]
    fn percent_encodes_a_parameter_value_that_contains_url_delimiters() {
        assert_eq!(
            article_url("a b#c?d%e"),
            addressed("http://localhost/articles/a%20b%23c%3Fd%25e/edit")
        );
    }

    #[test]
    fn substitutes_parameter_values_in_segment_order() {
        assert_eq!(
            build_url(
                ORIGIN,
                &[
                    UrlSegment::Literal("/articles/"),
                    parameter("article", "rust"),
                    UrlSegment::Literal("/comments/"),
                    parameter("comment", "42"),
                ],
            ),
            addressed("http://localhost/articles/rust/comments/42")
        );
    }

    #[test]
    fn keeps_a_leading_dot_of_a_value_that_is_not_a_dot_segment() {
        assert_eq!(
            article_url(".env"),
            addressed("http://localhost/articles/.env/edit")
        );
    }

    #[test]
    fn refuses_a_parent_directory_value() {
        assert_eq!(
            article_url(".."),
            unaddressable("article", PathSegmentRejection::DotSegment)
        );
    }

    #[test]
    fn refuses_a_current_directory_value() {
        assert_eq!(
            article_url("."),
            unaddressable("article", PathSegmentRejection::DotSegment)
        );
    }

    #[test]
    fn refuses_an_empty_value() {
        assert_eq!(
            article_url(""),
            unaddressable("article", PathSegmentRejection::Empty)
        );
    }

    #[test]
    fn refuses_a_value_that_contains_a_path_separator() {
        assert_eq!(
            article_url("rust/lang"),
            unaddressable("article", PathSegmentRejection::Separator)
        );
    }

    #[test]
    fn refuses_a_value_that_contains_a_line_break() {
        assert_eq!(
            article_url("a\r\nX-Injected: 1"),
            unaddressable("article", PathSegmentRejection::ControlCharacter)
        );
    }

    #[test]
    fn refuses_a_value_that_completes_a_dot_segment_with_its_literal_suffix() {
        assert_eq!(
            build_url(
                ORIGIN,
                &[
                    UrlSegment::Literal("/"),
                    UrlSegment::Parameter {
                        parameter: UrlParameter {
                            name: "name",
                            value: ".".to_string(),
                        },
                        prefix: "",
                        suffix: ".",
                    },
                ],
            ),
            unaddressable("name", PathSegmentRejection::DotSegment)
        );
    }

    #[test]
    fn encodes_a_value_together_with_the_literal_text_of_its_segment() {
        assert_eq!(
            build_url(
                ORIGIN,
                &[
                    UrlSegment::Literal("/images/"),
                    UrlSegment::Parameter {
                        parameter: UrlParameter {
                            name: "id",
                            value: ".".to_string(),
                        },
                        prefix: "img",
                        suffix: " big.png",
                    },
                ],
            ),
            addressed("http://localhost/images/img.%20big.png")
        );
    }

    #[test]
    fn keeps_path_separators_inside_a_catch_all_value() {
        assert_eq!(
            file_url("images/logo .png"),
            addressed("http://localhost/files/images/logo%20.png")
        );
    }

    #[test]
    fn keeps_a_trailing_separator_of_a_catch_all_value() {
        assert_eq!(
            file_url("images/"),
            addressed("http://localhost/files/images/")
        );
    }

    #[test]
    fn joins_the_literal_text_preceding_a_catch_all_with_its_value() {
        assert_eq!(
            build_url(ORIGIN, &[UrlSegment::Literal("/"), catch_all("x", "/a b")]),
            addressed("http://localhost/x/a%20b")
        );
    }

    #[test]
    fn refuses_an_empty_catch_all_value() {
        assert_eq!(
            file_url(""),
            unaddressable("rest", PathSegmentRejection::Empty)
        );
    }

    #[test]
    fn refuses_a_catch_all_value_with_an_empty_component() {
        assert_eq!(
            file_url("a//b"),
            unaddressable("rest", PathSegmentRejection::Empty)
        );
    }

    #[test]
    fn refuses_a_catch_all_value_with_a_dot_component() {
        assert_eq!(
            file_url("nested/../secret"),
            unaddressable("rest", PathSegmentRejection::DotSegment)
        );
    }
}
