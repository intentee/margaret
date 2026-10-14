use margaret_http::route_props::RouteProps;
use margaret_http::url_segment::UrlSegment;

pub struct TargetRouteProps;

impl RouteProps for TargetRouteProps {
    const NAME: &'static str = "target";

    type Segments = [UrlSegment; 1];

    fn into_segments(self) -> Self::Segments {
        [UrlSegment::Literal {
            decoded: "/target",
            encoded: "/target",
        }]
    }
}
