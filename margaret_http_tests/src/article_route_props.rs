use margaret_http::route_props::RouteProps;
use margaret_http::url_parameter::UrlParameter;
use margaret_http::url_segment::UrlSegment;

pub struct ArticleRouteProps {
    pub article: String,
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
            UrlSegment::Parameter {
                parameter: UrlParameter {
                    name: "article",
                    value: self.article,
                },
                prefix: "",
            },
        ]
    }
}
