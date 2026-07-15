use crate::route_url_template::route_url_template;
use crate::url_segment::UrlSegment;

pub(crate) struct RoutePath {
    pattern: String,
    segments: Vec<UrlSegment>,
}

impl RoutePath {
    pub(crate) fn parse(pattern: &str) -> Self {
        Self {
            segments: route_url_template(pattern),
            pattern: pattern.to_owned(),
        }
    }

    pub(crate) fn parameters(&self) -> impl Iterator<Item = &str> {
        self.segments.iter().filter_map(|segment| match segment {
            UrlSegment::Literal(_) => None,
            UrlSegment::Parameter(name) => Some(name.as_str()),
        })
    }

    pub(crate) fn pattern(&self) -> &str {
        &self.pattern
    }

    pub(crate) fn segments(&self) -> &[UrlSegment] {
        &self.segments
    }
}
