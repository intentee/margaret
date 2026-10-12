use margaret_url_path::admit_path_segment::admit_path_segment;
use margaret_url_path::encode_url_path_segment::encode_url_path_segment;
use margaret_url_path::path_segment::PathSegment;
use margaret_url_path::path_segment_admission::PathSegmentAdmission;
use margaret_url_path::path_segment_rejection::PathSegmentRejection;
use margaret_url_path::path_segments::path_segments;
use margaret_url_path::path_separator::PATH_SEPARATOR;

use crate::route_addressing::RouteAddressing;
use crate::route_position::RoutePosition;
use crate::server_paths::ServerPaths;
use crate::unaddressable_parameter::UnaddressableParameter;
use crate::url_parameter::UrlParameter;
use crate::url_segment::UrlSegment;

fn unaddressable(name: &'static str, rejection: PathSegmentRejection) -> SegmentAddressing {
    SegmentAddressing::Unaddressable(UnaddressableParameter { name, rejection })
}

fn addressed_catch_all(
    prefix: &str,
    UrlParameter { name, value }: &UrlParameter,
) -> SegmentAddressing {
    if value.is_empty() {
        return unaddressable(name, PathSegmentRejection::Empty);
    }

    let decoded = [prefix, value].concat();
    let mut encoded = String::with_capacity(decoded.len());

    for (position, segment) in path_segments(&decoded).enumerate() {
        if position > 0 {
            encoded.push(PATH_SEPARATOR);
        }

        if let PathSegment::Text(component) = segment {
            if let PathSegmentAdmission::Rejected(rejection) = admit_path_segment(component) {
                return unaddressable(name, rejection);
            }

            encoded.push_str(&encode_url_path_segment(component));
        }
    }

    SegmentAddressing::Addressed(AddressedSegment { decoded, encoded })
}

fn addressed_parameter(
    prefix: &str,
    UrlParameter { name, value }: &UrlParameter,
) -> SegmentAddressing {
    if value.is_empty() {
        return unaddressable(name, PathSegmentRejection::Empty);
    }

    let decoded = [prefix, value].concat();

    match admit_path_segment(&decoded) {
        PathSegmentAdmission::Admitted => SegmentAddressing::Addressed(AddressedSegment {
            encoded: encode_url_path_segment(&decoded),
            decoded,
        }),
        PathSegmentAdmission::Rejected(rejection) => unaddressable(name, rejection),
    }
}

struct AddressedSegment {
    decoded: String,
    encoded: String,
}

enum SegmentAddressing {
    Addressed(AddressedSegment),
    Unaddressable(UnaddressableParameter),
}

#[must_use]
pub fn build_url<const COUNT: usize>(
    origin: &str,
    paths: &ServerPaths<COUNT>,
    position: RoutePosition,
    segments: &[UrlSegment],
) -> RouteAddressing<String> {
    let mut url = String::from(origin);
    let mut path = String::new();

    for segment in segments {
        let addressing = match segment {
            UrlSegment::CatchAllParameter { parameter, prefix } => {
                addressed_catch_all(prefix, parameter)
            }
            UrlSegment::Literal { decoded, encoded } => {
                path.push_str(decoded);
                url.push_str(encoded);

                continue;
            }
            UrlSegment::Parameter { parameter, prefix } => addressed_parameter(prefix, parameter),
        };

        match addressing {
            SegmentAddressing::Addressed(AddressedSegment { decoded, encoded }) => {
                path.push_str(&decoded);
                url.push_str(&encoded);
            }
            SegmentAddressing::Unaddressable(parameter) => {
                return RouteAddressing::Unaddressable(parameter);
            }
        }
    }

    if paths.resolves_to(&path, position) {
        RouteAddressing::Addressed(url)
    } else {
        RouteAddressing::Misrouted { url }
    }
}

#[cfg(test)]
mod tests {
    use margaret_url_path::path_segment_rejection::PathSegmentRejection;

    use super::build_url;
    use crate::route_addressing::RouteAddressing;
    use crate::route_position::RoutePosition;
    use crate::server_paths::ServerPaths;
    use crate::unaddressable_parameter::UnaddressableParameter;
    use crate::url_parameter::UrlParameter;
    use crate::url_segment::UrlSegment;

    const ORIGIN: &str = "http://localhost";
    const GREETING: RoutePosition = RoutePosition::new(0);
    const ARTICLE_EDIT: RoutePosition = RoutePosition::new(1);
    const ARTICLE_COMMENT: RoutePosition = RoutePosition::new(2);
    const DOTTED_NAME: RoutePosition = RoutePosition::new(3);
    const IMAGE: RoutePosition = RoutePosition::new(4);
    const FILE: RoutePosition = RoutePosition::new(5);
    const PREFIXED_FILE: RoutePosition = RoutePosition::new(6);
    const ESCAPED_CALLBACK: RoutePosition = RoutePosition::new(7);
    const ARTICLE: RoutePosition = RoutePosition::new(8);

    fn paths() -> ServerPaths<11> {
        ServerPaths::build([
            "/greeting",
            "/articles/{article}/edit",
            "/articles/{article}/comments/{comment}",
            "/.{name}",
            "/images/img.{id}",
            "/files/{*rest}",
            "/x{*rest}",
            "/oauth/{{callback}}/{id}",
            "/articles/{article}",
            "/articles/new",
            "/files/readme",
        ])
        .expect("the fixture paths build")
    }

    fn literal(text: &'static str) -> UrlSegment {
        UrlSegment::Literal {
            decoded: text,
            encoded: text,
        }
    }

