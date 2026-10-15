use margaret_url_path::constants::PATH_SEPARATOR;
use margaret_url_path::encode_url_path_segment::encode_url_path_segment;
use margaret_url_path::path_segment::PathSegment;
use margaret_url_path::path_segments::path_segments;

use crate::route_addressing::RouteAddressing;
use crate::route_position::RoutePosition;
use crate::url_segment::UrlSegment;

fn encoded_tail(decoded: &str) -> String {
    let mut encoded = String::with_capacity(decoded.len());

    for (position, segment) in path_segments(decoded).enumerate() {
        if position > 0 {
            encoded.push(PATH_SEPARATOR);
        }

        if let PathSegment::Text(component) = segment {
            encoded.push_str(&encode_url_path_segment(component));
        }
    }

    encoded
}

pub(crate) fn build_url(
    origin: &str,
    matcher: &matchit::Router<RoutePosition>,
    position: RoutePosition,
    segments: &[UrlSegment],
) -> RouteAddressing<String> {
    let mut url = String::from(origin);
    let mut path = String::new();

    for segment in segments {
        match segment {
            UrlSegment::CatchAllParameter { prefix, value, .. } => {
                let decoded = [prefix, value.as_str()].concat();

                url.push_str(&encoded_tail(&decoded));
                path.push_str(&decoded);
            }
            UrlSegment::Literal { decoded, encoded } => {
                path.push_str(decoded);
                url.push_str(encoded);
            }
            UrlSegment::Parameter { prefix, value, .. } => {
                let decoded = [prefix, value.as_str()].concat();

                url.push_str(&encode_url_path_segment(&decoded));
                path.push_str(&decoded);
            }
        }
    }

    if matcher
        .at(&path)
        .is_ok_and(|matched| *matched.value == position)
    {
        RouteAddressing::Addressed(url)
    } else {
        RouteAddressing::Misrouted { url }
    }
}

#[cfg(test)]
mod tests {
    use margaret_url_path::path_parameter_value::PathParameterValue;
    use margaret_url_path::path_tail_value::PathTailValue;
    use margaret_url_path::segment_prefix::SegmentPrefix;
    use margaret_url_path::unprefixed_segment::UnprefixedSegment;

    use super::build_url;
    use crate::route_addressing::RouteAddressing;
    use crate::route_position::RoutePosition;
    use crate::url_segment::UrlSegment;

    const ORIGIN: &str = "http://localhost";
    const GREETING: RoutePosition = RoutePosition::new(0);
    const ARTICLE_EDIT: RoutePosition = RoutePosition::new(1);
    const ARTICLE_COMMENT: RoutePosition = RoutePosition::new(2);
    const IMAGE: RoutePosition = RoutePosition::new(4);
    const FILE: RoutePosition = RoutePosition::new(5);
    const PREFIXED_FILE: RoutePosition = RoutePosition::new(6);
    const ESCAPED_CALLBACK: RoutePosition = RoutePosition::new(7);
    const ARTICLE: RoutePosition = RoutePosition::new(8);

    struct ImagePrefix;

    impl SegmentPrefix for ImagePrefix {
        const TEXT: &'static str = "img.";
    }

    struct LetterPrefix;

    impl SegmentPrefix for LetterPrefix {
        const TEXT: &'static str = "x";
    }

    fn matcher() -> matchit::Router<RoutePosition> {
        let mut matcher = matchit::Router::new();

        for (index, path) in [
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
        ]
        .into_iter()
        .enumerate()
        {
            matcher
                .insert(path, RoutePosition::new(index))
                .expect("the fixture paths build");
        }

        matcher
    }

    fn literal(text: &'static str) -> UrlSegment {
        UrlSegment::Literal {
            decoded: text,
            encoded: text,
        }
    }

    fn parameter<TPrefix: SegmentPrefix>(name: &'static str, value: &str) -> UrlSegment {
        UrlSegment::parameter(
            name,
            PathParameterValue::<TPrefix>::new(value.to_string())
                .expect("the fixture value fills its segment"),
        )
    }

    fn catch_all<TPrefix: SegmentPrefix>(value: &str) -> UrlSegment {
        UrlSegment::catch_all(
            "rest",
            PathTailValue::<TPrefix>::new(value.to_string())
                .expect("the fixture tail fills its segments"),
        )
    }

    fn url(position: RoutePosition, segments: &[UrlSegment]) -> RouteAddressing<String> {
        build_url(ORIGIN, &matcher(), position, segments)
    }

    fn article_edit_url(value: &str) -> RouteAddressing<String> {
        url(
            ARTICLE_EDIT,
            &[
                literal("/articles/"),
                parameter::<UnprefixedSegment>("article", value),
                literal("/edit"),
            ],
        )
    }

    fn file_url(value: &str) -> RouteAddressing<String> {
        url(
            FILE,
            &[literal("/files/"), catch_all::<UnprefixedSegment>(value)],
        )
    }

    fn addressed(url: &str) -> RouteAddressing<String> {
        RouteAddressing::Addressed(url.to_string())
    }

    fn misrouted(url: &str) -> RouteAddressing<String> {
        RouteAddressing::Misrouted {
            url: url.to_string(),
        }
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
                    parameter::<UnprefixedSegment>("article", "rust"),
                    literal("/comments/"),
                    parameter::<UnprefixedSegment>("comment", "42"),
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
    fn encodes_a_value_together_with_the_literal_text_of_its_segment() {
        assert_eq!(
            url(
                IMAGE,
                &[
                    literal("/images/"),
                    parameter::<ImagePrefix>("id", " big.png")
                ]
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
            url(
                PREFIXED_FILE,
                &[literal("/"), catch_all::<LetterPrefix>("/a b")]
            ),
            addressed("http://localhost/x/a%20b")
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
                    parameter::<UnprefixedSegment>("id", "7"),
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
                &[
                    literal("/articles/"),
                    parameter::<UnprefixedSegment>("article", "new")
                ]
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
