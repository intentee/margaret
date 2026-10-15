use margaret_http::route_props::RouteProps;
use margaret_http::url_segment::UrlSegment;
use margaret_url_path::path_parameter_value::PathParameterValue;
use margaret_url_path::unprefixed_segment::UnprefixedSegment;

pub struct ArticleRouteProps {
    pub article: PathParameterValue<UnprefixedSegment>,
}

impl RouteProps for ArticleRouteProps {
    const NAME: &'static str = "article";

    type Segments = [UrlSegment; 2];

    fn into_segments(self) -> Self::Segments {
        [
            UrlSegment::Literal {
                decoded: "/articles/",
                encoded: "/articles/",
            },
            UrlSegment::parameter("article", self.article),
        ]
    }
}