    fn parameter(name: &'static str, prefix: &'static str, value: &str) -> UrlSegment {
        UrlSegment::Parameter {
            parameter: UrlParameter {
                name,
                value: value.to_string(),
            },
            prefix,
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

    fn url(position: RoutePosition, segments: &[UrlSegment]) -> RouteAddressing<String> {
        build_url(ORIGIN, &paths(), position, segments)
    }

    fn article_edit_url(value: &str) -> RouteAddressing<String> {
        url(
            ARTICLE_EDIT,
            &[
                literal("/articles/"),
                parameter("article", "", value),
                literal("/edit"),
            ],
        )
    }

    fn file_url(value: &str) -> RouteAddressing<String> {
        url(FILE, &[literal("/files/"), catch_all("", value)])
    }

    fn addressed(url: &str) -> RouteAddressing<String> {
        RouteAddressing::Addressed(url.to_string())
    }

    fn misrouted(url: &str) -> RouteAddressing<String> {
        RouteAddressing::Misrouted {
            url: url.to_string(),
        }
    }

    fn unaddressable(
        name: &'static str,
        rejection: PathSegmentRejection,
    ) -> RouteAddressing<String> {
        RouteAddressing::Unaddressable(UnaddressableParameter { name, rejection })
    }

    #[test]
    fn joins_literal_segments_onto_the_origin_so_static_routes_get_absolute_urls() {
        assert_eq!(
            url(GREETING, &[literal("/greeting")]),
            addressed("http://localhost/greeting")
        );
    }

    #[test]
    fn percent_encodes_a_parameter_value_that_contains_url_delimiters() {
        assert_eq!(
            article_edit_url("a b#c?d%e"),
            addressed("http://localhost/articles/a%20b%23c%3Fd%25e/edit")
        );
    }

    #[test]
    fn substitutes_each_parameter_in_segment_order_so_values_land_in_their_own_slot() {
        assert_eq!(
            url(
                ARTICLE_COMMENT,
                &[
                    literal("/articles/"),
                    parameter("article", "", "rust"),
                    literal("/comments/"),
                    parameter("comment", "", "42"),
                ],
            ),
            addressed("http://localhost/articles/rust/comments/42")
        );
    }

    #[test]
    fn keeps_a_leading_dot_of_a_value_that_is_not_a_dot_segment() {
        assert_eq!(
            article_edit_url(".env"),
            addressed("http://localhost/articles/.env/edit")
        );
    }

    #[test]
    fn refuses_a_parent_directory_value_since_clients_would_collapse_the_segment() {
        assert_eq!(
            article_edit_url(".."),
            unaddressable("article", PathSegmentRejection::DotSegment)
        );
    }

    #[test]
    fn refuses_a_current_directory_value_since_clients_would_drop_the_segment() {
        assert_eq!(
            article_edit_url("."),
            unaddressable("article", PathSegmentRejection::DotSegment)
        );
    }

    #[test]
    fn refuses_an_empty_parameter_value_since_an_empty_segment_cannot_address_the_route() {
        assert_eq!(
            article_edit_url(""),
            unaddressable("article", PathSegmentRejection::Empty)
        );
    }

    #[test]
    fn refuses_a_value_that_contains_a_path_separator() {
        assert_eq!(
            article_edit_url("rust/lang"),
            unaddressable("article", PathSegmentRejection::Separator)
        );
    }

    #[test]
    fn refuses_a_value_that_contains_a_line_break() {
        assert_eq!(
            article_edit_url("a\r\nX-Injected: 1"),
            unaddressable("article", PathSegmentRejection::ControlCharacter)
        );
    }

    #[test]
    fn refuses_a_value_that_completes_a_dot_segment_with_its_literal_prefix() {
        assert_eq!(
            url(DOTTED_NAME, &[literal("/"), parameter("name", ".", ".")]),
            unaddressable("name", PathSegmentRejection::DotSegment)
        );
    }

    #[test]
    fn encodes_a_value_together_with_the_literal_text_of_its_segment() {
        assert_eq!(
            url(
                IMAGE,
                &[literal("/images/"), parameter("id", "img.", " big.png")]
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
            url(PREFIXED_FILE, &[literal("/"), catch_all("x", "/a b")]),
            addressed("http://localhost/x/a%20b")
        );
    }

    #[test]
    fn refuses_an_empty_catch_all_value_since_an_empty_tail_cannot_address_the_route() {
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

    #[test]
    fn addresses_an_encoded_literal_route_by_resolving_the_path_the_server_decodes() {
        assert_eq!(
            url(
                ESCAPED_CALLBACK,
                &[
                    UrlSegment::Literal {
                        decoded: "/oauth/{callback}/",
                        encoded: "/oauth/%7Bcallback%7D/",
                    },
                    parameter("id", "", "7"),
                ],
            ),
            addressed("http://localhost/oauth/%7Bcallback%7D/7")
        );
    }

    #[test]
    fn misroutes_a_value_a_static_route_shadows_so_no_link_reaches_another_handler() {
        assert_eq!(
            url(
                ARTICLE,
                &[literal("/articles/"), parameter("article", "", "new")]
            ),
            misrouted("http://localhost/articles/new")
        );
    }

    #[test]
    fn misroutes_a_shadowed_catch_all_value_so_no_link_reaches_another_handler() {
        assert_eq!(
            file_url("readme"),
            misrouted("http://localhost/files/readme")
        );
    }
}
